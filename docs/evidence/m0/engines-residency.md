# Engine residency

Cited from `cursor/m0-engines-residency-b19b` tip `0972b2d` (PR #32). This branch does not replay the residency test. No acceptance box is checked.

One hundred `session::prove_native` cycles in one process (pid 29510) on the same Apple M1 Pro, 34359738368 bytes, macOS 27.0.1 (26A434). The binary is `t_m0_residency`, not `taco run`. `proves_c1` stays false. This host is not C2.

`phys_footprint` after cycle 1 teardown is 82625424 bytes. After cycle 100 it is 334972056 bytes. Delta 252346632. Tolerance 33554432. The cycle-100 teardown is 218792200 bytes over that band. Peak during a prove is 904627760 (cycle 97). Every cycle printed `ep=CPU`, `ac2=0`, and `ac3=0`. `MTLDevice.currentAllocatedSize` was not read.

- [ ] **M0-AC2:** Pins loaded on MTL0 and Silero's provider was CPU. The footprint is over tolerance. The process is not `taco run`.
- [ ] **M0-AC3:** Each prove set `teardown=1`. Resident footprint did not return to the cycle-1 teardown. The engine printed `ac3=0`.
