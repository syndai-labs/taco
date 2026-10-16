# M0-AC2 and M0-AC3 stay unchecked (tip `7f98ab1`)

Recorded 2026-10-16 against `cursor/m0-taco-engines-b19b` at `7f98ab1`.
Measurements are the ones already in [taco-engines.md](taco-engines.md).
This note does not re-run them and does not check an acceptance box.

Proven on that tip: `T-M0-LINK-THREE-MODELS` and `T-M0-CANCEL-LIFECYCLE` are
ok via `session::prove_native`. The process printed
`link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`. Silero ran through static ORT
on the CPU execution provider (`vad_ort=ok device=cpu`). Whisper VAD failed
bad magic. ASR and MT ran on Metal MTL0. Joins were under 2 seconds.
`otool -L` showed system libraries only. The run was not `taco run`.
Multi-cycle residency was not measured.

`origin/cursor/m0-engines-residency-b19b` is not on the remote. No pull
request measures catalog residency.

## Why both boxes stay open

(a) The VAD provider was CPU. M0-AC2 requires observed Metal GPU execution.
A CPU-only VAD run does not pass.

(b) The recorded pass is still `cargo test` calling `session::prove_native`.
`taco run` can call that same function when `TACO_ENGINES_PROVE=1`
([taco-run-prove.md](taco-run-prove.md)). This note does not record a signed
Mac run of that gate.

(c) `T-M0-LINK-THREE-MODELS` requires repeated init and teardown, with
resident native allocations back at the post-first-teardown baseline. Each
cargo test called `prove_native` once. That measurement was not taken.

## Consequence

M0 cannot claim M0-AC2 or M0-AC3 until (a), (b), and (c) are closed, or the
charter accepts them.
