# Silero VAD pin stays (stacked on #57 `88eda06`)

Stop. The VAD pin does not change. M0-AC2 stays unchecked.

Linux. No CoreML, no `MLComputePlan`, no Metal. The files below were
hashed and parsed. None was executed on a CoreML device. This note does
not print a device other than the fail-closed CPU result already
recorded for this host.

## Pin

`silero_vad.onnx`, sha256
`1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`,
2327524 bytes. `sha256sum` of the current
`snakers4/silero-vad` master file matches `freeze::PINS` and
[pin-measurement.md](pin-measurement.md). `scripts/measure-pins.sh`
still fetches that URL. The pin stays that digest.

## Builder set

ORT 1.31.0 `coreml::GetOpBuilders()` (`op_builder_factory.cc` at tag
`v1.31.0`) does not register `LSTM`, `If`, or `Equal`.

## Published ONNX files

From `src/silero_vad/data` on master, parsed here (onnx 1.23.2). Op
counts include subgraphs.

| File | sha256 | Bytes | Not in the 1.31 builder set |
| --- | --- | --- | --- |
| `silero_vad.onnx` (pin) | `1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3` | 2327524 | `If` (25), `LSTM` (4), `Equal`, `Not`, `Size` |
| `silero_vad_16k_op15.onnx` | `7ed98ddbad84ccac4cd0aeb3099049280713df825c610a8ed34543318f1b2c49` | 1289603 | `If` (12), `LSTM` (2), `Equal`, `Not`, `Size` |
| `silero_vad_half.onnx` | `1e0b195ad4806595ef4466f419d16fca7e4afcfc6669b8c0b5f76ea87547c769` | 1280395 | `If` (12), `LSTM` (2), `Equal`, `Not`, `Size` |
| `silero_vad_16k_sequence.onnx` | `9ccdacc4719d8aa7e45a77536bfabec45a03ba1f2fad5e241ab4060b24238a85` | 1246165 | `LSTM` (1). No `If` |
| `silero_vad_openvino_16k.onnx` | `7776b81ad1b0350c15d7f1555943b9232eb53e9ca5d989c6d0cea9ebc8664d87` | 1288203 | `LSTM` (1). No `If` |
| `silero_vad_op18_ifless.onnx` | `7671cd04b004e9076da0d4a7b1a5aec36adf161c39230c1cb94a4fd5db6bbd28` | 2845718 | top-level `If` (1) and `Equal` (1). No `LSTM` node |

`silero_vad_16k_sequence.onnx` and `silero_vad_openvino_16k.onnx` drop
the sample-rate `If` and keep one top-level `LSTM`. The recurrent
update still has no builder. That is the same Stop as the pin.
`silero_vad_16k_op15.onnx` and `silero_vad_half.onnx` keep both `If`
and `LSTM`.

## `silero_vad_op18_ifless.onnx`

Same Silero weight names (`model.encoder.*`, `model.decoder.rnn.*`,
`model.stft.forward_basis_buffer`, plus the 8 kHz copies). The `LSTM`
node is unrolled into `Gemm`, `Split`, `Sigmoid`, `Tanh`, and `Mul`,
which are registered. The top-level graph is still `Reshape`, `Gather`,
`Equal`, `If`. The `If` selects `sr == 16000`. `Equal` and `If` are not
registered. The compute is inside the two branches.

Those branches are not a graph the 1.31 support checks accept:

- Each branch has zero initializers. Weights and pads are parent-graph
  initializers.
- The first node is `Pad` with `mode=reflect`. Pads are `[0, 0, 0, 64]`
  on the 16 kHz branch and `[0, 0, 0, 32]` on the 8 kHz branch.
  `PadOpBuilder::IsOpSupportedImpl` looks `pads` up in
  `GraphViewer::GetAllInitializedTensors()`, which is only that
  subgraph's initializer set. The pads tensor is not in it.
- Reflect also requires the pad amount to be less than the dimension
  size. The audio input dims are symbolic (`batch`, `sequence`).
- `Slice` calls `GetStaticShape`, which rejects a dynamic dim. The STFT
  `Conv` that feeds `Slice` keeps a symbolic time axis.

`EnableOnSubgraphs=1` can look at a branch. It does not make those
checks pass. A name that appears in `GetOpBuilders()` is not a
partition. This host does not compile `compiled_model.mlmodelc`.

## Not an ONNX the EP takes

`aufklarer/Silero-VAD-v6.2.1-CoreML` is a CoreML `.mlmodelc` with
explicit `h` and `c` state. `taco_vad_ort` loads an ONNX file through
ORT. That package is not a pin.

## SPDX

`https://github.com/snakers4/silero-vad/blob/master/LICENSE` is the MIT
license text, Copyright (c) 2020-present Silero Team. The README says
the project is published under MIT. The badge alt text says
"License: CC BY-NC 4.0"; the badge image and the link target are the
MIT file. That alt text is not a second license. `freeze.rs`
`silero-vad-weights` keeps `spdx: None`. This note does not set it.

## Why the pin stays

M0 pins are the three measured files, recorded together. The charter
VAD family stays Silero. No published Silero ONNX is a graph ORT 1.31
CoreML can take: the other files still contain `LSTM`, or they keep a
top-level `If` whose body fails the Pad and Slice support checks.
Replacing `1a153a22…` would not move `device=` off `cpu`.

## What a later milestone needs

A publisher ONNX whose nodes the 1.31 support checks accept: no `LSTM`,
no `If`, pads and slice bounds visible as constants on the graph that
is partitioned, and static shapes where `Slice` and reflect `Pad`
require them. Or an ORT CoreML builder for `LSTM`, and for `If` if the
sample-rate wrapper stays. Then a Mac `taco run` with
`TACO_ENGINES_PROVE=1` has to print `MLGPUComputeDevice` or
`MLNeuralEngineComputeDevice` from `MLComputePlan` before `device=`
may leave `cpu`. No conversion on the user machine (ADR 0005). No
`.mlmodelc` side path. M0-AC2 stays unchecked until that Mac line
exists.

## Consequence

- [ ] **M0-AC2:** Still open. The pin is unchanged. ASR and MT on MTL0
  are unchanged.
