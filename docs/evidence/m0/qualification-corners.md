# Qualification corners (2026-10-16)

C1 is named from measurements already in
[macos-spike-results.md](macos-spike-results.md) and
[native-impl.md](native-impl.md). No second machine was measured.
C2 is open. Neither corner is qualified. M0-AC4 stays unchecked.

`C1_NAMED` in `src/freeze.rs` is the freeze marker for that name.
`Corner::qualified` stays false on both corners.

## C1

Named machine: Apple M1 Pro, 34359738368 bytes (32 GiB). OS build: macOS 27.0.1 (26A434).

| Field | Value | Source |
| --- | --- | --- |
| Chip | Apple M1 Pro | Host table in macos-spike-results.md, repeated in native-impl.md |
| Memory | 34359738368 bytes (32 GiB) | same |
| OS | macOS 27.0.1 (26A434) | same |
| Arch | arm64 (`aarch64-apple-darwin`) | macos-spike-results.md |
| Power | not recorded | — |

Still missing before a support claim: a tap/permission/packaging run and thermal
notes on this OS build.

The spike sentence "this host is not C1" rejected using this machine as proof
of the unmeasured proposal "M1 class, 16 GiB, macOS 14.2". This record revises
the name. It does not claim that proposal was measured.

## C2

Open. Evidence has one Mac. Still missing:

- a second Apple Silicon chip name
- that machine's memory in bytes
- that machine's macOS product version and build

"Latest macOS" is not a name. This host is not C2. Build 26A434 is not C2's build.

## Decision

Chosen: name C1 as the only measured host and leave C2 open.
Rejected: keep "proposed starting point" as the C1 name; treat this host as
proof of a 16 GiB macOS 14.2 floor; assign build 26A434 to C2 without a second
machine.
Consequence: M0-AC4 stays open. The support claim, M1, and M5 stay blocked.
The 6 GiB memory ceiling is unchanged. The 8 GiB revision reserved for an
unmeasured 16 GiB machine is not available from this 32 GiB corner.

## Minimum OS floor

Partial. Revised floor is macOS 27.0.1 (26A434), the only measured build.
macOS 14.2 was not measured and is not the floor. This is not a support
promise. Detail: [os-floor.md](os-floor.md).

## Alternate launch host

Named: launchd user agent. Not exercised. Not a second machine. Cursor.app
(`com.todesktop.230313mzl4w4u92`) is rejected as the TCC parent. C2 stays open.
