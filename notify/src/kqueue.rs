//! Watcher implementation for the kqueue API
//!
//! The kqueue() system call provides a generic method of notifying the user
//! when an event happens or a condition holds, based on the results of small
//! pieces of kernel code termed filters.

use super::event::*;
use super::{Config, Error, EventHandler, RecursiveMode, Result, Watcher};
use crate::{unbounded, Receiver, Sender};
use kqueue::{EventData, EventFilter, FilterFlag, Ident};
use std::collections::HashMap;
use std::env;
use std::fs::metadata;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std