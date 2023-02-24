
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
            Event {
                kind: EventKind::Remove(RemoveKind::Other),
                paths: vec!["/example".into()],
                attrs
            }
        ),
        String::from(
            "Event { kind: Remove(Other), paths: [\"/example\"], attr:tracker: None, attr:flag: Some(Rescan), attr:info: Some(\"unmount\"), attr:source: None }"
        )
    );
}

#[cfg(feature = "serde")]
#[test]
fn events_are_serializable() {
    assert_eq!(json!(EventKind::Any), json!("any"));

    assert_eq!(json!(EventKind::Other), json!("other"));

    assert_eq!(
        json!(Event {
            kind: EventKind::Access(AccessKind::Open(AccessMode::Execute)),
            paths: Vec::new(),
            attrs: EventAttributes::new(),
        }),
        json!({
            "type": { "access": { "kind": "open", "mode": "execute" } },
            "paths": [],
            "attrs": {},
        })
    );

    let mut attrs = EventAttributes::new();
    attrs.set_info("unmount".into());

    assert_eq!(
        json!(Event {
            kind: EventKind::Remove(RemoveKind::Other),
            paths: vec!["/example".into()],
            attrs: attrs.clone(),
        }),
        json!({
            "type": { "remove": { "kind": "other" } },
            "paths": ["/example"],
            "attrs": { "info": "unmount" }
        }),
        "{:#?} != {:#?}",
        json!(Event {
            kind: EventKind::Remove(RemoveKind::Other),
            paths: vec!["/example".into()],
            attrs: attrs.clone(),
        }),
        json!({
            "type": { "remove": { "kind": "other" } },
            "paths": ["/example"],
            "attrs": { "info": "unmount" }
        }),
    );
}

#[cfg(feature = "serde")]
#[test]
fn events_are_deserializable() {
    assert_eq!(
        serde_json::from_str::<EventKind>(r#""any""#).unwrap(),
        EventKind::Any
    );

    assert_eq!(
        serde_json::from_str::<EventKind>(r#""other""#).unwrap(),
        EventKind::Other
    );

    assert_eq!(
        serde_json::from_str::<Event>(
            r#"{
        "type": { "access": { "kind": "open", "mode": "execute" } },
        "paths": [],
        "attrs": {}
    }"#
        )
        .unwrap(),
        Event {
            kind: EventKind::Access(AccessKind::Open(AccessMode::Execute)),
            paths: Vec::new(),
            attrs: EventAttributes::new(),
        }
    );

    let mut attrs = EventAttributes::new();
    attrs.set_info("unmount".into());

    assert_eq!(
        serde_json::from_str::<Event>(
            r#"{
        "type": { "remove": { "kind": "other" } },
        "paths": ["/example"],
        "attrs": { "info": "unmount" }
    }"#
        )
        .unwrap(),
        Event {
            kind: EventKind::Remove(RemoveKind::Other),
            paths: vec!["/example".into()],
            attrs
        }
    );
}

#[cfg(feature = "serde")]
#[test]
fn access_events_are_serializable() {
    assert_eq!(
        json!(EventKind::Access(AccessKind::Any)),
        json!({
            "access": { "kind": "any" }
        })
    );

    assert_eq!(
        json!(EventKind::Access(AccessKind::Read)),
        json!({
            "access": { "kind": "read" }
        })
    );

    assert_eq!(
        json!(EventKind::Access(AccessKind::Open(AccessMode::Any))),
        json!({
            "access": { "kind": "open", "mode": "any" }
        })
    );

    assert_eq!(
        json!(EventKind::Access(AccessKind::Open(AccessMode::Execute))),
        json!({
            "access": { "kind": "open", "mode": "execute" }
        })
    );

    assert_eq!(
        json!(EventKind::Access(AccessKind::Open(AccessMode::Read))),
        json!({
            "access": { "kind": "open", "mode": "read" }
        })
    );

    assert_eq!(
        json!(EventKind::Access(AccessKind::Close(AccessMode::Write))),
        json!({
            "access": { "kind": "close", "mode": "write" }
        })
    );

    assert_eq!(
        json!(EventKind::Access(AccessKind::Close(AccessMode::Other))),
        json!({
            "access": { "kind": "close", "mode": "other" }
        })
    );

    assert_eq!(
        json!(EventKind::Access(AccessKind::Other)),
        json!({
            "access": { "kind": "other" }
        })
    );