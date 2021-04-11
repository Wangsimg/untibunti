use std::{path::Path, time::Duration};

use notify::{RecursiveMode};
use notify_debouncer_mini::new_debouncer;

/// Example for debouncer
fn main() {
    // emit some events by changing a file
