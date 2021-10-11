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

/// An event describing open or close operations on files.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum AccessMode {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when the file is executed, or the folder opened.
    Execute,

    /// An event emitted when the file is opened for reading.
    Read,

    /// An event emitted when the file is opened for writing.
    Write,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event describing non-mutating access operations on files.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", content = "mode"))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum AccessKind {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when the file is read.
    Read,

    /// An event emitted when the file, or a handle to the file, is opened.
    Open(AccessMode),

    /// An event emitted when the file, or a handle to the file, is closed.
    Close(AccessMode),

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event describing creation operations on files.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum CreateKind {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event which results in the creation of a file.
    File,

    /// An event which results in the creation of a folder.
    Folder,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event emitted when the data content of a file is changed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum DataChange {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when the size of the data is changed.
    Size,

    /// An event emitted when the content of the data is changed.
    Content,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event emitted when the metadata of a file or folder is changed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum MetadataKind {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when the a