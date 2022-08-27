
//! Generic Watcher implementation based on polling
//!
//! Checks the `watch`ed paths periodically to detect changes. This implementation only uses
//! Rust stdlib APIs and should work on all of the platforms it supports.

use crate::{EventHandler, RecursiveMode, Watcher, Config};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use data::{DataBuilder, WatchData};
mod data {
    use crate::{
        event::{CreateKind, DataChange, Event, EventKind, MetadataKind, ModifyKind, RemoveKind},
        EventHandler,
    };
    use filetime::FileTime;
    use std::{
        cell::RefCell,
        collections::{hash_map::RandomState, HashMap},
        fmt::{self, Debug},
        fs::{self, File, Metadata},
        hash::{BuildHasher, Hasher},
        io::{self, Read},
        path::{Path, PathBuf},
        time::Instant,
    };
    use walkdir::WalkDir;

    /// Builder for [`WatchData`] & [`PathData`].
    pub(super) struct DataBuilder {
        emitter: EventEmitter,

        // TODO: May allow user setup their custom BuildHasher / BuildHasherDefault
        // in future.
        build_hasher: Option<RandomState>,

        // current timestamp for building Data.
        now: Instant,
    }

    impl DataBuilder {
        pub(super) fn new<F>(event_handler: F, compare_content: bool) -> Self
        where
            F: EventHandler,
        {
            Self {
                emitter: EventEmitter::new(event_handler),
                build_hasher: compare_content.then(RandomState::default),
                now: Instant::now(),
            }
        }

        /// Update internal timestamp.
        pub(super) fn update_timestamp(&mut self) {
            self.now = Instant::now();
        }

        /// Create [`WatchData`].
        ///
        /// This function will return `Err(_)` if can not retrieve metadata from
        /// the path location. (e.g., not found).
        pub(super) fn build_watch_data(
            &self,
            root: PathBuf,
            is_recursive: bool,
        ) -> Option<WatchData> {
            WatchData::new(self, root, is_recursive)
        }

        /// Create [`PathData`].
        fn build_path_data(&self, meta_path: &MetaPath) -> PathData {
            PathData::new(self, meta_path)
        }
    }

    impl Debug for DataBuilder {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("DataBuilder")
                .field("build_hasher", &self.build_hasher)
                .field("now", &self.now)
                .finish()
        }
    }

    #[derive(Debug)]
    pub(super) struct WatchData {
        // config part, won't change.
        root: PathBuf,
        is_recursive: bool,

        // current status part.