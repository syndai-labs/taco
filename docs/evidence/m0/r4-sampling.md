# R4 quantization and sampling (2026-10-16)

Cited from `cursor/m0-r4-sampling-b19b` tip `1a52b8d` (PR #17). That note reads the metal probe through `bc194a3`. The decision register also cites PR #12 tip `a8b2ebc`: the same Q4_K_M load and `mt=ok text_bytes=34`, plus Silero on static ORT CPU (`link_three=1`). That later run is not a quantization or sampling selection. The Silero paragraph below is the pre-ORT reading and does not override R3. PR #18 tip `7ee17c8` later records `whisper-small-weights` as MIT; the license cell below is the `c04ab04` reading.

Stop. M0 accepts no quantization arm, no instruction winner, and no sampling arm.

This note was written on Linux. It did not run Metal, llama.cpp, or whisper.cpp.
Figures below are copied from the Mac notes named in the tip table. The header
paragraph of [metal-link.md](metal-link.md) still says the Hy-MT2 file was not
loaded. That sentence sits above the pinned-load section and was left in place
when the load was recorded. The pinned-load section is the measurement.
`bc194a3` keeps both the header sentence and the load.

`EXPERIMENT.ran` stays false. `selected_quantization`, `selected_instruction`,
`selected_sampling`, and `SELECTED_SAMPLING` stay empty. `COMPARISON_WINNER`
stays empty. `quantizations_compared` stays 0. The cap is 2.

## Tips

| Tip | Branch | Role in this Stop |
| --- | --- | --- |
| `c04ab04` | `cursor/m0-license-inventory-b19b` | Base. License inventory and the previous R4 row. |
| `5e9649c` | `cursor/m0-pin-apply-b19b` | Measured digests now in `freeze::PINS`. Ancestor of the base. |
| `c0b2b22` | `cursor/m0-metal-link-b19b` | Copy already in this tree. First recorded Q4_K_M load and 34-byte smoke. |
| `bc194a3` | `cursor/m0-metal-link-b19b` | Later probe tip. Same `mt=ok text_bytes=34`. Silero still fails magic. |
| `f40f2f5` | `cursor/m0-ac4-corners-b19b` | Host note. Both corners unqualified. Not copied into `freeze::CORNERS`. |
| `5f4bee1` | `cursor/m0-decision-sync-b19b` | Sibling register. Its R4 row matched `c04ab04` before this Stop. That tip has no license inventory, so it is not the base. |

## Hy-MT2

One main-repo file is pinned.

| Field | Value |
| --- | --- |
| Filename | `Hy-MT2-1.8B-Q4_K_M.gguf` |
| SHA-256 | `dc5f44fcf1fa496ee7ad725982c0c8c553a4de00259b53af84c4b89fb0c06699` |
| Bytes | 1133080448 |
| Header, from the probe note | GGUF v3, 354 tensors, `general.architecture=hunyuan-dense`, `general.file_type=15`, F32 129, Q4_K 192, Q6_K 33 |
| Load, from the probe note | `n_gpu_layers=-1`, 33/33 layers on GPU, `MTL0_Mapped` 1075.74 MiB, `CPU_Mapped` 193.57 MiB |
| Engine commit in that note | llama.cpp `2a667032dc788c3a071d538e0b572e45a3f12731` |
| License at `c04ab04` | Apache-2.0, `Copyright (C) 2026 Tencent. All rights reserved.`, GGUF `LICENSE.txt` revision `a0c709d9fac510f2c807aa3af52872340dc37a4a` |

`file_type=15` with that F32 / Q4_K / Q6_K census is the mix named by the
filename. A second file from `tencent/Hy-MT2-1.8B-GGUF` was not fetched.
The 1.25-bit and 2-bit repositories were not opened. Publisher PR 22836 stayed
unmerged and was not required for this load. The Apache notice does not score
quality, latency, or memory.

## Whisper small

| Field | Value |
| --- | --- |
| Filename | `ggml-small.bin` |
| SHA-256 | `1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b` |
| Bytes | 487601967 |
| Load, from the probe note | ggml type small, `use gpu = 1`, `MTL0 total size = 487.01 MB` |
| Call, from the probe note | `whisper_full` returned 0 on one second of a 440 Hz tone, language `en`, translate off, VAD inside that call off, `text_bytes=21` |
| Engine commit in that note | whisper.cpp `0c7be696371069e0baa2515ed7aa08d5a5473aa6` |
| License at `c04ab04` | `whisper-small-weights` SPDX and notice stay unset |

`freeze::PINS` has this one ASR file. No Whisper temperature arm is defined
next to the Hy-MT2 sampling arms. The unset weight license leaves the file
pinned and blocks treating it as a distributable selection.

## Sampling and instruction

`SAMPLING_ARMS` still lists two unused arms.

| Arm | Code | What ran |
| --- | --- | --- |
| `publisher` | `temperature_milli` 700 | Nothing. ADR 0004 also names top_p 0.6, top_k 20, and repetition penalty 1.05. None of those were executed. |
| `seeded` | `temperature_milli` 0 | Nothing on a corpus. |

The probe decoded 8 tokens with a greedy sampler and wrote 34 bytes. Stdout at
`c0b2b22` and again in the `bc194a3` teardown run is `mt=ok text_bytes=34`.
The card's 4096-token document setting was not used. The live token cap was
not measured.

`BASELINE_INSTRUCTION` remains the publisher English template. The Chinese
template was not compared. `language::PAIRS` is still `en→zh` and `zh→en`.
The development corpus slot has `natural_utterances = 0` and `sealed = false`.
Held-out labels stay closed.

## Silero

Silero is not a quant arm and not a sampling arm. The pin is `silero_vad.onnx`,
sha256 `1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`,
2327524 bytes. `whisper_vad_init_with_params` reports `invalid model data (bad magic)`
in the `c0b2b22` copy and again at `bc194a3`. The frame loop did not run.
R3 stays Stop. The MIT notice on the weights (`Copyright (c) 2020-present Silero Team`)
does not add a sampler.

## Corner

The probe host in metal-link.md is Apple M1 Pro, 34359738368 bytes, macOS 27.0.1
(26A434). `f40f2f5` names that host and leaves both corners unqualified.
`freeze::CORNERS` on this branch is unchanged: C1 is still "Apple Silicon M1
class, 16 GiB unified memory, AC power" on macOS 14.2, `qualified` false, and
C2 has no OS build. R6 was not revised in this change. Latency, memory, and
raw-quality gates were not measured on either naming.

## Still open

M0-AC4 stays open. M0-AC1, M0-AC2, M0-AC3, M0-AC5, and M0-AC6 are unchanged.
The acceptance plan's quantization and sampling name is this Stop.
