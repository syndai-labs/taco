# Native impl Info.plist bind (2026-10-16)

Measured in worktree `~/workspace/syndai/taco-m0-plist` at `234c26a`
(`cursor/m0-native-impl-b19b`). Acceptance checkboxes stay open.
No TCC allow/deny was shown. No Metal load was run.

## Host

| Field | Value |
| --- | --- |
| Chip | Apple M1 Pro |
| Memory | 34359738368 bytes (32 GiB) |
| OS | macOS 27.0.1, build 26A434 |
| Arch | arm64 |
| rustc | 1.99.0 (`b940084d7`, 2026-09-28) |
| When | 2026-10-16 23:40 KST |

## Bind

`cargo build` on this commit exited 0. `./scripts/sign-debug.sh` then exited 0.

The linker signature is still unbound. A clean rebuild at parent `964e384`
(`cargo clean && cargo build`, exit 0) reported:

```text
Identifier=taco-a8e556d459e2954c
Signature=adhoc
flags=0x20002(adhoc,linker-signed)
Info.plist=not bound
TeamIdentifier=not set
```

`scripts/sign-debug.sh` re-signs with `codesign --force -s - --identifier ai.syndai.taco`.
On `234c26a` that produced:

```text
Identifier=ai.syndai.taco
Format=Mach-O thin (arm64)
CodeDirectory v=20400 size=8007 flags=0x2(adhoc) hashes=245+2 location=embedded
CDHash=53ffae8077c8f8326bb798c38cee3acffd73a099
Signature=adhoc
Info.plist entries=5
TeamIdentifier=not set
ok: plist bound for /Users/jwu/workspace/syndai/taco-m0-plist/target/debug/taco
```

The same ad-hoc re-sign on `4969bf6` already reported `Info.plist entries=5`.
`-Xlinker -sectcreate` in `964e384` did not change the linker-signed line.

## Capture

The binary above (ad-hoc, plist bound) was invoked once per source. Each
printed this stderr and exited 1:

```text
error models_unpinned
```

```text
$ ./target/debug/taco run --source default --from en --to zh
$ ./target/debug/taco run --source system --from en --to zh
$ ./target/debug/taco run --source app:com.apple.Terminal --from en --to zh
```

No TCC prompt appeared. Cleanup of a tap was not reached. Microphone, system
output, and app tap are not demonstrated. Pins are still unset, so
`models_unpinned` is the stop.
