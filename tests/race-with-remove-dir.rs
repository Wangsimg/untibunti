use std::{fs, thread, time::Duration};

use notify::{RecursiveMode, Watcher};

/// Test for <https://github.com/notify-rs/notify/issues/301>.
/// Note: This test will fail if your temp directory is not writable.
#[test]
fn test_race_with_remove_dir() {
    let tmpdir = tempfile::tempdir().unwrap();

    {
        let tmpdir = tmpdir.path().to_path_buf();
        let _ = thread::Builder::new()
            .name("noti