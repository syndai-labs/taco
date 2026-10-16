# `taco run` prove gate

Recorded 2026-10-16 on Linux. No Mac `taco run` was executed with this gate.
M0-AC2 and M0-AC3 stay unchecked. `ac2` and `ac3` in the predicate line are
fixed at 0.

`taco run` calls `session::prove_native` only when `TACO_ENGINES_PROVE=1`.
That is the same `engines::prove_pins` path the native cargo tests use. On
`Ok` this process prints:

```text
link_three=<n> cancel_lifecycle=<n> ac2=0 ac3=0
```

The static engines print that same line during the call. Any other value of
the variable, including unset, keeps the capture preflight and does not call
`prove_native`. A missing `--model-dir` and an empty `TACO_MODEL_DIR` refuse
with `engine_not_linked` and do not call native code. An unbound or
non-macOS build refuses the same way from `prove_native`.

## Linux

Default `cargo test` (no `native` feature) includes
`taco_run_engines_prove_fails_closed`. It spawns:

```text
taco run --source default --from en --to zh --model-dir /tmp/taco-m0-absent
```

With `TACO_ENGINES_PROVE=1`: exit 1, stderr `engine_not_linked`, stdout has
no `link_three=` line. Without the variable, and with `=0`, stderr is
`platform_unsupported`.

`cargo test --features native` stays fail-closed on this host. The link and
cancel tests panic off macOS Apple Silicon.

## Not measured

No signed macOS `taco` binary was run with `TACO_ENGINES_PROVE=1`. The
`link_three=1 cancel_lifecycle=1` lines in
[taco-engines.md](taco-engines.md) are still `cargo test` processes, not
this CLI. CPU VAD and catalog residency are unchanged. See
[ac2-ac3-open.md](ac2-ac3-open.md).
