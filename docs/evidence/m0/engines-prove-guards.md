# Product prove-gate source guards

`tests/charter_guardrails.rs` (`taco_run_prove_gate_fails_closed_in_source`)
reads `src/main.rs`. `taco run` reaches `engines_prove` only when
`TACO_ENGINES_PROVE` is `1`. Any other value, including unset, still hits
`start_capture`. A missing `--model-dir` and an empty `TACO_MODEL_DIR` return
`engine_not_linked` before `prove_native`. The predicate line stays
`ac2=0 ac3=0`.

`tests/t_m0_reject_inputs.rs` (`taco_run_engines_prove_fails_closed`) still
spawns that binary for unset, `0`, and `1`. Capture, static ORT, `HOLD_MS`,
and `OPEN_MS` asserts are unchanged.

Checked with Linux default `cargo test` only. This note does not record a
Mac `taco run`.

- [ ] M0-AC2
- [ ] M0-AC3
