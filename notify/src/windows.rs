
#![allow(missing_docs)]
//! Watcher implementation for Windows' directory management APIs
//!
//! For more information see the [ReadDirectoryChangesW reference][ref].
//!
//! [ref]: https://msdn.microsoft.com/en-us/library/windows/desktop/aa363950(v=vs.85).aspx

use crate::{bounded, unbounded, BoundSender, Config, Receiver, Sender};
use crate::{event::*, WatcherKind};
use crate::{Error, EventHandler, RecursiveMode, Result, Watcher};
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::mem;
use std::os::raw::c_void;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::ptr;
use std::slice;
use std::sync::{Arc, Mutex};
use std::thread;
use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_OPERATION_ABORTED, HANDLE, INVALID_HANDLE_VALUE, WAIT_OBJECT_0,
};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, ReadDirectoryChangesW, FILE_ACTION_ADDED, FILE_ACTION_MODIFIED,
    FILE_ACTION_REMOVED, FILE_ACTION_RENAMED_NEW_NAME, FILE_ACTION_RENAMED_OLD_NAME,
    FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OVERLAPPED, FILE_LIST_DIRECTORY,
    FILE_NOTIFY_CHANGE_ATTRIBUTES, FILE_NOTIFY_CHANGE_CREATION, FILE_NOTIFY_CHANGE_DIR_NAME,
    FILE_NOTIFY_CHANGE_FILE_NAME, FILE_NOTIFY_CHANGE_LAST_WRITE, FILE_NOTIFY_CHANGE_SECURITY,
    FILE_NOTIFY_CHANGE_SIZE, FILE_NOTIFY_INFORMATION, FILE_SHARE_DELETE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows_sys::Win32::System::Threading::{
    CreateSemaphoreW, ReleaseSemaphore, WaitForSingleObjectEx,
};
use windows_sys::Win32::System::WindowsProgramming::INFINITE;
use windows_sys::Win32::System::IO::{CancelIo, OVERLAPPED};

const BUF_SIZE: u32 = 16384;

#[derive(Clone)]
struct ReadData {
    dir: PathBuf,          // directory that is being watched
    file: Option<PathBuf>, // if a file is being watched, this is its full path
    complete_sem: HANDLE,
    is_recursive: bool,
}

struct ReadDirectoryRequest {
    event_handler: Arc<Mutex<dyn EventHandler>>,
    buffer: [u8; BUF_SIZE as usize],
    handle: HANDLE,
    data: ReadData,
}

enum Action {
    Watch(PathBuf, RecursiveMode),
    Unwatch(PathBuf),
    Stop,
    Configure(Config, BoundSender<Result<bool>>),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MetaEvent {
    SingleWatchComplete,
    WatcherAwakened,
}

struct WatchState {
    dir_handle: HANDLE,
    complete_sem: HANDLE,
}

struct ReadDirectoryChangesServer {
    rx: Receiver<Action>,
    event_handler: Arc<Mutex<dyn EventHandler>>,
    meta_tx: Sender<MetaEvent>,
    cmd_tx: Sender<Result<PathBuf>>,
    watches: HashMap<PathBuf, WatchState>,
    wakeup_sem: HANDLE,
}

impl ReadDirectoryChangesServer {
    fn start(
        event_handler: Arc<Mutex<dyn EventHandler>>,
        meta_tx: Sender<MetaEvent>,
        cmd_tx: Sender<Result<PathBuf>>,
        wakeup_sem: HANDLE,
    ) -> Sender<Action> {
        let (action_tx, action_rx) = unbounded();
        // it is, in fact, ok to send the semaphore across threads
        let sem_temp = wakeup_sem as u64;
        let _ = thread::Builder::new()
            .name("notify-rs windows loop".to_string())
            .spawn(move || {
                let wakeup_sem = sem_temp as HANDLE;
                let server = ReadDirectoryChangesServer {
                    rx: action_rx,
                    event_handler,
                    meta_tx,
                    cmd_tx,
                    watches: HashMap::new(),
                    wakeup_sem,
                };
                server.run();
            });
        action_tx
    }

    fn run(mut self) {
        loop {
            // process all available actions first
            let mut stopped = false;

            while let Ok(action) = self.rx.try_recv() {
                match action {
                    Action::Watch(path, recursive_mode) => {
                        let res = self.add_watch(path, recursive_mode.is_recursive());
                        let _ = self.cmd_tx.send(res);
                    }
                    Action::Unwatch(path) => self.remove_watch(path),
                    Action::Stop => {
                        stopped = true;
                        for ws in self.watches.values() {
                            stop_watch(ws, &self.meta_tx);
                        }
                        break;
                    }
                    Action::Configure(config, tx) => {
                        self.configure_raw_mode(config, tx);
                    }
                }
            }

            if stopped {
                break;
            }

            unsafe {
                // wait with alertable flag so that the completion routine fires
                let waitres = WaitForSingleObjectEx(self.wakeup_sem, 100, 1);
                if waitres == WAIT_OBJECT_0 {
                    let _ = self.meta_tx.send(MetaEvent::WatcherAwakened);
                }
            }
        }

        // we have to clean this up, since the watcher may be long gone
        unsafe {
            CloseHandle(self.wakeup_sem);
        }
    }

    fn add_watch(&mut self, path: PathBuf, is_recursive: bool) -> Result<PathBuf> {
        // path must exist and be either a file or directory
        if !path.is_dir() && !path.is_file() {
            return Err(
                Error::generic("Input watch path is neither a file nor a directory.")
                    .add_path(path),
            );
        }

        let (watching_file, dir_target) = {
            if path.is_dir() {
                (false, path.clone())
            } else {
                // emulate file watching by watching the parent directory
                (true, path.parent().unwrap().to_path_buf())
            }
        };

        let encoded_path: Vec<u16> = dir_target
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        let handle;
        unsafe {
            handle = CreateFileW(
                encoded_path.as_ptr(),
                FILE_LIST_DIRECTORY,
                FILE_SHARE_READ | FILE_SHARE_DELETE | FILE_SHARE_WRITE,
                ptr::null_mut(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OVERLAPPED,
                0,
            );

            if handle == INVALID_HANDLE_VALUE {
                return Err(if watching_file {
                    Error::generic(
                        "You attempted to watch a single file, but parent \
                         directory could not be opened.",
                    )
                    .add_path(path)
                } else {
                    // TODO: Call GetLastError for better error info?
                    Error::path_not_found().add_path(path)
                });
            }
        }
        let wf = if watching_file {
            Some(path.clone())
        } else {
            None
        };
        // every watcher gets its own semaphore to signal completion
        let semaphore = unsafe { CreateSemaphoreW(ptr::null_mut(), 0, 1, ptr::null_mut()) };
        if semaphore == 0 || semaphore == INVALID_HANDLE_VALUE {
            unsafe {
                CloseHandle(handle);
            }
            return Err(Error::generic("Failed to create semaphore for watch.").add_path(path));
        }
        let rd = ReadData {
            dir: dir_target,
            file: wf,
            complete_sem: semaphore,
            is_recursive,
        };
        let ws = WatchState {
            dir_handle: handle,
            complete_sem: semaphore,
        };
        self.watches.insert(path.clone(), ws);
        start_read(&rd, self.event_handler.clone(), handle);
        Ok(path)
    }

    fn remove_watch(&mut self, path: PathBuf) {
        if let Some(ws) = self.watches.remove(&path) {
            stop_watch(&ws, &self.meta_tx);
        }
    }

    fn configure_raw_mode(&mut self, _config: Config, tx: BoundSender<Result<bool>>) {
        tx.send(Ok(false))
            .expect("configuration channel disconnect");
    }
}

fn stop_watch(ws: &WatchState, meta_tx: &Sender<MetaEvent>) {
    unsafe {
        let cio = CancelIo(ws.dir_handle);
        let ch = CloseHandle(ws.dir_handle);
        // have to wait for it, otherwise we leak the memory allocated for there read request
        if cio != 0 && ch != 0 {
            while WaitForSingleObjectEx(ws.complete_sem, INFINITE, 1) != WAIT_OBJECT_0 {
                // drain the apc queue, fix for https://github.com/notify-rs/notify/issues/287#issuecomment-801465550
            }
        }
        CloseHandle(ws.complete_sem);
    }
    let _ = meta_tx.send(MetaEvent::SingleWatchComplete);
}

fn start_read(rd: &ReadData, event_handler: Arc<Mutex<dyn EventHandler>>, handle: HANDLE) {
    let mut request = Box::new(ReadDirectoryRequest {
        event_handler,
        handle,
        buffer: [0u8; BUF_SIZE as usize],
        data: rd.clone(),
    });

    let flags = FILE_NOTIFY_CHANGE_FILE_NAME
        | FILE_NOTIFY_CHANGE_DIR_NAME
        | FILE_NOTIFY_CHANGE_ATTRIBUTES
        | FILE_NOTIFY_CHANGE_SIZE
        | FILE_NOTIFY_CHANGE_LAST_WRITE
        | FILE_NOTIFY_CHANGE_CREATION
        | FILE_NOTIFY_CHANGE_SECURITY;

    let monitor_subdir = if (&request.data.file).is_none() && request.data.is_recursive {
        1
    } else {
        0
    };

    unsafe {
        let mut overlapped: Box<OVERLAPPED> = Box::new(mem::zeroed());
        // When using callback based async requests, we are allowed to use the hEvent member
        // for our own purposes

        let req_buf = request.buffer.as_mut_ptr() as *mut c_void;
        let request_p = Box::into_raw(request) as isize;
        overlapped.hEvent = request_p;

        // This is using an asynchronous call with a completion routine for receiving notifications
        // An I/O completion port would probably be more performant
        let ret = ReadDirectoryChangesW(
            handle,
            req_buf,
            BUF_SIZE,
            monitor_subdir,
            flags,
            &mut 0u32 as *mut u32, // not used for async reqs
            &mut *overlapped as *mut OVERLAPPED,
            Some(handle_event),
        );

        if ret == 0 {
            // error reading. retransmute request memory to allow drop.
            // allow overlapped to drop by omitting forget()
            let request: Box<ReadDirectoryRequest> = mem::transmute(request_p);

            ReleaseSemaphore(request.data.complete_sem, 1, ptr::null_mut());
        } else {
            // read ok. forget overlapped to let the completion routine handle memory
            mem::forget(overlapped);
        }
    }
}

unsafe extern "system" fn handle_event(
    error_code: u32,
    _bytes_written: u32,
    overlapped: *mut OVERLAPPED,
) {
    let overlapped: Box<OVERLAPPED> = Box::from_raw(overlapped);
    let request: Box<ReadDirectoryRequest> = Box::from_raw(overlapped.hEvent as *mut _);

    if error_code == ERROR_OPERATION_ABORTED {
        // received when dir is unwatched or watcher is shutdown; return and let overlapped/request
        // get drop-cleaned
        ReleaseSemaphore(request.data.complete_sem, 1, ptr::null_mut());
        return;
    }

    // Get the next request queued up as soon as possible