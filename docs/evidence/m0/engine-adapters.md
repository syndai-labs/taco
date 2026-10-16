# Engine adapter scaffolding (2026-10-16)

`src/engines.rs` is the in-process surface for VAD, ASR (Whisper), and MT
(llama / Hy-MT2): `initialize`, bounded `infer`, cooperative `cancel`, and
`close`. `session::open_inference` is the call site. `taco run` uses that
function and does not spawn an inference CLI.

Every call refuses. Missing pins, a non-macOS host, and an unset `native`
feature each refuse. A host that passes those checks still refuses: nothing
is linked. Bounded `infer` rejects an oversized buffer before that refusal.

This is scaffolding, not M0-AC2 and not M0-AC3. It does not load models, show
Metal, choose a Silero runtime, or join a cancelled worker. Those results
stay with the Metal link probe. `T-M0-LINK-THREE-MODELS` and
`T-M0-CANCEL-LIFECYCLE` stay fail-closed. Acceptance checkboxes stay open.
