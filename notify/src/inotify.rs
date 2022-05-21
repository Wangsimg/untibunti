
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