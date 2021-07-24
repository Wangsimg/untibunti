// This file is dual-licensed under the Artistic License 2.0 as per the
// LICENSE.ARTISTIC file, and the Creative Commons Zero 1.0 license.
//! The `Event` type and the hierarchical `EventKind` descriptor.

use std::{
    fmt,
    hash::{Hash, Hasher},
    path::PathBuf,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An event describing open or close operat