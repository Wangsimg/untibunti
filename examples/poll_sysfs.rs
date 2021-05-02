/// Example for watching kernel internal filesystems like `/sys` and `/proc`
/// These can't be watched by the default backend or unconfigured pollwatcher
/// This example can't be demonstrated under windows, it might be relevant for network shares
#[cfg(not(target_os = "windows"))]
fn not_windows_main() -> notify::Result<()> {
    use notify::{Pol