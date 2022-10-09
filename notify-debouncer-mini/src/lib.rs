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
//!     let mut debouncer = new_debouncer(Duration::from_secs(2), None, |res: DebounceEventRe