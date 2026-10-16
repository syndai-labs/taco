# Silero VAD execution provider (2026-10-17)

Stop. The pinned Silero ONNX still runs on `CPUExecutionProvider`.
ONNX Runtime 1.31.0 has no Metal execution provider. A static CoreML
provider was linked and requested. The session assigned every input to
CPU. M0-AC2 stays open.

## Host

| Field | Value |
| --- | --- |
| Chip | Apple M1 Pro |
| OS | macOS 27.0.1, build 26A434 |
| Arch | arm64 |
| When | 2026-10-17 |

No TCC dialog was used.

## What was tried

ORT 1.31.0 source at the engines cache lists providers under
`onnxruntime/core/providers/`. There is a `coreml` directory. There is
no Metal execution provider.

The CPU static archives used by `cursor/m0-taco-engines-b19b` were built
with `onnxruntime_USE_COREML=OFF`. A second static build used the same
`build.py` flags plus `--use_coreml` and
`CMAKE_POLICY_VERSION_MINIMUM=3.5` (psimd's CMake floor). Configure
recorded `onnxruntime_USE_COREML=ON` and `onnxruntime_BUILD_SHARED_LIB=OFF`.
The build produced `libonnxruntime_providers_coreml.a` and
`libcoreml_proto.a`. `re2` is `EXCLUDE_FROM_ALL` in that tree; `libre2.a`
was built with `cmake --build ... --target re2` before the link.

`taco_vad_ort.cpp` appends provider `CoreML` with
`MLComputeUnits=CPUAndGPU`, `ModelFormat=MLProgram`, and
`ProfileComputePlan=1` only when that static archive is on the link
line. The device token is taken from `MLComputePlan` preferred-device
classes. A missing plan stays `device=cpu`.

## Measurement

One Mach-O, static link of `native/taco_vad_ort.cpp` and
`native/taco_vad_plan.mm` against those archives, loading
`silero_vad.onnx`
(`1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`).
This was not `cargo test --features native` and not `taco run`.
The full three-model native test was not re-run.

```text
vad_ort_provider=CoreMLExecutionProvider
vad_ort_provider=CPUExecutionProvider
vad_ort_hw type=npu vendor=Apple
vad_ort_hw type=gpu vendor=Apple
vad_ort_hw type=cpu vendor=Apple
vad_ort_request ep=CoreML units=CPUAndGPU format=MLProgram
vad_ort_session=ok version=1.31.0 inputs=3 outputs=2
vad_ort_assign=fail msg=Session configuration entry 'session.record_ep_graph_assignment_info' must be set to "1" to retrieve EP graph assignment information.
vad_ort_input_ep index=0 ep=CPUExecutionProvider hw=cpu
vad_ort_input_ep index=1 ep=CPUExecutionProvider hw=cpu
vad_ort_input_ep index=2 ep=CPUExecutionProvider hw=cpu
vad_ort_in name=input type=1 rank=2 dims=-1,-1
vad_ort_in name=state type=1 rank=3 dims=2,-1,128
vad_ort_in name=sr type=7 rank=0 dims=
vad_ort_frame=0 samples=512 sr=16000 prob=0.000592 state_l2=8.108584
vad_ort_frame=1 samples=512 sr=16000 prob=0.000528 state_l2=11.574153
vad_ort_frame=2 samples=512 sr=16000 prob=0.000592 state_l2=8.108584
vad_ort_reset=1 match=1 carried_state=1 window=512 state=2x1x128
vad_ort_plan=missing
vad_ort=ok device=cpu plan_gpu=0 plan_cpu=0 plan_ane=0 plan_other=0 coreml_nodes=0 cpu_nodes=0
```

Exit 0. `vad_ort_hw` is `GetHardwareDevices` inventory. The session
assignment is `vad_ort_input_ep`. No `MLGPUComputeDevice` line was
printed. `vad_ort_plan=missing` means the CoreML cache had no
`.mlmodelc` to profile. Probabilities match the earlier CPU-only run
in `taco-engines.md`.

`file` reported `Mach-O 64-bit executable arm64`. `otool -L` listed
system libraries only: CoreML, Foundation, CoreFoundation, Accelerate,
libc++, libobjc, libiconv, libz, libSystem. No `libonnxruntime` dylib.
`_OrtGetApiBase` is a defined text symbol.

Graph node counts were not recorded. The assignment API requires
`session.record_ep_graph_assignment_info=1`, and that entry was not set.
The input-provider lines above are the measured device.

## Consequence

- [ ] **M0-AC2:** ASR and MT on MTL0 are unchanged from `taco-engines.md`. Silero's measured execution provider is still CPU. This note does not check the box.
- R3 stays Stop. Incremental Silero runs. It does not run on Metal or on a CoreML compute device.
- A CoreML request flag, or a GPU listed by hardware inventory, is not a VAD device string.
