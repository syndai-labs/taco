# AudioCapture prompt prep

Cited from `cursor/m0-audiocapture-prep-b19b` tip `d78e7f8` (PR #44). This branch does not replay capture source. No acceptance box is checked. No tap was started. No dialog was clicked. No `tccutil reset` was run.

Host recorded on that tip: macOS 27.0.1 (26A434), arm64. That note says this machine is not C1 and it is not a named C2. It does not name a chip. Parent tip `cd9a4c8`.

At 01:55 KST pid 76134 was `taco run --source default --from en --to zh` from the tap worktree. `codesign -dv` on that binary: `Identifier=ai.syndai.taco`, ad-hoc, `Info.plist entries=5`. That pid was gone at 01:59:48 KST. Pid 82331 was already the same command from the same binary and was still running while the note was written. Neither pid was signaled. A pid exit is not an Allow and it is not a Deny. The source beside that binary waits up to `TACO_CAPTURE_SPIKE_OPEN_MS` milliseconds (default `180000` on that tip) and does not press Allow or Deny.

`TACO_CAPTURE_SPIKE=1` with `--source system`, `app:<bundle-id>`, or `pid:<pid>` creates a private process tap and a private aggregate, then Rust drops them. The aggregate dictionary sets `kAudioAggregateDeviceTapAutoStartKey` to 0. This path does not call `AudioDeviceStart` on that aggregate. `AudioDeviceStart` in the same file is the microphone input proc. The prompt Apple's tap sample ties to the first start of an aggregate is the `AudioDeviceStart` this path does not call. Recorded Chrome `app:` and `pid:` runs on this wiring exited 0 with no AudioCapture request and no dialog. Exit 0 is not an Allow.

`tccutil` reads copied at 01:59 KST:

| Query | Printed |
| --- | --- |
| `tccutil list -s AudioCapture` | `com.google.AIEdgeForesight` only |
| `tccutil list -s AudioCapture -b ai.syndai.taco` | `denied` |
| `tccutil list -b ai.syndai.taco` | empty |
| `tccutil list -s Microphone` | no `ai.syndai.taco` row |
| `tccutil list -s Microphone -b ai.syndai.taco` | `denied` |

`denied` from the pair query, with the bundle absent from `tccutil list -s`, is the same output as a bundle that has never been asked. It is not an operator Deny.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
