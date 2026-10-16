# M0 capture wire (2026-10-16)

Signed debug `taco` on tip `4c8dc27` (`cursor/m0-capture-wire-b19b`).
`cargo test` was green before these runs. No Metal load was run.

Stop record: preflight is past `models_unpinned`. With `TACO_CAPTURE_SPIKE=1`
the default selector opens the microphone path and blocks in CoreAudio while
a microphone prompt is up. An operator Allow is still needed before this run
can record a TCC allow or deny. Neither button was pressed. This is not an
allow and it is not a deny.

M0-AC1 through M0-AC6 stay open.

## Host

| Field | Value |
| --- | --- |
| OS | macOS 27.0.1, build 26A434 |
| Arch | arm64 |
| Worktree | `~/workspace/syndai/taco-m0-wire` |
| Commit | `4c8dc27` Wire mic capture behind TACO_CAPTURE_SPIKE. |

This machine is not C1 and it is not a named C2.

## Sign

`cargo build && ./scripts/sign-debug.sh` on `target/debug/taco`.
The script printed `ok: plist bound`. `codesign -dv --verbose=4`:

```text
Identifier=ai.syndai.taco
Format=Mach-O thin (arm64)
CodeDirectory v=20400 size=9159 flags=0x2(adhoc) hashes=281+2 location=embedded
CDHash=bb9d4be510b7955d3be10b62e1ea92051d9da8ec
Signature=adhoc
Info.plist entries=5
TeamIdentifier=not set
```

## Gate off

```text
$ ./target/debug/taco run --source default --from en --to zh
error capture_not_available
```

Exit 1. Stdout was empty. The line above is the entire stderr.
This was not `models_unpinned`.

## Gate on

```text
$ TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source default --from en --to zh
```

Stdout was empty. Stderr was empty. The process did not exit.
After about 45 seconds it was still inside the open, so it was sent SIGTERM.
`wait` then reported 143. That 143 is the signal, not a taco error code.

A second run (pid 28828) was sampled while it was still blocked.
The main thread was in `open_default_microphone` → CPAL `default_input_config`
→ `AudioUnitSetProperty` → `AudioDeviceCreateIOProcID` → `mach_msg`.
It had not reached the 750 ms hold in `open_microphone_spike`.

`tccd` for that first spike (pid 25876, 2026-10-16 23:56:50 KST) logged
`service=kTCCServiceMicrophone`, `preflight=false`, then:

```text
AUTHREQ_PROMPTING: msgID=592.103, service=kTCCServiceMicrophone,
subject=Sub:{com.todesktop.230313mzl4w4u92}Resp:{TCCDProcess: identifier=com.todesktop.230313mzl4w4u92, ... responsible_path=/Applications/Cursor.app/Contents/MacOS/Cursor}
```

The accessor on that request was `identifier=ai.syndai.taco`,
`binary_path=/Users/jwu/workspace/syndai/taco-m0-wire/target/debug/taco`.
`coreaudiod` was the requestor checking access for that accessor.
The prompt subject TCC recorded is Cursor, the responsible process that
spawned this binary, not a bundle id of `ai.syndai.taco` alone.

A screenshot taken while a later spike (pid 29351) was still blocked showed
a system dialog whose title named Cursor and whose text said Cursor would
like to access the microphone. The buttons were Don't Allow and Allow.
Neither button was pressed. The log slice for these runs has no
`AUTHREQ_RESULT` for that microphone prompt. After SIGTERM the dialog
was no longer in the window list.

No `models_unpinned` line was printed. No process-tap selector was run.
