use std::{path::Path, time::Duration};

use notify::{RecursiveMode, Config};
use notify_debouncer_mini::new_debouncer_opt;

/// Debouncer with custom backend and waiting for exit
fn main() {
    // emit some events by changing a file
    std::thread::spawn(|| {
        let path = Path::new("test.txt");
        let _ = std::fs::remove_file(&path);
        loop {
            std::