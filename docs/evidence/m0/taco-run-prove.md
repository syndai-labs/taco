# `taco run` prove gate

`taco run` calls `session::prove_native` only when `TACO_ENGINES_PROVE=1`.
That is the same `engines::prove_pins` path the native cargo tests use. On
`Ok` this process prints:

```text
link_three=<n> cancel_lifecycle=<n> ac2=0 ac3=0
```

The static engines print that same line during the call. Any other value of
the variable, including unset, keeps the capture preflight and does not call
`prove_native`. A missing `--model-dir` and an empty `TACO_MODEL_DIR` refuse
with `engine_not_linked` and do not call native code. An unbound or
non-macOS build refuses the same way from `prove_native`.

M0-AC2 and M0-AC3 stay unchecked. `ac2` and `ac3` in the predicate line are
fixed at 0.

## Linux

Recorded 2026-10-16. Default `cargo test` (no `native` feature) includes
`taco_run_engines_prove_fails_closed`. It spawns:

```text
taco run --source default --from en --to zh --model-dir /tmp/taco-m0-absent
```

With `TACO_ENGINES_PROVE=1`: exit 1, stderr `engine_not_linked`, stdout has
no `link_three=` line. Without the variable, and with `=0`, stderr is
`platform_unsupported`.

`cargo test --features native` stays fail-closed on that host. The link and
cancel tests panic off macOS Apple Silicon.

## macOS

Recorded 2026-10-17 01:35 KST on this Mac. Worktree
`~/workspace/syndai/taco-m0-run-prove`, branch
`cursor/m0-taco-run-prove-b19b`, tip `af2af02`. Ad-hoc signed release `taco`,
built `--features native`. Exit 0 in 9.35s. Stdout printed
`link_three=1 cancel_lifecycle=1 ac2=0 ac3=0` twice (engine printf, then the
CLI). That is not an acceptance pass.

### Host

| Field | Value |
| --- | --- |
| Chip | Apple M1 Pro |
| OS | macOS 27.0.1, build 26A434 |
| Arch | arm64 |
| rustc | 1.99.0 (`b940084d7`, 2026-09-28) |

### Build

`TACO_LLAMA_SRC` and `TACO_WHISPER_SRC` pointed at
`~/workspace/syndai/taco-m0-metal/.cache/m0-engines`. `TACO_ORT_ROOT` pointed
at `~/workspace/syndai/taco-m0-metal/.cache/m0-onnx`. Model files were
`~/workspace/syndai/taco-m0-pins-cache`.

```text
$ cargo build --release --features native
    Finished `release` profile [optimized] target(s) in 42.61s
```

Exit 0 at 2026-10-17 01:34:38 KST.

```text
$ file target/release/taco
target/release/taco: Mach-O 64-bit executable arm64
```

38282160 bytes. `_taco_engines_prove` and `_OrtGetApiBase` are defined.
`otool -L` lists system libraries only: Accelerate, Foundation, Metal,
MetalKit, CoreFoundation, `libc++`, `libobjc`, `libSystem`. No `.dylib`,
`.so`, `.metal`, or `.metallib` sits next to the executable.

### Signature

`sh scripts/sign-debug.sh target/release/taco` (ad-hoc, identifier
`ai.syndai.taco`). Not an Apple Development or Developer ID signature.

```text
Identifier=ai.syndai.taco
Format=Mach-O thin (arm64)
Signature=adhoc
Info.plist entries=5
TeamIdentifier=not set
```

### Command

```text
TACO_ENGINES_PROVE=1 \
TACO_MODEL_DIR=$HOME/workspace/syndai/taco-m0-pins-cache \
/usr/bin/time -p ./target/release/taco run \
  --source default --from en --to zh \
  --model-dir $HOME/workspace/syndai/taco-m0-pins-cache
```

Exit 0 at 2026-10-17 01:35:28 KST. `/usr/bin/time -p` on stderr:

```text
real 9.35
user 1.37
sys 0.82
```

Stdout is 1496 bytes. Stderr is 456367 bytes of whisper and llama logs.
The process did not block on a permission dialog.

### Stdout

```text
taco_engines_linked gpu_offload=1
digest role=vad ok=1 sha256=1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3
digest role=asr ok=1 sha256=1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b
digest role=mt ok=1 sha256=dc5f44fcf1fa496ee7ad725982c0c8c553a4de00259b53af84c4b89fb0c06699
vad=fail
vad_ort_session=ok version=1.31.0 inputs=3 outputs=2 ep=CPU
vad_ort_in name=input type=1 rank=2 dims=-1,-1
vad_ort_in name=state type=1 rank=3 dims=2,-1,128
vad_ort_in name=sr type=7 rank=0 dims=
vad_ort_out name=output type=1 rank=2 dims=-1,1
vad_ort_out name=stateN type=1 rank=3 dims=-1,-1,-1
vad_ort_frame=0 samples=512 sr=16000 prob=0.000592 state_l2=8.108584
vad_ort_frame=1 samples=512 sr=16000 prob=0.000528 state_l2=11.574153
vad_ort_frame=2 samples=512 sr=16000 prob=0.000592 state_l2=8.108584
vad_ort_reset=1 match=1 carried_state=1 window=512 state=2x1x128
vad_ort=ok device=cpu
gpu_offload=1
gpu_device=MTL0
asr=ok use_gpu=1 segments=1 text_bytes=21
whisper_cancel_decode=aborted rc=-8 hits=2 join_ms=178 reserve_ms=2000 within_reserve=1
whisper_cancel_init=aborted join_ms=214 poisoned=1 reserve_ms=2000 within_reserve=1
cancel_init=aborted join_ms=422 progress_milli=0 reserve_ms=2000 within_reserve=1
mt_load=0 n_gpu_layers=-1 offloaded=33/33
mt=ok text_bytes=34
cancel_decode=aborted rc=2 steps=2 join_ms=12 reserve_ms=2000 within_reserve=1
link_three=1 cancel_lifecycle=1 ac2=0 ac3=0
result=ran
teardown=returned
link_three=1 cancel_lifecycle=1 ac2=0 ac3=0
```

### Stderr lines that match the stdout

```text
ggml_metal_library_init: using embedded metal library
whisper_init_with_params_no_state: failed to load model
load_tensors: offloaded 33/33 layers to GPU
llama_model_load_from_file_impl: cancelled model load
load_tensors: offloaded 33/33 layers to GPU
llama_decode: failed to decode, ret = 2
```

The failed whisper load and `llama_decode` ret 2 are the cancel path. Joins
on stdout are inside the 2000 ms reserve. VAD execution provider is CPU.
`vad=fail` is whisper VAD on the ONNX pin. One `prove_native` call. No
repeated init/teardown residency measurement.

## Still unchecked

- [ ] **M0-AC2:** This `taco run` loaded the three pins, ran ASR and MT on MTL0, and ran Silero through static ORT on the CPU execution provider. It printed `ac2=0`. Catalog residency was not measured.
- [ ] **M0-AC3:** The four cancel joins were inside 2000 ms and `teardown=returned` on this one pass. It printed `ac3=0`. Late-completion, replacement-worker, and residency checks were not recorded.
