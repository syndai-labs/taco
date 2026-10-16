# Minimum OS floor and alternate launch host (2026-10-16)

Cited from `cursor/m0-ac4-os-host-b19b` tip `8ebd0fc` (PR #19). `OS_FLOOR`, `OS_FLOOR_PARTIAL`, `ALTERNATE_TERMINAL_HOST`, and `ALTERNATE_HOST_EXERCISED` on that commit are not replayed in this branch's `freeze.rs`. C2 stays open. No acceptance box is checked.

M0-AC1 and M0-AC4 stay unchecked. C2 stays open. No microphone button was pressed. No Metal load was run. This note does not re-run capture.

## Base

Branched from `cursor/m0-ac4-corners-b19b` at `f40f2f5` (PR #9).
`cursor/m0-decision-sync-b19b` at `5f4bee1` does not contain that commit and still says this Mac is not C1, so it is not the base.

## Minimum OS

Status: **Partial**.

| Claim | Result |
| --- | --- |
| Proposed floor | macOS 14.2 on Apple Silicon (ADR 0002). Not measured |
| Measured build | macOS 27.0.1 (26A434), Apple M1 Pro, 34359738368 bytes |
| Revised floor | macOS 27.0.1 (26A434) on Apple Silicon |
| Confirmed support promise | No. `OS_FLOOR_PARTIAL` is true |

Sources for the build string: [qualification-corners.md](qualification-corners.md) on `f40f2f5`; the capture-wire host table on `4c8dc27` (PR #8); the process-tap host line on `cd9a4c8` (PR #11). Those branches are not merged here. The build string matches. No second OS build appears in those records.

Unknown:

- An operator allow and an operator deny on 26A434.
- Any taco run on macOS 14.2, or on any build older than 26A434.
- Whether 26A434 is the oldest build that can run the tap. It is the floor because nothing older was measured, not because an older build failed.

Rejected: keep "macOS 14.2" as a confirmed minimum; treat 26A434 as a qualified support promise; assign 26A434 to C2.

## Alternate terminal host

Named: **launchd user agent**. Not exercised. `ALTERNATE_HOST_EXERCISED` is false.

This is the acceptance launch parent for the signed CLI when a GUI parent would take TCC responsibility. It is not a second Mac and not a TUI host. C2's chip, memory, and OS build stay unnamed. Presentation cells that run stay in Apple Terminal until a second GUI terminal is measured.

Why this name, from runs that already exist on the other branches:

PR #8 `docs/evidence/m0/capture-wire.md` (`4c8dc27`) records a microphone request with `preflight=false` and `AUTHREQ_PROMPTING`. The subject is `com.todesktop.230313mzl4w4u92`. The responsible path is `/Applications/Cursor.app/Contents/MacOS/Cursor`. The accessor was `ai.syndai.taco`. The dialog title named Cursor. Neither button was pressed. That is not an Allow and not a Deny.

PR #11 `docs/evidence/m0/process-tap.md` (`cd9a4c8`) records the same responsible process on the system and Chrome spike preflights (`kTCCServiceMicrophone`, `preflight=yes`, no `AUTHREQ_PROMPTING`). `tccutil` listed Microphone granted and AudioCapture denied for `com.todesktop.230313mzl4w4u92`. `ai.syndai.taco` had no listed services. Exit 0 is not an AudioCapture authorization. No button was pressed.

`osascript` `tell application "Terminal" to do script` of `taco run --source default` did not start `taco`. A Don't Allow / Allow dialog was on screen. Its title was not read. Neither button was pressed. That is not a completed Apple Terminal launch and not this alternate host.

`responsibility_spawnattrs_setdisclaim` was not called. That disclaim path was not measured. It is not the named host.

Rejected: Cursor's integrated terminal; an unmeasured GUI emulator; calling Apple Terminal proven from the unread `osascript` dialog; inventing a second machine and calling it the host.

Consequence: M0-AC1 still needs an allow and a deny from a parent that is not Cursor. M0-AC4 stays open. Release cells stay not run.
