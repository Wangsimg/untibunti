# Notify

[![» Crate](https://flat.badgen.net/crates/v/notify)][crate]
[![» Docs](https://flat.badgen.net/badge/api/docs.rs/df3600)][docs]
[![» CI](https://flat.badgen.net/github/checks/notify-rs/notify/main)][build]
[![» Downloads](https://flat.badgen.net/crates/d/notify)][crate]
[![» Conduct](https://flat.badgen.net/badge/contributor/covenant/5e0d73)][coc]
[![» Public Domain](https://flat.badgen.net/badge/license/CC0-1.0/purple)][cc0]

_Cross-platform filesystem notification library for Rust._

(Looking for desktop notifications instead? Have a look at [notify-rust] or
[alert-after]!)

- [API Documentation][docs]
- [Debouncer Documentation][debouncer]
- [Examples][examples]
- [Crate page][crate]
- [Changelog][changelog]
- [Upgrading from v4](UPGRADING_V4_TO_V5.md)
- Earliest supported Rust version: **1.56**
- **incomplete [Guides and in-depth docs][wiki]**

As used by: [alacritty], [cargo watch], [cobalt], [docket], [mdBook], [pax],
[rdiff], [rust-analyzer], [timetrack], [watchexec], [xi-editor], [watchfiles],
and others.

## Platforms

- Linux / Android: inotify
- macOS: FSEvents or kqueue, see features
- Windows: ReadDirectoryChangesW
- FreeBSD / NetBSD / OpenBSD / DragonflyBSD: kqueue
- All platforms: polling

### FSEvents

Due to the inner security model of FSEvents (see [FileSystemEventSecurity]),
some events cannot be observed easily when trying to follow files that do not
belong to you. In this case, reverting to the pollwatcher can fix the issue,
with a slight performance cost.

## License

Notify was undergoing a transition to using the
[Artistic License 2.0][artistic] from [CC Zero 1.0][cc0]. A part of
the code is only under CC0, and another part, including _all new code_ since
commit [`3378ac5a`], is under _both_ CC0 and Artistic. When the project was to be
entirely free of CC0 code, the license 