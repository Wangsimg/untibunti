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
- CHANGE: Rewrite immediate events to use new system.
- CHANGE: Remove `Sender`s from watcher API in favour of `EventFn` [#214]
- DEPS: Update inotify to 0.8. [#234]
- DEPS: Update crossbeam-channel to 0.4.
- DEPS: \[macOS\] Update fsevent to 2.0.1 and fsevent-sys to 3.0.0.

[#214]: https://github.com/notify-rs/notify/pull/214
[#234]: https://github.com/notify-rs/notify/pull/234

## 4.0.15 (2020-01-07)

- DEPS: Update inotify to 0.7.
- DEPS(DEV): Replace tempdir with tempfile since tempdir is deprecated.
- DEPS: Update winapi to 0.3 and remove kernel32-sys. [#232]

[#232]: https://github.com/notify-rs/notify/pull/232

## 5.0.0-pre.1 (2019-06-30)

_(no changes, just a new release because the old one failed to publish properly)_

## 5.0.0-pre.0 (2019-06-22)

- **yanked 2019-06-30**
- RUSTC: Push the minimum version to 1.36.0 [#201]
- RUSTC: Switch the crate to Rust 2018.
- FIX: Implement `Sync` for PollWatcher to support FreeBSD. [#197]
- FEATURE: Add new runtime configuration system.
- FEATURE: Add `Ongoing` events (optional, configured at runtime). [#146], [#183]
- FEATURE: Bring in new event system from `next` branch. [#187]
- FEATURE: Allow multiple watchers to send to the same channel. [`2a035c86`]
- CHANGE: Switch to crossbeam channel. [#160]
- CHANGE: Rename `Chmod` to `Metadata`. [#179], [#180], previously [#112], [#161]
- CHANGE: Remove `DebouncedEvent` event classification. [#187]
- DEPS: \[Linux\] Upgrade inotify to 0.7. [#184]
- DEPS: \[macOS\] Upgrade fsevent to 0.4. [#195]
- DEPS: Upgrade filetime to 0.2.6.
- META: Rename `v4-legacy` branch to `main`, to further clarify status and prepare for a breaking release.
- DOCS: Change `v5` to `Next Generation Notify` to allow for a breaking release.
- DOCS: Add rust-analyzer to Readme showcase.
- DOCS: Add github issue / PR templates and funding.

[#112]: https://github.com/notify-rs/notify/issues/112
[#146]: https://github.com/notify-rs/notify/issues/146
[#160]: https://github.com/notify-rs/notify/issues/160
[#161]: https://github.com/notify-rs/notify/issues/161
[#179]: https://github.com/notify-rs/notify/issues/179
[#180]: https://github.com/notify-rs/notify/issues/180
[#183]: https://github.com/notify-rs/notify/issues/183
[#184]: https://github.com/notify-rs/notify/issues/184
[#187]: https://github.com/notify-rs/notify/issues/187
[#195]: https://github.com/notify-rs/notify/issues/195
[#197]: https://github.com/notify-rs/notify/issues/197
[#201]: https://github.com/notify-rs/notify/issues/201
[`2a035c86`]: https://github.com/notify-rs/notify/commit/2a035c86c5f12aeee635a827c1f458211ca923ca

## 4.0.15 (2020)

- DEPS: Update winapi to 0.3.8 and remove kernel32-sys. [#232]
- META: The project maintainers are changed from @passcod to notify-rs.

[#232]: https://github.com/notify-rs/notify/pull/232

## 4.0.14 (2019-10-17)

- FIX: Fix deadlock in debouncer. [#210]

[#210]: https://github.com/notify-rs/notify/pull/210

## 4.0.13 (2019-09-01)

- FIX: Undo filetime pin. [#202], [`22e40f5e`]
- META: Project is abandoned.

[#202]: https://github.com/notify-rs/notify/issues/202
[`22e40f5e`]: https://github.com/notify-rs/notify/commit/22e40f5e4cb2a23528f169fc92015f935edc1c55

## 4.0.12 (2019-05-22)

- FIX: Implement `Sync` for PollWatcher to support FreeBSD. [#198]
- DEPS: Peg filetime to 1.2.5 to maintain rustc 1.26.1 compatibility. [#199]

[#198]: https://github.com/notify-rs/notify/issues/198
[#199]: https://github.com/notify-rs/notify/issues/199

## 4.0.11 (2019-05-08)

- DEPS: \[macOS\] Upgrade fsevent to 0.4. [#196]

[#196]: https://github.com/notify-rs/notify/issues/196

## 4.0.10 (2019-03-07)

- FIX: Panic caused by a clock race. [#182]
- DOCS: Add xi to Readme showcase. [`e6f09441`]

[#182]: https://github.com/notify-rs/notify/issues/182
[`e6f09441`]: https://github.com/notify-rs/notify/commit/e6f0944165551fa2ed9ad70e3e11d8b14186fc0a

## 4.0.9 (2019-02-09)

- FIX: High CPU usage in some conditions when using debouncing. [#177], [#178], coming from [rust-analyzer/#556]

[#177]: https://github.com/notify-rs/notify/issues/177
[#178]: https://github.com/notify-rs/notify/issues/178
[rust-analyzer/#556]: https://github.com/rust-analyzer/rust-analyzer/issues/556

## 4.0.8 (2019-02-06)

- DOCS: Mention hotwatch as alternative API. [#175], [`34775f26`]
- DEPS: \[Linux\] Disable `stream` feature for inotify. [#176], [`e729e279`]
- DOCS: Add dates to releases in changelog. [`cc621398`]
- DOCS: Backfill changelog: 4.0.2 to 4.0.7. [`6457f697`]
- DOCS: Backfill changelog: 0.0.1 to 2.6.0. [`d34e6ee7`]

[#175]: https://github.com/notify-rs/notify/issues/175
[`34775f26`]: https://github.com/notify-rs/notify/commit/34775f2695ec236fabc79f2c938e12e4cd54047b
[#176]: https://github.com/notify-rs/notify/issues/176
[`e729e279`]: https://github.com/notify-rs/notify/commit/e729e279f0721c4a5729e725a7cd5e4d761efb58
[`cc621398`]: https://github.com/notify-rs/notify/commit/cc621398e56e2257daf5816e8c2bb01ca79e8ddb
[`6457f697`]: https://github.com/notify-rs/notify/commit/6457f6975a9171483d531fcdafb956d2ee334d55
[`d34e6ee7`]: https://github.com/notify-rs/notify/commit/d34e6ee70df9b4905cbd04fe1a2b5770a9d2a4d4


## 4.0.7 (2019-01-23)

- DOCS: Document unexpected behaviour around watching a tree root. [#165], [#166]
- DOCS: Remove v2 documentation. [`8310b2cc`]
- TESTS: Change how tests are skipped. [`0b4c8400`]
- DOCS: Add timetrack to Readme showcase. [#167]
- META: Change commit message style: commits are now prefixed by a `[topic]`.
- FIX: Make sure debounced watcher terminates. [#170]
- FIX: \[Linux\] Remove thread wake-up on timeout (introduced in 4.0.5 by error). [#174]
- FIX: Restore compatibility with Rust before 1.30.0. [`eab75118`]
- META: Enforce compatibility with Rust 1.26.1 via CI. [`50924cd6`]
- META: Add maintenance status badge. [`ecd686ba`]
- DOCS: Freeze v4 branch (2018-10-05) [`8310b2cc`] — and subsequently unfreeze it. (2019-01-19) [`20c40f99`], [`c00da47c`]

[#165]: https://github.com/notify-rs/notify/issues/165
[#166]: https://github.com/notify-rs/notify/issues/166
[`8310b2cc`]: https://github.com/notify-rs/notify/commit/8310b2ccf68382548914df6ffeaf45248565b9fb
[`0b4c8400`]: https://github.com/notify-rs/notify/commit/0b4c840091f5b3ebd3262d7109308828800dc976
[#167]: https://github.com/notify-rs/notify/issues/167
[#170]: https://github.com/notify-rs/notify/issues/170
[#174]: https://github.com/notify-rs/notify/issues/174
[`eab75118`]: https://github.com/notify-rs/notify/commit/eab75118464dc5d0d48dce31ab7a8e07d7e68d80
[`50924cd6`]: https://github.com/notify-rs/notify/commit/50924cd676c8bce877634e32260ef3872f2feccb
[`ecd686ba`]: https://github.com/notify-rs/notify/commit/ecd686bab604442c315c114e536bdc310a9413b1
[`20c40f99`]: https://github.com/notify-rs/notify/commit/20c40f99ad042fba5abf36f65e9ee598562744d8
[`c00da47c`]: https://github.com/notify-rs/notify/commit/c00da47ce63815972ef7c4bafd3b8c2c11b8b0de


## 4.0.6 (2018-08-30)

- FIX: Add some consts to restore semver compatibility. [`6d4f1ab9`]

[`6d4f1ab9`]: https://github.com/notify-rs/notify/commit/6d4f1ab9af76ecfc856f573a3f5584ddcfe017df


## 4.0.5 (2018-08-29)

- DEPS: Update winapi (0.3), mio (0.6), inotify (0.6), filetime (0.2), bitflags (1.0). [#162]
- SEMVER BREAK: The bitflags upgrade introduced a breaking change to the API.

[#162]: https://github.com/notify-rs/notify/issues/162


## 4.0.4 (2018-08-06)

- Derive various traits for `RecursiveMode`. [#148]
- DOCS: Add docket to Readme showcase. [#154]
- DOCS: [Rename OS X to macOS](https://www.wired.com/2016/06/apple-os-x-dead-long-live-macos/). [#156]
- FIX: \[FreeBSD / Poll\] Release the lock while the thread sleeps (was causing random hangs). [#159]

[#148]: https://github.com/notify-rs/notify/issues/148
[#154]: https://github.com/notify-rs/notify/issues/154
[#156]: https://github.com/notify-rs/notify/issues/156
[#159]: https://github.com/notify-rs/notify/issues/159


## 4.0.3 (2017-11-26)

- FIX: \[macOS\] Concurrency-related FSEvent crash. [#132]
- FIX: \[macOS\] Deadlock due to race in FsEventWatcher. [#118], [#134]
- DEPS: Update walkdir to 2.0. [`fbffef24`]

[#118]: https://github.com/notify-rs/notify/issues/118
[#132]: https://github.com/notify-rs/notify/issues/132
[#134]: https://github.com/notify-rs/notify/issues/134
[`fbffef24`]: https://github.com/notify-rs/notify/commit/fbffef244726aae6e8a98e33ecb77a66274db91b


## 4.0.2 (2017-11-03)

- FI