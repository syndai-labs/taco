# M0 harness harden (2026-10-16)

Default-tier charter checks on Linux. No device, no Metal, no model load.
Forked from `origin/cursor/m0-capture-wire-b19b` (`4c8dc27`).
Acceptance checkboxes stay open. This file does not accept M0.

## What changed

`tests/charter_guardrails.rs` now walks every `.rs` file under `src/` (including
`src/capture/`) and adds `charter_harness_gaps`:

| Invariant | How it fails |
| --- | --- |
| One binary and one library, both named `taco` | A second `[[bin]]`, a renamed target, or `taco-spike` |
| Languages are only `en`↔`zh` | A third tag, a non-empty experimental list, or `en`→`en` / `ja`→`zh` parsing |
| Four source categories, closed selector grammar | `default`, `input:`, `system`, `app:`, `pid:` classifying onto anything else |
| No `unsafe` in `src` | `unsafe fn`, `unsafe {`, `unsafe impl`, or `unsafe trait` in any Rust file under `src` |
| No `capture_started = true` literal | That assignment, with or without spaces, anywhere under `src` |
| `PINS_MEASURED` matches the pin file | A pin missing a digest or filename, or a digest absent from `pin-measurement.md` |
| Info.plist keys in `assets/` | Missing CFBundle identifier, name, or executable, or bytes that differ from `usage_plist()` |
| Spike env required before device open | `start_capture` without `TACO_CAPTURE_SPIKE` allocating a ring or calling the backend |
| Linux fail-closed | `open_default_microphone` succeeding, or `backend_calls` moving, when the host is not macOS arm64 |

`T-M0-REJECT-INPUTS` and `T-M0-LIMIT-VECTORS` were already enforced. No catalog
assertion was missing that the current types can satisfy, so those files were
not padded.

## Decision register

Updated in `docs/roadmap/risks-and-decisions.md` only where Metal is not required:

- **R7 closed for the table:** `en→zh` and `zh→en` only. Experimental pairs empty. Corpus quality is still not qualified.
- **R8 progressed, not closed:** `T-M0-LIMIT-VECTORS` proves the fixture matches the acceptance plan. Latency, memory, and slope were not measured.
- **Reject-inputs:** default-tier pass is recorded. It does not check M0-AC6.

R2, R5, and the other Metal or device rows stay Stop.

## Not claimed

- M0-AC1 through M0-AC6 remain unchecked.
- No TCC allow/deny, no process-tap open, no whisper.cpp or llama.cpp link, no cancellation join.
- `T-M0-LINK-THREE-MODELS`, `T-M0-CANCEL-LIFECYCLE`, and `T-M0-SPIKE-SOURCES` stay `verdict=not_run`.

## Command

```text
cargo test
```

Host: Linux, device-free, 2026-10-16. `cargo test` exited 0.

```text
charter_guardrails          8 passed
t_m0_limit_vectors          4 passed
t_m0_one_macho              3 passed
t_m0_reject_inputs         10 passed
t_m0_spike_sources          2 passed
```

Native tests (`t_m0_link_three_models`, `t_m0_cancel_lifecycle`) are behind
`--features native` and were not run. No `TACO_CAPTURE_SPIKE` was set.
