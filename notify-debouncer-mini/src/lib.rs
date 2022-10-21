//! Debouncer for notify
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! notify-debouncer-mini = "0.2.0"
//! ```
//! In case you want to select specific features of notify,
//! specify notify as dependency explicitely in your dependencies.
//! Otherwise you can just use the re-export of notify from debouncer-mini.
//! ```toml
//! notify-debouncer-mini = "0.2.0"
//! notify = { version = "..", features = [".."] }
//! ```
//!  
//! # Examples
//!
//! ```rust,no_run
//! # use std::path::Path;
//! # use std::time::Duration;
//! use notify_debouncer_mini::{notify::*,new_debouncer,DebounceEventResult};
//!
//! # fn main() {
//!     // setup initial watcher backend config
//!     let config = Config::default();
//! 
//!     // Select recommended watcher for debouncer.
//!     // Using a callback here, could also be a channel.
//!     let mut debouncer = new_debouncer(Duration::from_secs(2), None, |res: DebounceEventResult| {
//!         match res {
//!             Ok(events) => events.iter().for_each(|e|println!("Event {:?} for {:?}",e.kind,e.path)),
//!             Err(errors) => errors.iter().for_each(|e|println!("Error {:?}",e)),
//!         }
//!     }).unwrap();
//!
//!     // Add a path to be watched. All files and directories at that path and
//!     // below will be monitored for changes.
//!     debouncer.watcher().watch(Path::new("."), RecursiveMode::Recursive).unwrap();
//! # }
//! ```
//!
//! # Features
//!
//! The following crate features can be turned on or off in your cargo dependency config:
//!
//! - `crossbeam` enabled by default, adds [`DebounceEventHandler`](DebounceEventHandler) support for crossbeam channels.
//!   Also enables crossbeam-channel in the re-exported notify. You may want to disable this when using the tokio async runtime.
//! - `serde` enables serde support for events.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

pub use notify;
use notify::{Error, ErrorKind, Event, RecommendedWatcher, Watcher};

/// The set of requirements for watcher debounce event handling functions.
///
/// # Example implementation
///
/// ```rust,no_run
/// # use notify::{Event, Result, EventHandler};
/// # use notify_debouncer_mini::{DebounceEventHandler,DebounceEventResult};
///
/// /// Prints received events
/// struct EventPrinter;
///
/// impl DebounceEventHandler for EventPrinter {
///     fn handle_event(&mut self, event: DebounceEventResult) {
///         match event {
///             Ok(events) => {
///                 for event in events {
///                     println!("Event {:?} for path {:?}",event.kind,event.path);
///                 }
///             },
///             // errors are batched, so you get either events or errors, probably both per debounce tick (two calls)
///             Err(errors) => errors.iter().for_each(|e|println!("Got error {:?}",e)),
///         }
///     }
//