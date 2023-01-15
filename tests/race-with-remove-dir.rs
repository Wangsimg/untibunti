use std::{fs, thread, time::Duration};

use notify::{RecursiveMode, Watcher};

/// Test for <https://github.com/notify-rs/notify/issues/301>.
/// Note: This test will fail if your temp directory is not writable.
