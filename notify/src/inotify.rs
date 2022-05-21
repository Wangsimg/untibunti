
//! Watcher implementation for the inotify Linux API
//!
//! The inotify API provides a mechanism for monitoring filesystem events.  Inotify can be used to
//! monitor individual files, or to monitor directories.  When a directory is monitored, inotify
//! will return events for the directory itself, and for files inside the directory.

use super::event::*;
use super::{Config, Error, ErrorKind, EventHandler, RecursiveMode, Result, Watcher};
use crate::{bounded, unbounded, BoundSender, Receiver, Sender};
use inotify as inotify_sys;
use inotify_sys::{EventMask, Inotify, WatchDescriptor, WatchMask};
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::metadata;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;

const INOTIFY: mio::Token = mio::Token(0);
const MESSAGE: mio::Token = mio::Token(1);

// The EventLoop will set up a mio::Poll and use it to wait for the following:
//
// -  messages telling it what to do
//
// -  events telling it that something has happened on one of the watched files.
struct EventLoop {
    running: bool,
    poll: mio::Poll,
    event_loop_waker: Arc<mio::Waker>,
    event_loop_tx: Sender<EventLoopMsg>,
    event_loop_rx: Receiver<EventLoopMsg>,
    inotify: Option<Inotify>,
    event_handler: Box<dyn EventHandler>,
    watches: HashMap<PathBuf, (WatchDescriptor, WatchMask, bool)>,
    paths: HashMap<WatchDescriptor, PathBuf>,
    rename_event: Option<Event>,
}

/// Watcher implementation based on inotify
#[derive(Debug)]
pub struct INotifyWatcher {
    channel: Sender<EventLoopMsg>,
    waker: Arc<mio::Waker>,
}

enum EventLoopMsg {
    AddWatch(PathBuf, RecursiveMode, Sender<Result<()>>),
    RemoveWatch(PathBuf, Sender<Result<()>>),
    Shutdown,
    RenameTimeout(usize),
    Configure(Config, BoundSender<Result<bool>>),
}

#[inline]
fn send_pending_rename_event(
    rename_event: &mut Option<Event>,
    event_handler: &mut dyn EventHandler,
) {
    if let Some(e) = rename_event.take() {
        event_handler.handle_event(Ok(e));
    }
}

#[inline]
fn add_watch_by_event(
    path: &Option<PathBuf>,
    event: &inotify_sys::Event<&OsStr>,
    watches: &HashMap<PathBuf, (WatchDescriptor, WatchMask, bool)>,
    add_watches: &mut Vec<PathBuf>,
) {
    if let Some(ref path) = *path {
        if event.mask.contains(EventMask::ISDIR) {
            if let Some(parent_path) = path.parent() {
                if let Some(&(_, _, is_recursive)) = watches.get(parent_path) {
                    if is_recursive {
                        add_watches.push(path.to_owned());
                    }
                }
            }
        }
    }
}

#[inline]
fn remove_watch_by_event(
    path: &Option<PathBuf>,
    watches: &HashMap<PathBuf, (WatchDescriptor, WatchMask, bool)>,
    remove_watches: &mut Vec<PathBuf>,
) {
    if let Some(ref path) = *path {
        if watches.contains_key(path) {
            remove_watches.push(path.to_owned());
        }
    }
}

impl EventLoop {
    pub fn new(inotify: Inotify, event_handler: Box<dyn EventHandler>) -> Result<Self> {
        let (event_loop_tx, event_loop_rx) = unbounded::<EventLoopMsg>();
        let poll = mio::Poll::new()?;

        let event_loop_waker = Arc::new(mio::Waker::new(poll.registry(), MESSAGE)?);

        let inotify_fd = inotify.as_raw_fd();
        let mut evented_inotify = mio::unix::SourceFd(&inotify_fd);
        poll.registry()
            .register(&mut evented_inotify, INOTIFY, mio::Interest::READABLE)?;

        let event_loop = EventLoop {
            running: true,
            poll,
            event_loop_waker,
            event_loop_tx,
            event_loop_rx,
            inotify: Some(inotify),
            event_handler,
            watches: HashMap::new(),
            paths: HashMap::new(),
            rename_event: None,
        };
        Ok(event_loop)
    }

    // Run the event loop.
    pub fn run(self) {
        let _ = thread::Builder::new()
            .name("notify-rs inotify loop".to_string())
            .spawn(|| self.event_loop_thread());
    }

    fn event_loop_thread(mut self) {
        let mut events = mio::Events::with_capacity(16);
        loop {
            // Wait for something to happen.
            match self.poll.poll(&mut events, None) {
                Err(ref e) if matches!(e.kind(), std::io::ErrorKind::Interrupted) => {
                    // System call was interrupted, we will retry
                    // TODO: Not covered by tests (to reproduce likely need to setup signal handlers)
                }
                Err(e) => panic!("poll failed: {}", e),
                Ok(()) => {}
            }

            // Process whatever happened.
            for event in &events {
                self.handle_event(event);
            }

            // Stop, if we're done.
            if !self.running {
                break;
            }
        }
    }

    // Handle a single event.
    fn handle_event(&mut self, event: &mio::event::Event) {
        match event.token() {
            MESSAGE => {
                // The channel is readable - handle messages.
                self.handle_messages()
            }
            INOTIFY => {
                // inotify has something to tell us.
                self.handle_inotify()
            }
            _ => unreachable!(),
        }
    }

    fn handle_messages(&mut self) {
        while let Ok(msg) = self.event_loop_rx.try_recv() {
            match msg {
                EventLoopMsg::AddWatch(path, recursive_mode, tx) => {
                    let _ = tx.send(self.add_watch(path, recursive_mode.is_recursive(), true));
                }
                EventLoopMsg::RemoveWatch(path, tx) => {
                    let _ = tx.send(self.remove_watch(path, false));
                }
                EventLoopMsg::Shutdown => {
                    let _ = self.remove_all_watches();
                    if let Some(inotify) = self.inotify.take() {
                        let _ = inotify.close();
                    }
                    self.running = false;
                    break;
                }
                EventLoopMsg::RenameTimeout(cookie) => {
                    let current_cookie = self.rename_event.as_ref().and_then(|e| e.tracker());
                    // send pending rename event only if the rename event for which the timer has been created hasn't been handled already; otherwise ignore this timeout
                    if current_cookie == Some(cookie) {
                        send_pending_rename_event(&mut self.rename_event, &mut *self.event_handler);
                    }
                }
                EventLoopMsg::Configure(config, tx) => {
                    self.configure_raw_mode(config, tx);
                }
            }
        }
    }

    fn configure_raw_mode(&mut self, _config: Config, tx: BoundSender<Result<bool>>) {
        tx.send(Ok(false))
            .expect("configuration channel disconnected");
    }

    fn handle_inotify(&mut self) {
        let mut add_watches = Vec::new();
        let mut remove_watches = Vec::new();

        if let Some(ref mut inotify) = self.inotify {
            let mut buffer = [0; 1024];
            // Read all buffers available.
            loop {
                match inotify.read_events(&mut buffer) {
                    Ok(events) => {
                        let mut num_events = 0;
                        for event in events {
                            num_events += 1;
                            if event.mask.contains(EventMask::Q_OVERFLOW) {
                                let ev = Ok(Event::new(EventKind::Other).set_flag(Flag::Rescan));
                                self.event_handler.handle_event(ev);
                            }

                            let path = match event.name {
                                Some(name) => {
                                    self.paths.get(&event.wd).map(|root| root.join(&name))
                                }
                                None => self.paths.get(&event.wd).cloned(),
                            };

                            if event.mask.contains(EventMask::MOVED_FROM) {
                                send_pending_rename_event(
                                    &mut self.rename_event,
                                    &mut *self.event_handler,
                                );
                                remove_watch_by_event(&path, &self.watches, &mut remove_watches);
                                self.rename_event = Some(
                                    Event::new(EventKind::Modify(ModifyKind::Name(
                                        RenameMode::From,
                                    )))
                                    .add_some_path(path.clone())
                                    .set_tracker(event.cookie as usize),
                                );
                            } else {
                                let mut evs = Vec::new();
                                if event.mask.contains(EventMask::MOVED_TO) {
                                    if let Some(e) = self.rename_event.take() {
                                        if e.tracker() == Some(event.cookie as usize) {
                                            self.event_handler.handle_event(Ok(e.clone()));
                                            evs.push(
                                                Event::new(EventKind::Modify(ModifyKind::Name(
                                                    RenameMode::To,
                                                )))
                                                .set_tracker(event.cookie as usize)
                                                .add_some_path(path.clone()),
                                            );
                                            evs.push(
                                                Event::new(EventKind::Modify(ModifyKind::Name(
                                                    RenameMode::Both,
                                                )))
                                                .set_tracker(event.cookie as usize)
                                                .add_some_path(e.paths.first().cloned())
                                                .add_some_path(path.clone()),
                                            );
                                        } else {
                                            // TODO should it be rename?
                                            evs.push(
                                                Event::new(EventKind::Create(
                                                    if event.mask.contains(EventMask::ISDIR) {
                                                        CreateKind::Folder
                                                    } else {
                                                        CreateKind::File
                                                    },
                                                ))
                                                .add_some_path(path.clone()),
                                            );
                                        }
                                    } else {
                                        // TODO should it be rename?
                                        evs.push(
                                            Event::new(EventKind::Create(
                                                if event.mask.contains(EventMask::ISDIR) {
                                                    CreateKind::Folder
                                                } else {
                                                    CreateKind::File