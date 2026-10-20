# Engine residency (2026-10-17)

A second attempt did not enter the band. The vmtrace and the reverted
teardown experiments are in [engines-residency-retry.md](engines-residency-retry.md).
The holder is named in [engines-residency-rootcause.md](engines-residency-rootcause.md).
M0-AC2 and M0-AC3 stay unchecked.

One hundred `session::prove_native` cycles in one process. The catalog metric
is `task_vm_info.phys_footprint` after teardown. Cycle 100 is 252346632 bytes
above the cycle-1 teardown. The tolerance is 33554432 bytes (32 MiB). The
comparison fails. M0-AC2 and M0-AC3 stay unchecked.

## Host

| Field | Value |
| --- | --- |
| Chip | Apple M1 Pro |
| RAM | 34359738368 |
| OS | macOS 27.0.1, build 26A434 |
| rustc | 1.99.0 (`b940084d7`, 2026-09-28) |
| When | 2026-10-17 01:24–01:28 KST |
| PID | 29510 for all 100 cycles |
| Binary | `target/debug/deps/t_m0_residency-19dd4402484d0b01` (Mach-O arm64, debug Rust, Release cmake engines) |

Not a qualified corner. `proves_c1` stays false.

## Command

```text
TACO_LLAMA_SRC=~/workspace/syndai/taco-m0-metal/.cache/m0-engines/llama.cpp \
TACO_WHISPER_SRC=~/workspace/syndai/taco-m0-metal/.cache/m0-engines/whisper.cpp \
TACO_ORT_ROOT=~/workspace/syndai/taco-m0-metal/.cache/m0-onnx \
TACO_MODEL_DIR=~/workspace/syndai/taco-m0-pins-cache \
TACO_RESIDENCY_CYCLES=100 \
cargo test --features native --test t_m0_residency -- --nocapture --test-threads=1
```

Exit 0. `prove_predicates=1` on every cycle: `link_three=1`, `vad_ort=1`,
`gpu_offload=1`, `teardown=1`, `gpu_device=MTL0`. Each cycle printed
`ep=CPU`, `ac2=0`, and `ac3=0`. Sum of per-cycle `elapsed_ms` is 209817.
No capture was started.

## Catalog rule

From `docs/roadmap/acceptance.md`: after 100 in-process start/stop cycles,
`phys_footprint` after cycle 100 ≤ footprint after cycle 1 teardown + 32 MiB.
RSS and allocator counters sit beside that number and do not replace it.
`T-M0-LINK-THREE-MODELS` uses the same tolerance against the post-first-teardown
baseline. A new process per cycle does not count. This run is one process.

| | bytes |
| --- | --- |
| process start, before any prove | 3179000 |
| after cycle 1 teardown | 82625424 |
| after cycle 100 teardown | 334972056 |
| delta cycle 100 − cycle 1 | 252346632 |
| tolerance | 33554432 |
| cycle 1 + tolerance | 116179856 |
| cycle 100 − (cycle 1 + tolerance) | 218792200 |
| within tolerance | 0 |
| max after-teardown | 334972056 (cycle 100) |
| min after-teardown | 82625424 (cycle 1) |
| max sampled `phys_footprint` during a prove | 904627760 (cycle 97) |
| proposed ceiling | 6442450944 |

The 6 GiB ceiling is the proposed qualification ceiling for a measured window
on a corner. This host is not that corner. The peak is reported, not a pass.

## After-teardown `phys_footprint`

Same PID. `resident_size` is RSS from the same `task_vm_info` read.
`peak_phys` is the max `phys_footprint` sampled every 50 ms during that prove,
including the loaded models. The tolerance uses the after-teardown column only.

| Cycle | phys_footprint | delta vs cycle 1 | resident_size | peak_phys | elapsed_ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1 | 82625424 | 0 | 189923328 | 755041720 | 7020 |
| 2 | 172229688 | 89604264 | 279445504 | 739280360 | 2059 |
| 10 | 185287736 | 102662312 | 292503552 | 754910672 | 2049 |
| 18 | 190202936 | 107577512 | 297418752 | 758908392 | 2110 |
| 19 | 99566600 | 16941176 | 219004928 | 759891408 | 2067 |
| 20 | 179225656 | 96600232 | 298631168 | 748881360 | 2040 |
| 25 | 332514480 | 249889056 | 458047488 | 902170184 | 2043 |
| 29 | 243188864 | 160563440 | 378421248 | 812844568 | 2022 |
| 30 | 322847920 | 240222496 | 458047488 | 892503624 | 2031 |
| 50 | 322733208 | 240107784 | 458014720 | 892388912 | 2036 |
| 60 | 332629144 | 250003720 | 467943424 | 902284848 | 2058 |
| 80 | 332842136 | 250216712 | 468156416 | 902497840 | 2031 |
| 96 | 334955672 | 252330248 | 470269952 | 903661128 | 2082 |
| 100 | 334972056 | 252346632 | 470286336 | 904627760 | 2000 |

Cycle 2 already exceeds the tolerance (89604264 > 33554432). Cycle 19 is the
only later teardown back inside the band (16941176 ≤ 33554432). Cycle 20 leaves
it again. Five teardowns are below the previous teardown: 19, 24, 29, 34, 38.
The drops are 90636336, 6193152, 89325616, 131096, and 32768 bytes. From cycle
97 through 100 the after-teardown value stays 334972056.

All 100 after-teardown `phys_footprint` values, cycle 1 through 100:

```text
82625424 172229688 179962936 181257272 184615992 184697912 184992824 185222200
185254968 185287736 187745336 188056632 188089400 188089400 188105784 188138552
188171320 190202936 99566600 179225656 259032168 259032168 259032168 252839016
332514480 332514480 332514480 332514480 243188864 322847920 322847920 322847920
322847920 322716824 322716824 322716824 322716824 322684056 322684056 322684056
322700440 322700440 322700440 322700440 322700440 322700440 322716824 322716824
322716824 322733208 322733208 322733208 330957976 330990744 330990744 332629144
332629144 332629144 332629144 332629144 332629144 332629144 332629144 332629144
332645528 332842136 332842136 332842136 332842136 332842136 332842136 332842136
332842136 332842136 332842136 332842136 332842136 332842136 332842136 332842136
332842136 332842136 332842136 332842136 332842136 332858520 332858520 332858520
332858520 332858520 332858520 332858520 334939288 334939288 334939288 334955672
334972056 334972056 334972056 334972056
```

RSS after cycle 1 is 189923328. RSS after cycle 100 is 470286336. Delta
280363008. Default-zone `malloc_size_in_use` is 36577264 after cycle 1 and
36520176 after cycle 100. `max_size_in_use` read 0 on every sample, including
process start, so that counter did not report a high-water mark.

## What was not measured

`MTLDevice.currentAllocatedSize` was not read. The prove API does not return
the ggml Metal device. `task_vm_info.device` was 0 on every sample.
`ledger_tag_graphics_footprint` was 0 before the first prove and 4390912 after
every teardown. It did not grow. That ledger is not a Metal buffer inventory.
The catalog comparison above uses `phys_footprint` only.

## Stop / partial

Partial. The 100-cycle comparison stands: 252346632 bytes over the cycle-1
teardown, against a tolerance of 33554432. A later teardown fix was tried and
did not enter that band. M0-AC2 and M0-AC3 stay unchecked.

Phase samples in one 8-cycle process, same prove path, before that attempt.
`phys_footprint` bytes:

| Cycle | after_vad_ort | after_whisper_free | after_mt_cancel_init | after_backend_free |
| --- | ---: | ---: | ---: | ---: |
| 1 | 64111336 | 174949408 | 75907984 | 82658216 |
| 2 | 84181928 | 170820712 | 173982824 | 178685032 |

Cycle 1 retains 110838072 bytes across `whisper_free`, then the aborted MT
load drops the footprint. Cycle 2 retains 86638784 bytes across `whisper_free`
and the aborted MT load does not give them back. `llama_backend_free` did not
change the footprint on either cycle (`after_free_mt` equaled
`after_backend_free`). `before_load` on cycle 1 was 45204200.
`after_vad_whisper` was 45220584, so the failed whisper VAD load is not the
retainer.

On cycle 1, `virtual_size` went from 500829667328 after ORT to 501346025472
after `whisper_free` (+516358144). By cycle 8 `after_backend_free` it was
still 501400977408, with `region_count` 284. `internal` tracked
`phys_footprint`. `compressed` was 0.

The flush that was not kept: after `endResidency`, submit a 16-byte private
blit and `waitUntilCompleted` on that device queue. Six teardowns in one
process: after cycle 1, 82445200; after cycle 6, 181257272; delta 98812072.
Still outside 33554432. The patch was reverted and is not in this tree.

Consequence: repeated `prove_native` teardown is not a bounded-residency
result. Do not treat this run as meeting `T-M0-LINK-THREE-MODELS` or the
acceptance.md 100-cycle rule. Do not start M1 on a claim that engine teardown
returns to the cycle-1 baseline. The pinned ggml has no safe device release;
see [metal-device-teardown.md](metal-device-teardown.md). #55 tip `0829a70`
names the holder. #59 tip `ed9ee10` locks the AC2/AC3 checkbox. Neither box
below is a pass.

## Still open

- [ ] **M0-AC2:** Pins loaded in process on MTL0, and the cycle series above was recorded. Silero's execution provider was CPU (`ep=CPU`). The measured process is the residency test, not `taco run`. `phys_footprint` did not stay within 33554432 of the cycle-1 teardown. The engine printed `ac2=0`.
- [ ] **M0-AC3:** Each prove joined its cancels and set `teardown=1`. Late-completion and replacement-worker checks were not added. Resident footprint did not return to the post-first-teardown baseline. The engine printed `ac3=0`.
