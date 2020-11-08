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
- FEATURE: Add Watcher::kind() [#364]
- FEATURE: Add more Debug/Copy trait impls [#377] [#378]
- FIX: Fix selection of RecommendedWatcher for macos_kqueue feature  [#362]
- FIX: Turn possible panic into an error in FSEvents backend when file is deleted rapidly [#369]
- FIX: lqueue: emit Create Events and watch all files in a directory [#372]
- FIX: inotify: don't panic on shutdown [#373]

[#386]: https://github.com/notify-rs/notify/pull/386
[#360]: https://github.com/notify-rs/notify/pull/360
[#370]: https://github.com/notify-rs/notify/pull/370
[#371]: https://github.com/notify-rs/notify/pull/371
[#383]: https://github.com/notify-rs/notify/pull/383
[#364]: https://github.com/notify-rs/notify/pull/364
[#377]: https://github.com/notify-rs/notify/pull/377
[#378]: https://github.com/notify-rs/notify/pull/378
[#362]: https://github.com/notify-rs/notify/pull/362
[#369]: https://github.com/notify-rs/notify/pull/369
[#372]: https://github.com/notify-rs/notify/pull/372
[#373]: https://github.com/notify-rs/notify/pull/373


## 5.0.0-pre.13 (2021-09-07)

- Fix: Add path information to inotify and kqueue watch/unwatch errors  [#354]
- Fix: Delete dbg call from kqueue.rs  [#357]

[#354]: https://github.com/notify-rs/notify/pull/354
[#357]: https://github.com/notify-rs/notify/pull/357

## 5.0.0-pre.12 (2021-08-12)

- CHANGE: Move creation of watcher into trait [#345]
- CHANGE: Add EventHandler trait to replace EventFn [#346]
- FIX: Fix build failure on x86_64-unknown-netbsd [#347]

[#345]: https://github.com/notify-rs/notify/pull/345
[#346]: https://github.com/notify-rs/notify/pull/346
[#347]: https://github.com/notify-rs/notify/pull/347

## 5.0.0-pre.11 (2021-07-22)

- FEATURE: Add `Kqueue` backend for use on BSD [#335]
- CHANGE: Change EventFn to take FnMut [#333]
- CHANGE: Make `Watcher` object safe [#336]
- FIX: Join thread in `fseven` on shutdown [#337]
- FIX: Only check for ENOSPC on inotify_add_watch in `inotify` [#330]
- FIX: Free context when stream is deallocated in `fsevent` [#329]
- DOCS: Fix missing comma in docs [#340]

[#333]: https://github.com/notify-rs/notify/pull/333
[#336]: https://github.com/notify-rs/notify/pull/336
[#340]: https://github.com/notify-rs/notify/pull/340
[#337]: https://github.com/notify-rs/notify/pull/337
[#335]: https://github.com/notify-rs/notify/pull/335
[#330]: https://github.com/notify-rs/notify/pull/330
[#329]: https://github.com/notify-rs/notify/pull/329

## 5.0.0-pre.10 (2021-06-04)

- FIX: Make StreamContextInfo `Send` to fix soundness issue [#325]

[#325]: https://github.com/notify-rs/notify/pull/325

## 5.0.0-pre.9 (2021-05-21)

- DEPS: Upgrade fsevent-sys dependency to 4.0 [#322]
- CHANGE: Remove dependency on `fsevent`. [#313]
- FIX: Correct the return type for `CFRunLoopIsWaiting` to be `Boolean` [#319]
- CHANGE: Hide fsevent::{CFRunLoopIsWaiting,callback}, fix clippy lint warnings [#312]
- FIX: Fix some clippy lints [#320]

[#319]: https://github.com/notify-rs/notify/pull/319
[#313]: https://github.com/notify-rs/notify/pull/313
[#312]: https://github.com/notify-rs/notify/pull/312
[#320]: https://github.com/notify-rs/notify/pull/320
[#322]: https://github.com/notify-rs/notify/pull/322

## 4.0.17 (2021-05-13)

- FIX: Don't crash on macos when creating & deleting folders in rapid succession [#303]

[#303]: https://github.com/notify-rs/notify/pull/303

## 5.0.0-pre.8 (2021-05-12)

- HOTFIX: Fix breaking change in fsevent-sys in minor version destroying builds [#316]
- FIX: Don't crash on macos when creating & deleting folders in rapid succession [#302]
- FIX: Remove `anymap`, and replace event attributes with an opaque type. [#306]

[#302]: https://github.com/notify-rs/notify/pull/302
[#306]: https://github.com/notify-rs/notify/pull/306
[#316]: https://github.com/notify-rs/notify/pull/316

## 5.0.0-pre.7 (2021-04-15)

- FIX: Display proper error message when reaching inotify limits on linux [#285]
- FIX: Fix leaks on Windows [#298]

[#285]: https://github.com/notify-rs/notify/pull/285
[#298]: https://github.com/notify-rs/notify/pull/298

## 5.0.0-pre.6 (2021-02-20)

- FIX: Handle interrupted system call errors from mio [#281]

[#281]: https://github.com/notify-rs/notify/pull/281

## 5.0.0-pre.5 (2021-01-28)

- RUSTC: Push the minimum version to 1.47.0 [#280]
- DEPS: Update `inotify` to 0.9 [#280]
- DEPS: Update `mio` to 0.7 and remove `mio-extras` [#278]
- FIX: Report events promptly on Linux, even when many occur in rapid succession. [#268]

[#280]: https://github.com/notify-rs/notify/pull/280
[#278]: https://github.com/notify-rs/notify/pull/278

## 5.0.0-pre.4 (2020-10-31)

- CHANGE: Avoid stating the watched path for non-recursive watches with inotify [#256]
- DOCS: Fix broken link in crate documentation [#260]

[#256]: https://github.com/notify-rs/notify/pull/256
[#260]: https://github.com/notify-rs/notify/pull/260

## 5.0.0-pre.3 (2020-06-22)

- DEPS: Removed unused chashmap dependency [#242]

[#242]: https://github.com/notify-rs/notify/pull/242

## 4.0.16 (2021-04-14)

- FIX: Report events promptly on Linux, even when many occur in rapid succession. [#268]
- FIX: Fix leaks on Windows and debounce module. [#288]
- FIX: Display proper error message when reaching inotify limits on linux. [#290]

[#268]: https://github.com/notify-rs/notify/pull/268
[#288]: https://github.com/notify-rs/notify/pull/288
[#290]: https://github.com/notify-rs/notify/pull/290

## 5.0.0-pre.2 (2020-01-07)

- (Temporary): Remove event debouncing.
- (Temporary): Remove tests.
- 