/// Example for watching kernel internal filesystems like `/sys` and `/proc`
/// These can't be watched by the default backend or unconfigured pollwatcher
/// This example can't be demonstrated under windows, it might be relevant for network shares
#[cfg(not(target_os = "windows"))]
fn not_windows_main() -> notify::Result<()> {
    use notify::{PollWatcher, RecursiveMode, Watcher, Config};
    use std::path::Path;
    use std::time::Duration;

    let mut paths: Vec<_> = std::env::args()
        .skip(1)
        .map(|arg| Path::new(&arg).to_path_buf())
        .collect();
    if paths.is_empty() {
        let lo_stats = Path::new("/sys/class/net/lo/statistics/tx_bytes").to_path_buf();
        if !lo_stats.exists() {
            