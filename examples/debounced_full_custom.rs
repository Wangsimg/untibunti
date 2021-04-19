use std::{path::Path, time::Duration};

use notify::{RecursiveMode, Config};
use notify_debouncer_mini::new_debouncer_opt;

/// Debouncer with custom backend and waiting for exit
fn main() {
    