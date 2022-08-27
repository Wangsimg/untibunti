
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
        all_path_data: HashMap<PathBuf, PathData>,
    }

    impl WatchData {
        /// Scan filesystem and create a new `WatchData`.
        ///
        /// # Side effect
        ///
        /// This function may send event by `data_builder.emitter`.
        fn new(data_builder: &DataBuilder, root: PathBuf, is_recursive: bool) -> Option<Self> {
            // If metadata read error at `root` path, it will emit
            // a error event and stop to create the whole `WatchData`.
            //
            // QUESTION: inconsistent?
            //
            // When user try to *CREATE* a watch by `poll_watcher.watch(root, ..)`,
            // if `root` path hit an io error, then watcher will reject to
            // create this new watch.
            //
            // This may inconsistent with *POLLING* a watch. When watcher
            // continue polling, io error at root path will not delete
            // a existing watch. polling still working.
            //
            // So, consider a config file may not exists at first time but may
            // create after a while, developer cannot watch it.
            //
            // FIXME: Can we always allow to watch a path, even file not
            // found at this path?
            if let Err(e) = fs::metadata(&root) {
                data_builder.emitter.emit_io_err(e, &root);
                return None;
            }

            let all_path_data =
                Self::scan_all_path_data(data_builder, root.clone(), is_recursive).collect();

            Some(Self {
                root,
                is_recursive,
                all_path_data,
            })
        }

        /// Rescan filesystem and update this `WatchData`.
        ///
        /// # Side effect
        ///
        /// This function may emit event by `data_builder.emitter`.
        pub(super) fn rescan(&mut self, data_builder: &mut DataBuilder) {
            // scan current filesystem.
            for (path, new_path_data) in
                Self::scan_all_path_data(data_builder, self.root.clone(), self.is_recursive)
            {
                let old_path_data = self
                    .all_path_data
                    .insert(path.clone(), new_path_data.clone());

                // emit event
                let event =
                    PathData::compare_to_event(path, old_path_data.as_ref(), Some(&new_path_data));
                if let Some(event) = event {
                    data_builder.emitter.emit_ok(event);
                }
            }

            // scan for disappeared paths.
            let mut disappeared_paths = Vec::new();
            for (path, path_data) in self.all_path_data.iter() {
                if path_data.last_check < data_builder.now {
                    disappeared_paths.push(path.clone());
                }
            }

            // remove disappeared paths
            for path in disappeared_paths {
                let old_path_data = self.all_path_data.remove(&path);

                // emit event
                let event = PathData::compare_to_event(path, old_path_data.as_ref(), None);
                if let Some(event) = event {
                    data_builder.emitter.emit_ok(event);
                }
            }
        }

        /// Get all `PathData` by given configuration.
        ///
        /// # Side Effect
        ///
        /// This function may emit some IO Error events by `data_builder.emitter`.
        fn scan_all_path_data(
            data_builder: &'_ DataBuilder,
            root: PathBuf,
            is_recursive: bool,
        ) -> impl Iterator<Item = (PathBuf, PathData)> + '_ {
            // WalkDir return only one entry if root is a file (not a folder),
            // so we can use single logic to do the both file & dir's jobs.
            //
            // See: https://docs.rs/walkdir/2.0.1/walkdir/struct.WalkDir.html#method.new
            WalkDir::new(root)
                .follow_links(true)
                .max_depth(Self::dir_scan_depth(is_recursive))
                .into_iter()
                //
                // QUESTION: should we ignore IO Error?
                //
                // current implementation ignore some IO error, e.g.,
                //
                // - `.filter_map(|entry| entry.ok())`
                // - all read error when hashing
                //
                // but the code also interest with `fs::metadata()` error and
                // propagate to event handler. It may not consistent.
                //
                // FIXME: Should we emit all IO error events? Or ignore them all?
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| match entry.metadata() {
                    Ok(metadata) => {
                        let path = entry.into_path();

                        let meta_path = MetaPath::from_parts_unchecked(path, metadata);
                        let data_path = data_builder.build_path_data(&meta_path);

                        Some((meta_path.into_path(), data_path))
                    }
                    Err(e) => {
                        // emit event.
                        let path = entry.into_path();
                        data_builder.emitter.emit_io_err(e, path);

                        None
                    }
                })
        }

        fn dir_scan_depth(is_recursive: bool) -> usize {
            if is_recursive {
                usize::max_value()
            } else {
                1
            }
        }
    }

    /// Stored data for a one path locations.
    ///
    /// See [`WatchData`] for more detail.
    #[derive(Debug, Clone)]
    struct PathData {
        /// File updated time.
        mtime: i64,

        /// Content's hash value, only available if user request compare file
        /// contents and read successful.
        hash: Option<u64>,

        /// Checked time.
        last_check: Instant,
    }

    impl PathData {
        /// Create a new `PathData`.
        fn new(data_builder: &DataBuilder, meta_path: &MetaPath) -> PathData {
            let metadata = meta_path.metadata();

            PathData {
                mtime: FileTime::from_last_modification_time(metadata).seconds(),
                hash: data_builder
                    .build_hasher
                    .as_ref()
                    .filter(|_| metadata.is_file())
                    .and_then(|build_hasher| {
                        Self::get_content_hash(build_hasher, meta_path.path()).ok()
                    }),

                last_check: data_builder.now,
            }
        }

        /// Get hash value for the data content in given file `path`.
        fn get_content_hash(build_hasher: &RandomState, path: &Path) -> io::Result<u64> {
            let mut hasher = build_hasher.build_hasher();
            let mut file = File::open(path)?;
            let mut buf = [0; 512];

            loop {
                let n = match file.read(&mut buf) {
                    Ok(0) => break,