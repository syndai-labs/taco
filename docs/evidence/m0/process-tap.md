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

`authValue=2` from the system-spike preflight stays a number. `man` has no `tcc` or `tccd` page on this host, and the `tccd` binary does not define which integer is denied, restricted, or allowed. It is not recorded as either.

## Terminal running (2026-10-17 00:10 KST)

`open -a Terminal`. `pgrep -x Terminal` printed `42771 Terminal`. Same signed binary as the run above. Neither command hung. No dialog was shown. No button was pressed. The CLI does not print the native tap status; stderr is only the closed error string. M0-AC1 stays unchecked.

### `app:com.apple.Terminal`

```text
$ TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source app:com.apple.Terminal --from en --to zh
error capture_not_available
```

Exit 1. Stdout empty. Process 43241 returned immediately.

### `pid:42771`

```text
$ TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source pid:42771 --from en --to zh
error capture_not_available
```

Exit 1. Stdout empty. Process 43248 returned immediately. Not a TCC allow or deny.

## Native status with Terminal still live (2026-10-17 00:15 KST)

Same Terminal pid `42771`. A spike-only `fprintf` in `native/process_tap.m` printed the HAL result, then that print was removed. `forbid(unsafe_code)` stayed on `taco`. The lookup was not changed: this is not a binding bug, and a miss does not become a system tap. M0-AC1 stays unchecked.

Diagnostic binary `CDHash=d52e10825f6efdc77472f9404e250661a22a04cf`, Info.plist entries=5, ad-hoc.

`TACO_CAPTURE_SPIKE=1` `--source app:com.apple.Terminal`:

```text
capture_tap where=find_bundle taco=1 osstatus=0 count=29 matches=0
error capture_not_available
```

Exit 1. `taco=1` is `TACO_NOT_FOUND`. `osstatus=0` is `noErr`. The process list read succeeded (29 HAL clients). None had bundle id `com.apple.Terminal`. `AudioHardwareCreateProcessTap` was not called.

`TACO_CAPTURE_SPIKE=1` `--source pid:42771`:

```text
capture_tap where=find_pid taco=1 osstatus=0 pid=42771 object=0 in_list=0
error capture_not_available
```

Exit 1. `object=0` is `kAudioObjectUnknown`. The pid was not in the same process list. CoreAudio's `kAudioHardwarePropertyTranslatePIDToProcessObject` returns that unknown id without an error when the pid is not a HAL client. `kAudioHardwarePropertyProcessObjectList` is only clients connected to CoreAudio. Terminal was running and was not one of them. No dialog. Not an allow or a deny.

## Chrome, a client already on the list (2026-10-17 00:18 KST)

A one-shot list of `kAudioHardwarePropertyProcessObjectList` (not kept in the tree) printed `count=29 osstatus=0`. Terminal was still absent. Chrome was present:

```text
object=103 pid=688 pid_os=0 running=0 running_os=0 bundle=com.google.Chrome
```

`running=0` means `kAudioProcessPropertyIsRunning` was 0. No media was started. The main bundle id appeared once. Helper processes were `com.google.Chrome.helper`, so `app:com.google.Chrome` is not an ambiguous match. The miss was not turned into a system tap. M0-AC1 stays unchecked.

Signed `target/debug/taco` after `./scripts/sign-debug.sh`:

```text
Identifier=ai.syndai.taco
CDHash=adbd84482322ad3d53fc7eef81d356eba28d2ccb
Signature=adhoc
Info.plist entries=5
TeamIdentifier=not set
```

`ps` still showed pid 688 as `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`.

```text
$ TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source app:com.google.Chrome --from en --to zh
```

Stdout empty. Stderr empty. Exit 0. Did not hang. No dialog was shown. No button was pressed.

```text
$ TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source pid:688 --from en --to zh
```

Stdout empty. Stderr empty. Exit 0. Did not hang. No dialog was shown. No button was pressed.

`tccd` for those two processes (52235 and 52251) logged a microphone preflight only, with no `AUTHREQ_PROMPTING` line:

```text
service=kTCCServiceMicrophone preflight=yes
AUTHREQ_RESULT authValue=2 authReason=2
```

`authValue=2` stays a number. This is not an operator Allow or Deny. No `kTCCServiceAudioCapture` line was in that slice. Exit 0 means the tap open returned and the process-tap value was dropped before exit, which destroys the tap and the private aggregate in-process. That destroy was not separately traced in a log.

### Why exit 0 had no dialog

Both Chrome spike processes sent a microphone preflight. `tccd` attributed the request to responsible process Cursor (`com.todesktop.230313mzl4w4u92`, `/Applications/Cursor.app`) and accessor `ai.syndai.taco` (`.../taco-m0-tap/target/debug/taco`). The subject of the preflight was Cursor's bundle id. There was no `AUTHREQ_PROMPTING` line. The process exited before a dialog could be required.

`tccutil list` on this host, which prints its own granted/denied word, said:

```text
Microphone com.todesktop.230313mzl4w4u92: granted
AudioCapture com.todesktop.230313mzl4w4u92: denied
SystemAudioRecording com.todesktop.230313mzl4w4u92: denied
```

`ai.syndai.taco` had no listed services. Those words are `tccutil`'s. They are not a mapping of `authValue=2`. A prior Microphone record for the responsible app is consistent with a preflight that does not prompt. It is not recorded here as proof that `authValue=2` means granted.

Exit 0 is also not an AudioCapture authorization. The same `tccutil` list says AudioCapture and SystemAudioRecording are denied for that responsible bundle, and the Chrome log slice had no AudioCapture request. The CLI still exited 0. A denied AudioCapture record did not become a dialog or a CLI deny.

## Allow and deny are still missing

AC1 needs an allow and a deny from `taco run` for microphone, system, and an app. These Chrome and system runs did not show a dialog and no button was pressed. That gap is open.

`tccutil reset Microphone com.apple.Terminal` printed `Successfully reset Microphone approval status for com.apple.Terminal`. The next `tccutil list -s Microphone -b com.apple.Terminal` still printed `denied`. Terminal's AudioCapture list was already `denied`. A second launch, `osascript` `tell application "Terminal" to do script` of `taco run --source default`, did not start `taco` and did not return. A dialog with Don't Allow and Allow was on screen. Its title was not read, so it is not recorded as a microphone or AudioCapture prompt. Neither button was pressed. The `osascript` process was then killed. No allow and no deny were taken.

## External input reserved for M2

`system_profiler SPAudioDataType` on this Mac listed `MacBook Pro Microphone` as the default input and `syndai-iphone-02 Microphone` as another input (transport Unknown, not default). The second name is reserved for M2. `qualified_in_m0` stays false. `EXTERNAL_INPUT.display_name` stays unset in code. This is not an M0 qualification.
