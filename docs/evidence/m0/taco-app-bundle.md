# M0 Taco.app bundle (2026-10-17)

AC1 stays open. This note only packs a signed app around the existing debug Mach-O so the next microphone re-prompt can be attributed to Taco. The app was not launched. No dialog was shown. No button was pressed. Pid 82331 was not signaled and its binary was not replaced.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.

## Host

Apple M1 Pro, 34359738368 bytes, macOS 27.0.1 (26A434), arm64. This is not corner C1 and it is not a qualified C2. Measured at 02:03 KST in worktree `~/workspace/syndai/taco-m0-app` on `cursor/m0-taco-app-bundle-b19b`, parent `cd9a4c8` (`cursor/m0-process-tap-b19b`).

## What was packed

`scripts/m0-pack-taco-app.sh` copied the already-signed debug binary and ad-hoc signed the copy inside the bundle. It did not `codesign` the source path.

| | |
| --- | --- |
| Source | `/Users/jwu/workspace/syndai/taco-m0-tap/target/debug/taco` |
| Source mtime | 2026-10-17 01:33 |
| Source inode | 156923676 |
| App | `/Users/jwu/workspace/syndai/taco-m0-app/packaging/Taco.app` |
| App executable inode | 156979312 |

The source file is the one pid 82331 still has mapped (`taco run --source default --from en --to zh`, elapsed and still `Ss` after the copy). That worktree HEAD was `95d05e7`, which is ahead of published #11 (`cd9a4c8`) by the open-timeout and hold commits. This branch does not contain those Rust changes. The on-disk app is a copy of that newer binary, not a rebuild from `cd9a4c8`. `strings` on the copy shows `TACO_CAPTURE_SPIKE`, `TACO_CAPTURE_SPIKE_OPEN_MS`, and `TACO_CAPTURE_SPIKE_HOLD_MS`.

`packaging/Info.plist` keeps the keys already bound on #4 (`assets/Info.plist` at this parent): `CFBundleIdentifier` `ai.syndai.taco`, `CFBundleName` `taco`, `CFBundleExecutable` `taco`, `NSMicrophoneUsageDescription`, and `NSAudioCaptureUsageDescription`. The usage strings are unchanged. The bundle plist also has `CFBundlePackageType` `APPL`, `CFBundleDisplayName` `Taco`, and the version keys Launch Services needs for an app-attributed sheet. `plutil -lint packaging/Info.plist` printed `OK`.

## codesign -dv

Source, after the copy, still the bare Mach-O. `Info.plist entries=5` is the embedded plist from `scripts/sign-debug.sh`.

```text
Executable=/Users/jwu/workspace/syndai/taco-m0-tap/target/debug/taco
Identifier=ai.syndai.taco
Format=Mach-O thin (arm64)
CodeDirectory v=20400 size=9639 flags=0x2(adhoc) hashes=296+2 location=embedded
CDHash=9b88412e3f579dc52070c68aa9fb88f462d8ec6f
Signature=adhoc
Info.plist entries=5
TeamIdentifier=not set
```

Bundle, `codesign -dv --verbose=4 packaging/Taco.app`:

```text
Executable=/Users/jwu/workspace/syndai/taco-m0-app/packaging/Taco.app/Contents/MacOS/taco
Identifier=ai.syndai.taco
Format=app bundle with Mach-O thin (arm64)
CodeDirectory v=20400 size=9671 flags=0x2(adhoc) hashes=296+3 location=embedded
CDHash=43cd516ef42658381dab6afa5e3ea9732115fa65
Signature=adhoc
Info.plist entries=10
TeamIdentifier=not set
Sealed Resources version=2 rules=13 files=0
```

`Info.plist entries=10` counts the bundle plist, including the five proven keys. Ad-hoc is not a Developer ID, not notarization, and not a TCC allow or deny.

## Next launch

Do not run this while pid 82331 is still the live spike. `scripts/m0-launch-taco-app.sh` is the launcher. It does not start until that script is executed. `OPEN_MS` and `HOLD_MS` are short names. The binary reads `TACO_CAPTURE_SPIKE_OPEN_MS` and `TACO_CAPTURE_SPIKE_HOLD_MS`. Unset `OPEN_MS` uses `300000`. Unset `HOLD_MS` uses `1800000`, the same default as the hold in the copied binary.

```text
OPEN_MS=300000 HOLD_MS=1800000 ./scripts/m0-launch-taco-app.sh
```

That is:

```text
open -W \
  --env TACO_CAPTURE_SPIKE=1 \
  --env TACO_CAPTURE_SPIKE_OPEN_MS=300000 \
  --env TACO_CAPTURE_SPIKE_HOLD_MS=1800000 \
  /Users/jwu/workspace/syndai/taco-m0-app/packaging/Taco.app \
  --args run --source default --from en --to zh
```

`open` is Launch Services, so the sheet's responsible app can be Taco rather than the bare Mach-O under Cursor. This command was not run. M0-AC1 stays unchecked.
