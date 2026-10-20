# Engine residency retry

Cited from `cursor/m0-residency-retry-b19b` tip `37ff031` (PR #49). This branch does not replay the residency test. No acceptance box is checked. The 100-cycle Stop at `a0e1614` stands. See [engines-residency.md](engines-residency.md).

The holder is named in [engines-residency-rootcause.md](engines-residency-rootcause.md) (PR #55 tip `0829a70`): the process-lifetime Metal device in `ggml_backend_metal_reg`. This file is the vmtrace. It is not a pass.

Stop. Not a pass. Same host as the 100-cycle run: Apple M1 Pro, 34359738368 bytes, macOS 27.0.1 (26A434). Not a qualified corner. `proves_c1` stays false. No capture was started.

A vmtrace on the unpatched residency binary freed every tracked `vm_allocate` of at least 8 MiB, including the 487014400-byte weight buffer. `phys_footprint` on cycle 2 was 175309952, 94699688 bytes above the cycle-1 teardown of 80610264. Tolerance is 33554432. `GGML_METAL_NO_RESIDENCY=1` missed that band in eight cycles (delta 89096360). A synchronize before the weight-buffer free (eight-cycle delta 98173096) and a waited blit after `vm_deallocate` (eight-cycle delta 103071936) also missed. Those two edits were reverted and are not in this tree. Silero stayed `ep=CPU`. The probes printed `ac2=0` and `ac3=0`.

`vm_deallocate` returning is not `phys_footprint` returning. Do not treat this retry as meeting `T-M0-LINK-THREE-MODELS` or the acceptance.md 100-cycle rule.

- [ ] **M0-AC2:** Unchecked. Silero stayed `ep=CPU`. `phys_footprint` did not stay within 33554432 of the cycle-1 teardown.
- [ ] **M0-AC3:** Unchecked. Resident footprint did not return to the post-first-teardown baseline.
