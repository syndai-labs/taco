# M0 Taco.app bundle

Cited from `cursor/m0-taco-app-bundle-b19b` tip `7fb5006` (PR #46). This branch does not replay `scripts/m0-pack-taco-app.sh`, `scripts/m0-launch-taco-app.sh`, or `packaging/Info.plist`. No acceptance box is checked. The app was not launched. No dialog was shown. No button was pressed. Pid 82331 was not signaled and its binary was not replaced.

Host recorded on that tip: Apple M1 Pro, 34359738368 bytes, macOS 27.0.1 (26A434), arm64. That note says this is not corner C1 and it is not a qualified C2. Measured at 02:03 KST in worktree `~/workspace/syndai/taco-m0-app`, parent `cd9a4c8`.

`scripts/m0-pack-taco-app.sh` copied the already-signed debug binary and ad-hoc signed the copy inside the bundle. It did not `codesign` the source path. Source inode `156923676`. App executable inode `156979312`. The source file is the one pid 82331 still had mapped. That worktree HEAD was `95d05e7`, ahead of published #11 (`cd9a4c8`) by the open-timeout and hold commits. This branch does not contain those Rust changes. The on-disk app is a copy of that newer binary, not a rebuild from `cd9a4c8`.

Source `codesign -dv`: `Identifier=ai.syndai.taco`, `Signature=adhoc`, `Info.plist entries=5`, `TeamIdentifier=not set`, `CDHash=9b88412e3f579dc52070c68aa9fb88f462d8ec6f`.

Bundle `codesign -dv`: `Format=app bundle with Mach-O thin (arm64)`, `Signature=adhoc`, `Info.plist entries=10`, `TeamIdentifier=not set`, `CDHash=43cd516ef42658381dab6afa5e3ea9732115fa65`. Ad-hoc is not a Developer ID, not notarization, and not a TCC allow or deny.

`scripts/m0-launch-taco-app.sh` was not run. Unset `OPEN_MS` uses `300000`. Unset `HOLD_MS` uses `1800000`.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
