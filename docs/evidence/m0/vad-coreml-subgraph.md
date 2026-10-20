# Silero CoreML subgraph request

Cited from `cursor/m0-vad-coreml-ep-b19b` tip `88eda06` (PR #57). Stacked on #37 tip `d194012` ([vad-metal-ep.md](vad-metal-ep.md)). This branch does not replay `native/taco_vad_ort.cpp` or that tip's charter guard. No acceptance box is checked. This note does not name C2.

Stop. `device=` stays `cpu`. M0-AC2 stays unchecked. No `device=metal` string is recorded.

#37 linked ONNX Runtime 1.31.0 CoreML, requested `MLComputeUnits=CPUAndGPU` and `ModelFormat=MLProgram`, and measured `vad_ort_plan=missing` plus `vad_ort=ok device=cpu`. `EnableOnSubgraphs` was left at its default `0`. `CoreMLExecutionProvider::GetCapability` then returns an empty partition for every subgraph, so `Compile` never writes `compiled_model.mlmodelc`.

The pinned Silero file (`silero_vad.onnx`, sha256 `1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`, 2327524 bytes) is a top-level `If`. Four `LSTM` nodes sit inside nested `If` branches. ORT 1.31.0 `coreml::GetOpBuilders()` registers neither `LSTM` nor `If`. On that tip the product session requests `EnableOnSubgraphs=1`. That option lets `GetCapability` look at the branch bodies. It does not add those builders.

This cite host is Linux. CoreML is not in the binary. The recorded fail-closed line is `vad_ort=not_linked device=cpu reason=coreml_ep_not_in_this_binary`. `MLComputePlan` was not called. That is not a Metal measurement.

Before `device=` may leave `cpu`, a product prove on the CoreML static link has to print `MLGPUComputeDevice` or `MLNeuralEngineComputeDevice`. This document does not record that print.

- [ ] **M0-AC2:** Unchecked. Silero's measured execution provider is still CPU.
