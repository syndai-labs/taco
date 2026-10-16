# Engine adapter scaffolding (2026-10-16)

`src/engines.rs` is the in-process surface for VAD, ASR (Whisper), and MT
(llama / Hy-MT2): `initialize`, bounded `infer`, cooperative `cancel`, and
`close`. `session::open_inference` is the call site. `taco run` uses that
function and does not spawn an inference CLI.

Every product call refuses. Missing pins, a non-macOS host, and an unset
`native` feature each refuse. A host that passes those checks still refuses.
Bounded `infer` rejects an oversized buffer before that refusal.

## Static link hook

This note is scaffolding bridge only. `native/taco_engines.h` is the probe symbol
`taco_engines_linked` from `cursor/m0-metal-link-b19b` at `afa3cab`.
`native/CMakeLists.txt` builds the product static library on llama.cpp's
ggml, then whisper.cpp, with shared libraries off and Metal shaders embedded.
It does not build `taco_link_probe`.

`native/ffi` is the only crate that may call that symbol. The `taco` crate
still forbids unsafe. `--features native` on macOS aarch64 links those static
archives into `taco` when both `TACO_LLAMA_SRC` and `TACO_WHISPER_SRC` point
at local checkouts. Unset variables skip cmake. Linux ignores those variables.
Nothing is downloaded. Models are not opened. Silero stays out of this hook.

`engines::gpu_offload_linked` is that call. `initialize`, `infer`, `cancel`,
and `close` do not use it. A true result is the probe's GPU-offload flag,
not a model load, not a joined cancel, and not a pass.

This is scaffolding, not M0-AC2 and not M0-AC3. `T-M0-LINK-THREE-MODELS` and
`T-M0-CANCEL-LIFECYCLE` stay fail-closed. Acceptance checkboxes stay open.
