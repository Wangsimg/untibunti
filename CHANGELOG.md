# Changelog

v4 commits split out to branch `v4_maintenance` starting with `4.0.16`

## notify 5.1.0 (2023-01-15)

- CHANGE: switch from winapi to windows-sys [#457]
- FIX: kqueue-backend: batch file-watching together to improve performance [#454]
- DOCS: include license file in crate again [#461]
- DOCS: typo and examples fixups

[#454]: https://github.com/notify-rs/notify/pull/454
[#461]: https://github.com/notify-rs/notify/pull/461
[#457]: https://github.com/notify-rs/notify/pull/457

## debouncer-mini 0.2.1 (2022-09-05)

- DOCS: correctly document the `crossbeam` feature [#440]

[#440]: https://github.com/notify-rs/notify/pull/440

## debouncer-mini 0.2.0 (2022-08-30)

Upgrade notify dependency to 5.0.0

## notify 5.0.0 (2022-08-28)

For a list of changes when upgrading from v4 see [UPGRADING_V4_TO_V5.md](UPGRADING_V4_TO_V5.md).

Differences to 5.0.0-pre.16:

- FIX: update minimum walkdir version to 2.2.2 [#432]
- CHANGE: add `need_rescan` function to `Event`, allowing easier detection when a rescan is required [#435]
- FIX: debouncer-mini: change crossbeam feature to `crossbeam`, to allow passthrough with notify re-exports [#429]
- DOCS: improve v5-to-v5 upgrade docs [#431]
- DOCS: file back v4 changelog into main [#437]
- DOCS: cleanups and link fixes

[#431]: https://github.com/notify-rs/notify/pull/431
[#432]: https://github.com/notify-rs/notify/pull/432
[#437]: https://github.com/notify-rs/notify/pull/437
[#435]: https://github.com/notify-rs/notify/pull/435
[#429]: https://github.com/notify-rs/notify/pull/429

## 5.0.0-pre.16 (2022-08-12)

- CHANGE: require config for watcher creation and unify config [#426]
- CHANGE: fsevent: use RenameMode::Any for renaming events [#371]
- FEATURE: re-add debouncer as 