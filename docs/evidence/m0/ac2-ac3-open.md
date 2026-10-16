# M0-AC2 and M0-AC3 stay unchecked

Refreshed 2026-10-16. No acceptance box is checked. The cargo-test prove on
`7f98ab1` stays in [taco-engines.md](taco-engines.md). This note does not
re-run it.

## Gaps

(a) Still open. Silero is the ORT CPU execution provider (`device=cpu` on
the #30 `taco run`, `ep=CPU` on the #32 residency series). M0-AC2 requires
observed Metal GPU execution. A Metal EP worker may ship separately.

(b) Closed for prove reachable from `taco run`. #30 `e2ec751` ran an ad-hoc
signed Mac `taco run` with `TACO_ENGINES_PROVE=1`. Stdout printed
`link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`. Exit 0. `codesign` reports
`Signature=adhoc`, which is not a Developer ID signature. `ac2` and `ac3`
stayed 0. Detail: [taco-run-prove.md](taco-run-prove.md).

(c) Still blocks M0-AC2 and M0-AC3. #32 `a0e1614` records a Stop, not a fix.
100 `session::prove_native` cycles in one process. `phys_footprint` after
cycle 100 is 252346632 bytes above the cycle-1 teardown. Tolerance is
33554432 bytes. `whisper_free` retains the bytes. A Metal command-buffer
flush missed the band (6-cycle delta 98812072) and was reverted. Detail:
`docs/evidence/m0/engines-residency.md` on `cursor/m0-engines-residency-b19b`.

## Consequence

M0-AC2 and M0-AC3 stay unchecked. (a) and (c) are still open. Repeated
teardown is not a bounded-residency result. Do not treat #32 as meeting
`T-M0-LINK-THREE-MODELS` or the acceptance.md 100-cycle rule, and do not
start M1 on a claim that engine teardown returns to the cycle-1 baseline.
