use std::{path::Path, time::Duration};

use notify::{RecursiveMode};
use notify_debouncer_mini::new_debouncer;

/// Example for debouncer
fn main() {
    // emit some events by changing a file
    std::thread::spawn(|| {
        let path = Path::new("test.txt");
        let _ = std::fs::remove_file(&path);
        loop {
            std::fs::write(&path, b"Lorem ips