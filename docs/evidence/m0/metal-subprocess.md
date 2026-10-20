# Metal subprocess (2026-10-20)

Stop. Not a pass. M0-AC2 and M0-AC3 stay unchecked.

This stacks on #70 tip `e06ea0e`, which stacks on #63 tip `232ea78` and #55 tip `0829a70`. It asks whether a helper process that owns the Metal device, or the ggml backend, could return `phys_footprint` between prove cycles without violating single Mach-O, in-process Metal, or LINK-THREE. There is no charter-legal path. This host is Linux. No connected self-hosted workers. No Mac `phys_footprint` was recorded. Nothing here is a Metal run. No prototype was built. Linux must not fake one.

The charter test is `metal_subprocess_stop_cites_55_63_and_70`. It is not `ac2_ac3_cannot_flip_while_stop_shas_stand`. It is not `metal_refcount_stop_cites_63_and_55`.

## Why a helper does not close the residual

#55 names the holder: the process-lifetime Metal device in `ggml_backend_metal_reg`. `phys_footprint` is the task's. A thread, a dispatch queue, or a second `MTLDevice` beside that registry does not unload it.

#63 and #70 name why an in-process free is not the patch. Pinned llama.cpp `2a667032dc788c3a071d538e0b572e45a3f12731`. `ggml_metal_device_get` only appends. Cycle 2 still uses the cycle-1 pointer. `ggml_metal_device_get_memory` reads `mtl_device` with no nil check. `product::g_asr` and `product::g_mt` share the device. Freeing it from taco is a use-after-free.

A helper drops the parent's footprint only by moving that device out of the parent. The parent then does not run Metal. That is the subprocess the charter already fails. A child `MTLDevice` does not nil the parent's `ggml_backend_metal_reg`. The parent creates its own device on first use.

## Charter clauses

| Gate | Clause the helper breaks |
| --- | --- |
| Single Mach-O | Charter form is one Rust single binary, with no daemon and no external inference service. T-M0-ONE-MACHO: one Mach-O, and no runtime helper required to launch. ADR 0004: "Two dylibs on disk and a helper process are both failures." |
| In-process Metal | ADR 0004: v0.1 runs in process, on Metal. "A build that only works through a helper CLI or only on CPU fails M0." M0-AC2: pinned artifacts, in process, with observed Metal GPU execution. A subprocess CLI does not pass. The agent contract refuses a `whisper-cli` subprocess and says a patch is not permission to fork the architecture into a helper process. |
| LINK-THREE | T-M0-LINK-THREE-MODELS loads VAD, multilingual Whisper small, and Hy-MT2 in one process. Fail on a subprocess. The cycle sentence is the post-first-teardown baseline of that same process, inside 33554432. R2: fix linkage inside one binary; no helper process, sidecar dylib, or CPU-only fallback. |

These are the same failure, not a narrower legal shape:

- `whisper-cli`, `llama-completion`, or `llama-server`.
- A second `[[bin]]` or an `add_executable` Metal helper.
- `posix_spawn` or XPC of this same Mach-O, child owns ggml, parent forwards audio.
- Re-exec between `prove_native` cycles. Exiting zeroes the footprint. That is a new task, not the post-first-teardown baseline. Do not start a new process per cycle.

## What this tree does not do

No patch under `native/patches/`. CMake has no helper executable. `Cargo.toml` stays one `[[bin]]`. `taco_runtime.cpp` does not spawn and does not call `ggml_metal_device_free`. The prove line stays `ac2=0 ac3=0`.

Upstream still owes the in-process rebuild #70 described. A helper is not that function. Changing the charter to allow one is a charter decision, outside this Stop.

## Still open

- [ ] **M0-AC2:** Unchecked. #55 tip `0829a70`, #63 tip `232ea78`, and #70 tip `e06ea0e` stand. A helper process is not a pass. The engine prints `ac2=0`.
- [ ] **M0-AC3:** Unchecked. One Mach-O, in-process Metal, and LINK-THREE still stand. No Mac `phys_footprint` was recorded. The engine prints `ac3=0`.
