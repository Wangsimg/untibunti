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
- FEATURE: re-add debouncer as new crate and fixup CI [#286]
- FEATURE: allow disabling crossbeam-channel dependency [#425]
- FIX: PollWatcher panic after delete-and-recreate [#406]
- MISC: rework pollwatcher internally [#409]
- DOCS: cleanup all docs towards v5 [#395]

[#395]: https://github.com/notify-rs/notify/pull/395
[#406]: https://github.com/notify-rs/notify/pull/406
[#409]: https://github.com/notify-rs/notify/pull/409
[#425]: https://github.com/notify-rs/notify/pull/425
[#286]: https://github.com/notify-rs/notify/pull/286
[#426]: https://github.com/notify-rs/notify/pull/426
[#371]: https://github.com/notify-rs/notify/pull/371

## 5.0.0-pre.15 (2022-04-30)

- CHANGE: raise MSRV to 1.56! [#396] and [#402]
- FEATURE: add support for pseudo filesystems like sysfs/procfs [#396]
- FIX: Fix builds on (Free)BSD due to changes in kqueue fix release [#399]

[#396]: https://github.com/notify-rs/notify/pull/396
[#399]: https://github.com/notify-rs/notify/pull/399
[#402]: https://github.com/notify-rs/notify/pull/402

## 5.0.0-pre.14 (2022-03-13)

- CHANGE: upgrade mio to 0.8 [#386]
- CHANGE: PollWatcher: unify signature of new and with_delay  [#360]
- CHANGE: emit EventKind::Modify on kqueue write event [#370]
- CHANGE: use RenameMode::Any for renaming events [#371]
- CHANGE: name all threads spawned by notify [#383]
- FEATURE: Add Watcher::ki