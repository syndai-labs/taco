# Engine adapters (2026-10-17)

`src/engines.rs` is the in-process surface for VAD, ASR (Whisper), and MT
(llama / Hy-MT2): `initialize`, bounded `infer`, cooperative `cancel`, and
`close`. `session::open_inference` is the call site. `taco run` uses that
function and does not spawn an inference CLI.

An unbound call still refuses. Missing pins, a non-macOS host, and an unset
`native` feature each refuse. A host that has not bound a model directory
still refuses, even when the static archives are linked. That refusal is
fail closed. Bounded `infer` rejects an oversized buffer before any native call.

When a model directory is bound and this Mach-O linked the static engines,
`initialize`, `infer`, and `cancel` call `taco_engines_init`,
`taco_engines_infer`, and `taco_engines_cancel`. VAD uses the static ONNX
Runtime session when those archives were present at build time. ASR and MT
use whisper.cpp and llama.cpp on one ggml with Metal shaders embedded.
`session::prove_native` runs the three-pin load, infer, and cancel in this
process. `taco run` calls that function only when `TACO_ENGINES_PROVE=1`.
A nonzero native status stays `engine_not_linked`. It is not a pass.

This is not M0-AC2 and not M0-AC3. Acceptance checkboxes stay open.
The measured Mach-O is in `taco-engines.md`.
