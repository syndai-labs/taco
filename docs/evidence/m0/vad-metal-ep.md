# Silero VAD execution provider

Cited from `cursor/m0-vad-metal-ep-b19b` tip `d194012` (PR #37). This branch does not replay the CoreML link. No acceptance box is checked. This note does not name C2.

Stop. ONNX Runtime 1.31.0 has no Metal execution provider. A static CoreML provider was linked and requested with `MLComputeUnits=CPUAndGPU` and `ModelFormat=MLProgram`. The session assigned every input to `CPUExecutionProvider`. `vad_ort_plan=missing` (no `.mlmodelc`). `vad_ort=ok device=cpu`. Frame probabilities match the earlier CPU run (0.000592 then 0.000528). The Mach-O was not `cargo test --features native` and not `taco run`. `otool -L` was system libraries only. A hardware-inventory GPU line is not the VAD assignment.

- [ ] **M0-AC2:** Silero's measured execution provider is still CPU.
