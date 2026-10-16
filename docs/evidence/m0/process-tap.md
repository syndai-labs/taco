# M0 process tap wiring

AC1 stays open. Rust now calls `native/process_tap.m` for `system`, `app:`, and `pid:` through the `taco-tap` crate. That crate is the `unsafe` boundary. `taco` still has `forbid(unsafe_code)`. The `.m` file is compiled only on macOS and uses CoreAudio process taps (`AudioHardwareCreateProcessTap`, private aggregate), not a stub.

`TACO_CAPTURE_SPIKE` gates every real open, same as the microphone path. Unset, `start_capture` returns `capture_not_available` and does not call the tap. On a non-macOS host the spike env returns `platform_unsupported` before any open. Linux `cargo test` does not set the env and does not open a device.

Tap open creates the process tap and private aggregate, then `Drop` destroys them. It does not start device IO. This tree does not record a TCC prompt.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.

## Mac follow-up

Still needs a signed arm64 binary, `TACO_CAPTURE_SPIKE=1`, and `taco run` for `default`, `system`, and `app:<bundle>` (or `pid:`) to show the real TCC / process-tap prompt, allow, deny, and cleanup. Do not check AC1 from this wiring alone.

## Signed Mac run (2026-10-17)

Tip `36ccc74` in `~/workspace/syndai/taco-m0-tap`. macOS 27.0.1 (26A434), arm64. This machine is not C1 and it is not a named C2. `cargo test` was green (one dead-code warning on `ProcessTap.inner`). `./scripts/sign-debug.sh` printed `ok: plist bound`.

```text
Identifier=ai.syndai.taco
flags=0x2(adhoc)
CDHash=e30d6ba1fef1e281c6156ca86438f029aafda0d6
Signature=adhoc
Info.plist entries=5
TeamIdentifier=not set
```

No process was killed. No dialog was shown. No button was pressed. This is not an operator Allow and it is not a Deny. M0-AC1 stays unchecked.

### Gate off, `system`

```text
$ ./target/debug/taco run --source system --from en --to zh
error capture_not_available
```

Exit 1. Stdout was empty. Not `models_unpinned`.

### Gate on, `system`

```text
$ TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source system --from en --to zh
```

Stdout empty. Stderr empty. Exit 0. The process returned in well under a second (pid 39296, about 00:05:34 KST). It did not hang the way the microphone spike did, so it was not sampled and it was not sent a signal.

`tccd` for that pid logged a microphone preflight, not a prompt:

```text
AUTHREQ_CTX: msgID=39296.1, service=kTCCServiceMicrophone, preflight=yes
AUTHREQ_ATTRIBUTION: requesting identifier=ai.syndai.taco, binary_path=.../taco-m0-tap/target/debug/taco, responsible Cursor.app
AUTHREQ_RESULT: msgID=39296.1, authValue=2, authReason=2, authVersion=1, desired_auth=0, error=(null)
```

No `AUTHREQ_PROMPTING` line for this pid. The same log slice had no `kTCCServiceAudioCapture` line. Exit 0 means `open_system_tap` returned Ok and the tap value was dropped before the process exited. The destroy call was not separately observed.

A microphone Allow dialog would not have changed this command. It did not wait on one. The earlier `default` mic spike is a different path.

### Gate on, `app:com.apple.Terminal`

`pgrep -x Terminal` was empty. Terminal was not running.

```text
$ TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source app:com.apple.Terminal --from en --to zh
error capture_not_available
```

Exit 1. Stdout empty. The native status string is not on stderr. This is not a TCC allow or deny.

### `pid:741` (Finder)

Ungated: stderr `error capture_not_available`, exit 1.

```text
$ TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source pid:741 --from en --to zh
error capture_not_available
```

Exit 1. Stdout empty. Returned immediately. No dialog. Not a TCC allow or deny.
