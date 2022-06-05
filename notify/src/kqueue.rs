//! Watcher implementation for the kqueue API
//!
//! The kqueue() system call provides a generic method of notifying the user
//! when an event happens or a condition holds, based on the results of small
//! pieces of kernel code termed filters.

use super::event::*;
use super::{Config, Error, EventHandler, RecursiveMode, Result, Watcher};
use crate::{unbounded, Receiver, Sender};
use kqueue::{EventData, EventFilter, FilterFlag, Ident};
use std::collections::HashMap;
use std::env;
use std::fs::metadata;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use walkdir::WalkDir;

const KQUEUE: mio::Token = mio::Token(0);
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
    kqueue: kqueue::Watcher,
    event_handler: Box<dyn EventHandler>,
    watches: HashMap<PathBuf, bool>,
}

/// Watcher implementation based on inotify
#[derive(Debug)]
pub struct KqueueWatcher {
    channel: Sender<EventLoopMsg>,
    waker: Arc<mio::Waker>,
}

enum EventLoopMsg {
    AddWatch(PathBuf, RecursiveMode, Sender<Result<()>>),
    RemoveWatch(PathBuf, Sender<Result<()>>),
    Shutdown,
}

impl EventLoop {
    pub fn new(kqueue: kqueue::Watcher, event_handler: Box<dyn EventHandler>) -> Result<Self> {
        let (event_loop_tx, event_loop_rx) = unbounded::<EventLoopMsg>();
        let poll = mio::Poll::new()?;

        let event_loop_waker = Arc::new(mio::Waker::new(poll.registry(), MESSAGE)?);

        let kqueue_fd = kqueue.as_raw_fd();
        let mut evented_kqueue = mio::unix::SourceFd(&kqueue_fd);
        poll.registry()
            .register(&mut evented_kqueue, KQUEUE, mio::Interest::READABLE)?;

        let event_loop = EventLoop {
            running: true,
            poll,
            event_loop_waker,
            event_loop_tx,
            event_loop_rx,
            kqueue,
            event_handler,
            watches: HashMap::new(),
        };
        Ok(event_loop)
    }

    // Run the event loop.
    pub fn run(self) {
        let _ = thread::Builder::new()
            .name("notify-rs kqueue loop".to_string())
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
              