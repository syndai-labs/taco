# Silero CoreML subgraph request (stacked on #37)

Stop. `device=` stays `cpu`. M0-AC2 stays unchecked.

#37 (`d194012`, [vad-metal-ep.md](vad-metal-ep.md)) linked ONNX Runtime 1.31.0
CoreML, requested `MLComputeUnits=CPUAndGPU` and `ModelFormat=MLProgram`, and
measured `vad_ort_plan=missing` plus `vad_ort=ok device=cpu`. This note names
why that cache stayed empty, and the one session option the product path now
sets. It does not re-measure that Mac run.

## What #37 did not set

`CoreMLExecutionProvider::GetCapability` in ORT 1.31.0 returns an empty
partition when the graph is a subgraph and `EnableOnSubgraph()` is false.
That flag is provider option `EnableOnSubgraphs`. It defaults to `0`.
`COREML_FLAG_ENABLE_ON_SUBGRAPH` is the same bit (`0x002`). #37's option map
did not include it.

The pinned Silero file is the top-level `If` that selects the sample-rate
branch. `compiled_model.mlmodelc` is written only after `Compile` of a
non-empty CoreML partition (`model.mm`, `CompileOrReadCachedModel`). With
subgraph partitioning skipped, that file is never created, and
`taco_vad_profile_cache` prints `vad_ort_plan=missing`. Input lines from
`GetEpDeviceForInputs` stay `CPUExecutionProvider` because those inputs feed
the CPU `If`, not a CoreML partition. #37 also left
`session.record_ep_graph_assignment_info` unset, so node counts were not
available. That entry is not a device.

## Pinned graph

`silero_vad.onnx`, sha256
`1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`,
2327524 bytes (Silero VAD v6.2.1). Parsed here; not executed on this host.

| Depth | What is there |
| --- | --- |
| 0 | `Constant`, `Equal`, `If`, two `Identity` |
| 2 | four `LSTM` nodes, inside nested `If` branches |

ORT 1.31.0 `coreml::GetOpBuilders()` has no `LSTM` builder and no `If`
builder. `EnableOnSubgraphs=1` lets `GetCapability` look at the branch
bodies. It does not add those builders. Conv nodes in the branches can be
partitioned only if the support check accepts them. The recurrent state
update cannot. No Linux run can show which partition, if any, compiles.

## Product path

`taco_engines_prove` and `taco_engines_init` call `taco_vad_ort`. `taco run`
calls `session::prove_native` only when `TACO_ENGINES_PROVE=1`. When
`TACO_HAVE_ORT_COREML` is set, that session now appends `CoreML` with
`EnableOnSubgraphs=1` and sets
`session.record_ep_graph_assignment_info=1`. The `device=` token is still
taken only from `MLComputePlan` preferred-device classes. A missing plan
stays `cpu`. There is no `device=metal` string in this tree.

## This host

Linux. CoreML is not in the binary. `taco_vad_ort` prints
`vad_ort=not_linked device=cpu reason=coreml_ep_not_in_this_binary` and
returns 1. `AppendExecutionProvider("CoreML")` is not called.
`MLComputePlan` is not called. That is the fail-closed result. It is not a
Metal measurement.

## What a Mac run has to print

Before `device=` may leave `cpu`, a product prove
(`TACO_ENGINES_PROVE=1` or `session::prove_native`) on the CoreML static
link has to print `vad_ort_plan op=… device=MLGPUComputeDevice` or
`device=MLNeuralEngineComputeDevice`. `subgraphs=1` on the request line is
not that. Hardware inventory is not that. `ac2` stays 0.

## Consequence

- [ ] **M0-AC2:** Still open. ASR and MT on MTL0 are unchanged. Silero's
  measured execution provider is still CPU.
- R3 stays Stop, stacked on #37. The missing runtime fact on this host is
  `MLComputePlan` over a CoreML `compiled_model.mlmodelc`. The missing EP
  fact, even on a Mac, is an `LSTM` / `If` op builder in ORT 1.31.0 CoreML.
