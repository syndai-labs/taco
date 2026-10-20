# ORT CoreML upgrade is out of M0 (stacked on #60 `aab661c`)

Stop. Upgrading the static ONNX Runtime is out of M0. `coreml::GetOpBuilders()`
on current `main` is the same file as tag `v1.31.0` and still does not
register `LSTM` or `If`. The Silero pin stays
`1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`.
`device=` stays `cpu`. M0-AC2 stays unchecked.

Linux. No CoreML, no `MLComputePlan`, no Metal. This note does not create
a session and does not print a device. The fail-closed line for this host
is still `vad_ort=not_linked device=cpu reason=coreml_ep_not_in_this_binary`
from [vad-coreml-subgraph.md](vad-coreml-subgraph.md).

## Registry

Checked 2026-10-20 against `microsoft/onnxruntime`.

| Ref | Commit | What it is |
| --- | --- | --- |
| tag `v1.31.0` | `402979ac1bef89dcd55add41eaa7277c407599eb` | Newest `v1.*` release. Published 2026-10-09. The static archives #37 linked print `version=1.31.0`. |
| `main` | `a7a1172c9a044629bb12c75fb2a89c0c347c7f28` | 2026-10-20. Compare `v1.31.0...main` is diverged: 91 commits ahead, 1 behind, 300 files. |

`onnxruntime/core/providers/coreml/builders/op_builder_factory.cc` is git blob
`891704e8d9ca33411424c97ec7dc75ab1600540b`, 4639 bytes, on both refs.
`CreateOpBuilderRegistrations()` does not register `LSTM`, `If`, `Equal`,
`Not`, or `Size`. `builders/impl/` on `main` has no `lstm_op_builder`,
`if_op_builder`, or `equal_op_builder`.

The last commit that edits that factory file is `073e49ffe1` (2026-06-08),
`[CoreML EP] Add Where and And builders (#28597)`. That commit is already
in `v1.31.0`.

The only CoreML builder file in the 91 commits is
`builders/impl/base_op_builder.h`, changed by `988cc07aed` (2026-10-12,
`#33134`, ONNX 1.23.2 plumbing). The diff is one line:
`GetMaxSupportedOpSet` returns 28 instead of 27. The same commit appends
`ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT6E2M3` and `FLOAT6E3M2` to
`ONNXTensorElementDataType` in `onnxruntime_c_api.h`. That is an opset
ceiling and two element types. It is not an operator registration.

The v1.31.0 release notes' CoreML section is device-based EP selection,
including Apple Neural Engine through `PREFER_NPU` (`#31975`). LSTM lines
in that release are CPU kernel checks. QNN has its own LSTM builder
(`2aa961e`, `#24646`). That file lives under `providers/qnn/`, not
`providers/coreml/`.

`EnableOnSubgraphs=1` (#57 `88eda06`) lets `GetCapability` look inside an
`If`. It does not add a builder. #60 `aab661c` already showed the published
Silero ONNX files still need `LSTM` or a top-level `If` this registry
cannot take. This note does not re-parse those files and does not change
the pin.

## Static Mach-O

ORT is not a Cargo dependency. `native/CMakeLists.txt` sets
`BUILD_SHARED_LIBS OFF` and links archives from `ORT_ROOT`
(`TACO_ORT_ROOT`, else `.cache/m0-onnx`). When
`libonnxruntime_providers_coreml.a` is present it is a whole-archive
static input plus `libcoreml_proto.a` and the `CoreML` system framework.
ADR 0004 freezes the runtime inside one Mach-O and rejects a
user-installed ONNX dylib. #37's `otool -L` was system libraries only.

A Mac rebuild of `main` at `a7a1172c` with `onnxruntime_USE_COREML=ON` and
`onnxruntime_BUILD_SHARED_LIB=OFF` would still be one Mach-O. It would
compile this same `GetOpBuilders()` body. The pinned graph would still
have no CoreML builder for its `LSTM` nodes or its `If` nodes. `device=`
cannot leave `cpu` because of that rebuild. The C API enum grew two
FLOAT6 values, so the rebuild is also not a drop-in header bump. M0 does
not take that rebuild.

## What a later milestone needs

A merged ORT change whose `GetOpBuilders()` registers `LSTM`, and `If` if
the sample-rate wrapper stays, built as static CoreML archives and linked
the way `CMakeLists.txt` already links them. Then a Mac `taco run` with
`TACO_ENGINES_PROVE=1` has to print `MLGPUComputeDevice` or
`MLNeuralEngineComputeDevice` from `MLComputePlan` before `device=` may
leave `cpu`. #59 `ed9ee10` already fails the suite if M0-AC2 is checked
while `88eda06` stands. This note is why that Stop is not waiting on an
ORT version bump.

## Consequence

- [ ] **M0-AC2:** Still open. The pin is unchanged. ASR and MT on MTL0 are
  unchanged. Silero stays on the CPU execution provider.
