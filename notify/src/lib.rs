
//! Cross-platform file system notification library
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! notify = "5.1.0"
//! ```
//! 
//! If you want debounced events, see [notify-debouncer-mini](https://github.com/notify-rs/notify/tree/main/notify-debouncer-mini)
//! 
//! ## Features
//! 
//! List of compilation features, see below for details
//! 
//! - `serde` for serialization of events
//! - `macos_fsevent` enabled by default, for fsevent backend on macos
//! - `macos_kqueue` for kqueue backend on macos
//! - `crossbeam-channel` enabled by default, see below
//!
//! ### Serde
//!
//! Events are serialisable via [serde](https://serde.rs) if the `serde` feature is enabled:
//!
//! ```toml
//! notify = { version = "5.1.0", features = ["serde"] }
//! ```
//! 
//! ### Crossbeam-Channel & Tokio
//! 
//! By default crossbeam-channel is used internally by notify.
//! This can [cause issues](https://github.com/notify-rs/notify/issues/380) when used inside tokio.
//! 
//! You can disable crossbeam-channel, letting notify fallback to std channels via
//! 
//! ```toml
//! notify = { version = "5.1.0", default-features = false, features = ["macos_kqueue"] }
//! // Alternatively macos_fsevent instead of macos_kqueue
//! ```
//! Note the `macos_kqueue` requirement here, otherwise no backend is available on macos.
//! 
//! # Known Problems
//! 
//! ### Docker with Linux on MacOS M1
//! 
//! Docker on macos M1 [throws](https://github.com/notify-rs/notify/issues/423) `Function not implemented (os error 38)`.
//! You have to manually use the [PollWatcher], as the native backend isn't available inside the emulation.
//! 
//! ### MacOS, FSEvents and unowned files
//! 
//! Due to the inner security model of FSEvents (see [FileSystemEventSecurity](https://developer.apple.com/library/mac/documentation/Darwin/Conceptual/FSEvents_ProgGuide/FileSystemEventSecurity/FileSystemEventSecurity.html)),
//! some events cannot be observed easily when trying to follow files that do not
//! belong to you. In this case, reverting to the pollwatcher can fix the issue,
//! with a slight performance cost.
//! 
//! ### Editor Behaviour
//! 
//! If you rely on precise events (Write/Delete/Create..), you will notice that the actual events
//! can differ a lot between file editors. Some truncate the file on save, some create a new one and replace the old one.
//! See also [this](https://github.com/notify-rs/notify/issues/247) and [this](https://github.com/notify-rs/notify/issues/113#issuecomment-281836995) issues for example.
//!
//! ### Parent folder deletion
//! 
//! If you want to receive an event for a deletion of folder `b` for the path `/a/b/..`, you will have to watch its parent `/a`.
//! See [here](https://github.com/notify-rs/notify/issues/403) for more details.
//! 
//! ### Pseudo Filesystems like /proc,/sys
//! 
//! Some filesystems like `/proc` and `/sys` on *nix do not emit change events or use correct file change dates.
//! To circumvent that problem you can use the [PollWatcher] with the `compare_contents` option.
//! 
//! ### Linux: Bad File Descriptor / No space left on device
//! 
//! This may be the case of running into the max-files watched limits of your user or system.
//! (Files also includes folders.) Note that for recursive watched folders each file and folder inside counts towards the limit.
//! 
//! You may increase this limit in linux via
//! ```sh
//! sudo sysctl fs.inotify.max_user_instances=8192 # example number
//! sudo sysctl fs.inotify.max_user_watches=524288 # example number
//! sudo sysctl -p
//! ```
//! 
//! Note that the [PollWatcher] is not restricted by this limitation, so it may be an alternative if your users can't increase the limit.
//! 
//! # Examples
//! 
//! For more examples visit the [examples folder](https://github.com/notify-rs/notify/tree/main/examples) in the repository.
//!
//! ```rust,no_exec
//! # use std::path::Path;
//! use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};
//!
//! fn main() -> Result<()> {
//!     // Automatically select the best implementation for your platform.
//!     let mut watcher = notify::recommended_watcher(|res| {
//!         match res {
//!            Ok(event) => println!("event: {:?}", event),
//!            Err(e) => println!("watch error: {:?}", e),
//!         }
//!     })?;
//!
//!     // Add a path to be watched. All files and directories at that path and
//!     // below will be monitored for changes.
//!     watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## With different configurations
//!
//! It is possible to create several watchers with different configurations or implementations that
//! all call the same event function. This can accommodate advanced behaviour or work around limits.
//!
//! ```
//! # use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
//! # use std::path::Path;
//! #
//! # fn main() -> Result<()> {
//!       fn event_fn(res: Result<notify::Event>) {
//!           match res {
//!              Ok(event) => println!("event: {:?}", event),
//!              Err(e) => println!("watch error: {:?}", e),
//!           }
//!       }
//!
//!       let mut watcher1 = notify::recommended_watcher(event_fn)?;
//!       // we will just use the same watcher kind again here
//!       let mut watcher2 = notify::recommended_watcher(event_fn)?;
//! #     watcher1.watch(Path::new("."), RecursiveMode::Recursive)?;
//! #     watcher2.watch(Path::new("."), RecursiveMode::Recursive)?;
//! #
//! #     Ok(())
//! # }
//! ```

#![deny(missing_docs)]

pub use config::{Config, RecursiveMode};
pub use error::{Error, ErrorKind, Result};
pub use event::{Event, EventKind};
use std::path::Path;

#[allow(dead_code)]
#[cfg(feature = "crossbeam-channel")]
pub(crate) type Receiver<T> = crossbeam_channel::Receiver<T>;
#[allow(dead_code)]
#[cfg(not(feature = "crossbeam-channel"))]
pub(crate) type Receiver<T> = std::sync::mpsc::Receiver<T>;

#[allow(dead_code)]