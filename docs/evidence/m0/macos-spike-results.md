# M0 macOS spike results (2026-10-16)

Measured on the contract-freeze tree `828bd48` (`cursor/m0-contract-freeze-b19b`).
No model digest was assigned. No Metal load was run. Acceptance checkboxes stay open.

This host is a development machine. It is not C1 and it is not a named C2.

## Host

| Field | Value |
| --- | --- |
| Chip | Apple M1 Pro |
| Memory | 34359738368 bytes (32 GiB) |
| OS | macOS 27.0.1, build 26A434 |
| Arch | arm64 (`aarch64-apple-darwin`) |
| rustc | 1.99.0 (`b940084d7`, 2026-09-28) |
| Metal compiler | Apple metal version 32023.921 (`metalfe-32023.921.6`), target `air64-apple-darwin27.0.0` |
| Metal path | `/var/run/com.apple.security.cryptexd/mnt/com.apple.MobileAsset.MetalToolchain-v27.1.266.1.9bqNNJ/Metal.xctoolchain/usr/bin/metal` |

C1 remains "M1 class, 16 GiB, macOS 14.2" and is not this machine. C2 still has no OS build. Neither `qualified` flag was set.

## T-M0-ONE-MACHO

`cargo build` and `cargo build --release` succeeded before any later tree edits.
`target/debug/taco` and `target/release/taco` are each one thin arm64 Mach-O.
The only dynamic dependency is `/usr/lib/libSystem.B.dylib`.

```text
$ file target/debug/taco
target/debug/taco: Mach-O 64-bit executable arm64

$ otool -L target/debug/taco
target/debug/taco:
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1359.0.0)

$ otool -h target/debug/taco
Mach header
      magic  cputype cpusubtype  caps    filetype ncmds sizeofcmds      flags
 0xfeedfacf 16777228          0  0x00           2    17       2104 0x00a00085
```

`cputype` 16777228 is `CPU_TYPE_ARM64`. `filetype` 2 is `MH_EXECUTE`.

Linker signature of `target/debug/taco` (`codesign -dv --verbose=4`):

```text
Identifier=taco-3270e472ce86c413
Format=Mach-O thin (arm64)
CodeDirectory v=20400 size=31406 flags=0x20002(adhoc,linker-signed)
CDHash=32e242d4b381092b0c29f9a9df0edbaa2e6b14af
Signature=adhoc
Info.plist=not bound
TeamIdentifier=not set
```

`target/release/taco` is the same shape: thin arm64, only `libSystem.B.dylib`,
`Signature=adhoc`, `flags=0x20002(adhoc,linker-signed)`, `Info.plist=not bound`,
`TeamIdentifier=not set`, `CDHash=ae9d9a4928a61a5aae98894db2ab5128344e5924`.
`strings` on that binary contains `NSMicrophoneUsageDescription` and
`NSAudioCaptureUsageDescription`. Those bytes come from `include_bytes!` of
`assets/Info.plist`. They are not a bound `__TEXT,__info_plist`.

```text
$ cargo test --test t_m0_one_macho -- --nocapture
test t_m0_one_macho_inspector_rejects_bad_artifacts ... ok
test t_m0_one_macho_manual_script_is_not_run ... ok
thread 't_m0_one_macho_host_binary_is_not_a_pass' panicked at tests/t_m0_one_macho.rs:57:5:
must not claim Mach-O pass: Pass
test result: FAILED. 2 passed; 1 failed
```

`inspect_path` returned `Pass` for the debug binary: one arm64 execute, only a
system dylib, both usage keys present in the file, no sidecar with a watched
extension next to the binary. The harness test failed because it refuses that
claim. The manual script `scripts/qual/T-M0-ONE-MACHO` still exits 2 with
`verdict=not_run`. This is not an M0-AC2 or M0-AC3 pass. The plist is not bound
and the signature is the linker ad-hoc signature.

## T-M0-SPIKE-SOURCES

Keychain has one codesigning identity:

```text
36DE9F66E808688C3EBE00A33C23EA6197889104 "Apple Development: ops@syndai.com (CT93T5LALG)"
     1 valid identities found
```

The contract binary never opens a capture API. `preflight` returns
`models_unpinned` while pins are unset, before `start_capture`.

Unmodified `target/debug/taco`, then an ad-hoc re-sign of a copy
(`codesign --force --sign - --identifier ai.syndai.taco`), then a Development
re-sign of a copy (`codesign --force --sign "Apple Development: ops@syndai.com (CT93T5LALG)" --identifier ai.syndai.taco`).
Each of these three commands was run on each binary:

```text
taco run --source default --from en --to zh
taco run --source system --from en --to zh
taco run --source app:com.apple.Terminal --from en --to zh
```

Every invocation printed this stderr and exited 1:

```text
error models_unpinned
```

Ad-hoc re-sign (`CDHash=4d1f2ff7b6820d2c741c111892b2332b0f861b43`):

```text
Identifier=ai.syndai.taco
flags=0x2(adhoc)
Signature=adhoc
Info.plist=not bound
TeamIdentifier=not set
```

Apple Development re-sign (`CDHash=f5455d873156271820035ef4fe0ac7bbe2e8be07`, signed 2026-10-16 23:16 local):

```text
Identifier=ai.syndai.taco
flags=0x0(none)
Authority=Apple Development: ops@syndai.com (CT93T5LALG)
Authority=Apple Worldwide Developer Relations Certification Authority
Authority=Apple Root CA
TeamIdentifier=3LQ94P5794
Info.plist=not bound
```

No TCC allow/deny prompt was shown. Cleanup of a tap was not reached. The
manual script still records `verdict=not_run`. Microphone, system output, and
app tap are not demonstrated. No external input device was reserved.

Spike stopped after that signed rerun. Process-tap code was not added.

## T-M0-LINK-THREE-MODELS and T-M0-CANCEL-LIFECYCLE

`taco models list` on the measured binary:

```text
vad silero sha256=TBD file=TBD
asr whisper-small-multilingual sha256=TBD file=TBD
mt hy-mt2-1.8b-gguf sha256=TBD file=TBD
```

`PINS_MEASURED` is false. No Silero, Whisper small, or Hy-MT2 file was pinned.
Spotlight `*.gguf` hits on this Mac are other families (Gemma and Qwen under
other apps). They were not loaded.

The Metal compiler listed above is installed. It was not used to load a model
or to compile a shader into `taco`.

```text
$ cargo test --features native -- --nocapture
```

Stopped in `charter_guardrails` before the native tests:

```text
thread 'session_stays_local_and_commits_are_immutable' panicked at tests/charter_guardrails.rs:98:5:
assertion failed: !taco::session::platform_supported()
test result: FAILED. 6 passed; 1 failed
```

`platform_supported()` is true on `aarch64-apple-darwin`. The assertion was
written for a host where that function is false.

Invoked directly:

```text
$ cargo test --features native --test t_m0_cancel_lifecycle -- --nocapture
thread 't_m0_cancel_lifecycle' panicked at tests/t_m0_cancel_lifecycle.rs:12:5:
T-M0-CANCEL-LIFECYCLE not executed: no Metal worker was cancelled or joined inside the shutdown reserve
test result: FAILED. 0 passed; 1 failed
```

`T-M0-LINK-THREE-MODELS` did not execute. `cargo test --features native` exited
on the guardrail failure, and a follow-up that named both native tests exited
on the cancel panic before the link binary ran. The test source on `828bd48`
panics after the macOS assert because the digests are TBD and Metal execution
was not observed. That panic text was not produced by a test process in this
run. No in-process Metal load, GPU-backend readback, or cancellation join was
attempted. `EXPERIMENT.ran` stays false. No quantization, instruction winner,
or sampling arm was selected.

## What this does not accept

- M0-AC1 through M0-AC6 stay open.
- M1 and M5 stay blocked.
- A newer 32 GiB Mac on macOS 27.0.1 does not qualify C1.
- Inspector `Pass` plus unbound Info.plist is not a signed distribution spike.
- `models_unpinned` is not an allow/deny/cleanup record.
