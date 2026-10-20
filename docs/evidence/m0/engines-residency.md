# Engine residency

Cited from `cursor/m0-engines-residency-b19b` tip `0972b2d` (PR #32). This branch does not replay the residency test. No acceptance box is checked. PR #35 tip `7ae6d98` names the later tip `a0e1614` as a Stop, not a fix. `whisper_free` retains the bytes. A Metal command-buffer flush missed the band (6-cycle delta 98812072) and was reverted. The numbers below are from `0972b2d`.

One hundred `session::prove_native` cycles in one process (pid 29510) on the same Apple M1 Pro, 34359738368 bytes, macOS 27.0.1 (26A434). The binary is `t_m0_residency`, not `taco run`. `proves_c1` stays false. This host is not C2.

`phys_footprint` after cycle 1 teardown is 82625424 bytes. After cycle 100 it is 334972056 bytes. Delta 252346632. Tolerance 33554432. The cycle-100 teardown is 218792200 bytes over that band. Peak during a prove is 904627760 (cycle 97). Every cycle printed `ep=CPU`, `ac2=0`, and `ac3=0`. `MTLDevice.currentAllocatedSize` was not read.

- [ ] **M0-AC2:** Pins loaded on MTL0 and Silero's provider was CPU. The footprint is over tolerance. The process is not `taco run`.
- [ ] **M0-AC3:** Each prove set `teardown=1`. Resident footprint did not return to the cycle-1 teardown. The engine printed `ac3=0`.

PR #49 tip `37ff031` ([engines-residency-retry.md](engines-residency-retry.md)) is a stronger Stop on the same host. It does not drop `a0e1614`. Cycle 2 stayed 94699688 bytes above cycle 1 after the weight buffer was freed. Boxes stay unchecked.
