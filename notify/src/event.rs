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

    /// An event emitted when the access time of the file or folder is changed.
    AccessTime,

    /// An event emitted when the write or modify time of the file or folder is changed.
    WriteTime,

    /// An event emitted when the permissions of the file or folder are changed.
    Permissions,

    /// An event emitted when the ownership of the file or folder is changed.
    Ownership,

    /// An event emitted when an extended attribute of the file or folder is changed.
    ///
    /// If the extended attribute's name or type is known, it should be provided in the
    /// `Info` event attribute.
    Extended,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event emitted when the name of a file or folder is changed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum RenameMode {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted on the file or folder resulting from a rename.
    To,

    /// An event emitted on the file or folder that was renamed.
    From,

    /// A single event emitted with both the `From` and `To` paths.
    ///
    /// This event should be emitted when both source and target are known. The paths should be
    /// provided in this exact order (from, to).
    Both,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event describing mutation of content, name, or metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", content = "mode"))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum ModifyKind {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when the data content of a file is changed.
    Data(DataChange),

    /// An event emitted when the metadata of a file or folder is changed.
    Metadata(MetadataKind),

    /// An event emitted when the name of a file or folder is changed.
    #[cfg_attr(feature = "serde", serde(rename = "rename"))]
    Name(RenameMode),

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event describing removal operations on files.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum RemoveKind {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when a file is removed.
    File,

    /// An event emitted when a folder is removed.
    Folder,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// Top-level event kind.
///
/// This is arguably the most important classification for events. All subkinds below this one
/// represent details that may or may not be available for any particular backend, but most tools
/// and Notify systems will only care about which of these four general kinds an event is about.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum EventKind {
    /// The catch-all event kind, for unsupported/unknown events.
    ///
    /// This variant should be used as the "else" case when mapping native kernel bitmasks or
    /// bitmaps, such that if the mask is ever extended with new event types the backend will not
    /// gain bugs due to not matching new unknown event types.
    ///
    /// This variant is also the default variant used when Notify is in "imprecise" mode.
    Any,

    /// An event describing non-mutating access operations on files.
    ///
    /// This event is about opening and closing file handles, as well as executing files, and any
    /// other such event that is about accessing files, folders, or other structures rather than
    /// mutating them.
    ///
    /// Only some platforms are capable of generating these.
    Access(AccessKind),

    /// An event describing creation operations on files.
    ///
    /// This event is about the creation of files, folders, or other structures but not about e.g.
    /// writing new content into them.
    Create(CreateKind),

    /// An event describing mutation of content, name, or metadata.
    ///
    /// This event is about the mutation of files', folders', or other structures' content, name
    /// (path), or associated metadata (attributes).
    Modify(ModifyKind),

    /// An event describing removal operations on files.
    ///
    /// This event is about the removal of files, folders, or other structures but not e.g. erasing
    /// content from them. This may also be triggered for renames/moves that move files _out of the
    /// watched subpath_.
    ///
    /// Some editors also trigger Remove events when saving files as they may opt for removing (or
    /// renaming) the original then creating a new file in-place.
    Remove(RemoveKind),

    /// An event not fitting in any of the above four categories.
    ///
    /// This may be used for meta-events about the watch itself.
    Other,
}

impl EventKind {
    /// Indicates whether an event is an Access variant.
    pub fn is_access(&self) -> bool {
        matches!(self, EventKind::Access(_))
    }

    /// Indicates whether an event is a Create variant.
    pub fn is_create(&self) -> bool {
        matches!(self, EventKind::Create(_))
    }

    /// Indicates whether an event is a Modify variant.
    pub fn is_modify(&self) -> bool {
        matches!(self, EventKind::Modify(_))
    }

    /// Indicates whether an event is a Remove variant.
    pub fn is_remove(&self) -> bool {
        matches!(self, EventKind::Remove(_))
    }

    /// Indicates whether an event is an Other variant.
    pub fn is_other(&self) -> bool {
        matches!(self, EventKind::Other)
    }
}

impl Default for EventKind {
    fn default() -> Self {
        EventKind::Any
    }
}

/// Notify event.
///
/// You might want to check [`Event::need_rescan`] to make sure no event was missed before you
/// received this one.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Event {
    /// Kind or type of the event.
    ///
    /// This is a hierarchy of enums describing the event as precisely as possible. All enums in
    /// the hierarchy have two variants always present, `Any` and `Other`, accompanied by one or
    /// more specific variants.
    ///
    /// `Any` should be used when more detail about the event is not known beyond the variant
    /// already selected. For example, `AccessMode::Any` means a file has been accessed, but that's
    /// all we know.
    ///
    /// `Other` should be used when more detail _is_ available, but cannot be encoded as one of the
    /// defined variants. When specifying `Other`, the event attributes should contain an `Info`
    /// entry with a short string identifying this detail. That string is to be considered part of
    /// the interface of the backend (i.e. a change should probably be breaking).
    ///
    /// For example, `CreateKind::Other` with an `Info("mount")` may indicate the binding of a
    /// mount. The documentation of the particular backend should indicate if any `Other` events
    /// are generated, and what their description means.
    ///
    /// The `EventKind::Any` variant should be used as the "else" case when mapping native kernel
    /// bitmasks or bitmaps, such that if the mask is ever extended with new event types the
    /// backend will not gain bugs due to not matching new unknown event types.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: EventKind,

    /// Paths the event is about, if known.
    ///
    /// If an event concerns two or more paths, and the paths are known at the time of event
    /// creation, they should all go in this `Vec`. Otherwise, using the `Tracker` attr may be more
    /// appropriate.
    ///
    /// The order of the paths is likely to be significant! For example, renames where both ends of
    /// the name change are known will have the "source" path first, and the "target" path last.
    pub paths: Vec<PathBuf>,

    // "What should be in the struct" and "what can go in the attrs" is an interesting question.
    //
    // Technically, the paths could go in the attrs. That would reduce the type size to 4 pointer
    // widths, instead of 7 like it is now. Anything 8 and below is probably good — on x64 that's
    // the size of an L1 cache line. The entire kind classification fits in 3 bytes, and an AnyMap
    // is 3 pointers. A Vec<PathBuf> is another 3 pointers.
    //
    // Type size aside, what's behind these structures? A Vec and a PathBuf is stored on the heap.
    // An AnyMap is stored on the heap. But a Vec is directly there, requiring about one access to
    // get, while retrieving anything in the AnyMap requires some accesses as overhead.
    //
    // So things that are used often should be on the struct, and things that are used more rarely
    // should go in the attrs. Additionally, arbitrary data can _only_ go in the attrs.
    //
    // The kind and the paths vie for first place on this scale, depending on how downstream wishes
    // to use the information. Everything else is secondary. So far, that's why paths live here.
    //
    // In the future, it might be possible to have more data and to benchmark things properly, so
    // the performance can be actually quantified. Also, it might turn out that I have no idea what
    // I was talking about, so the above may be discarded or reviewed. We'll see!
    //
    /// Additional attributes of the event.
    ///
    /// Arbitrary data may be added to this field, without restriction beyond the `Sync` and
    /// `Clone` properties. Some data added here is considered for comparing and hashing, but not
    /// all: at this writing this is `Tracker`, `Flag`, `Info`, and `Source`.
    #[cfg_attr(feature = "serde", serde(default))]
    pub attrs: EventAttributes,
}

/// Additional attributes of the event.
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventAttributes {
    #[cfg_attr(feature = "serde", serde(flatten))]
    inner: Option<Box<EventAttributesInner>>,
}

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct EventAttributesInner {
    /// Tracking ID for events that are related.
    ///
    /// For events generated by backends with the `TrackRelated` capability. Those backends _may_
    /// emit events that are related to each other, and tag those with an identical "tracking id"
    /// or "cookie". The value is normalised to `usize`.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    tracker: Optio