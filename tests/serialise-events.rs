
// This file is dual-licensed under the Artistic License 2.0 as per the
// LICENSE.ARTISTIC file, and the Creative Commons Zero 1.0 license.

use notify::event::*;
#[cfg(feature = "serde")]
use serde_json::json;

#[test]
fn events_are_debuggable() {
    assert_eq!(format!("{:?}", EventKind::Any), String::from("Any"));

    assert_eq!(
        format!(
            "{:?}",
            EventKind::Access(AccessKind::Open(AccessMode::Execute))
        ),
        String::from("Access(Open(Execute))")
    );

    let mut attrs = EventAttributes::new();
    attrs.set_info("unmount");
    attrs.set_flag(Flag::Rescan);

    assert_eq!(
        format!(
            "{:?}",