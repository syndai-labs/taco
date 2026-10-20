# Engine residency root cause

Cited from `cursor/m0-residency-rootcause-b19b` tip `0829a70` (PR #55). Stacked on #49 tip `37ff031` ([engines-residency-retry.md](engines-residency-retry.md)) and the 100-cycle record `a0e1614` ([engines-residency.md](engines-residency.md)). This branch does not replay the residency test or that tip's charter guard. No acceptance box is checked.

Stop. Not a pass. No new Mac run was taken. This cite host is Linux and does not link Metal. Nothing below is a 100-cycle pass.

## Holder

The residual `phys_footprint` is the process-lifetime Metal device inside `ggml_backend_metal_reg`, not a live `vm_allocate` from the prove path.

On cycle 2, after the 487014400-byte weight-buffer free had already returned, `virtual_size` at `after_whisper_free` was still 498401280 bytes above `before_load`. That mapping was not created by the interposed `vm_allocate` (threshold 8388608, `live_n=0` at process exit). It is the Metal-side mapping owned by the static device `ggml_backend_metal_reg` installs on first use. `llama_backend_free` does not destroy that registry. The device stays mapped until process exit. Shared-mode unified memory is counted in `internal` / `phys_footprint`. It is not counted in `ledger_tag_graphics_footprint`, which stayed 4390912 after the first teardown.

From cycle 2 the after-teardown series is already 89604264 bytes over cycle 1, then stair-steps to 252346632 by cycle 100. Tolerance is 33554432.

## Ruled out on the #32/#49 numbers

| Candidate | Why it is not the holder |
| --- | --- |
| App `vm_allocate` ≥ 8 MiB, including the 487014400-byte weight buffer | #49 matched every free. `exit_sum=0`. |
| Metal residency-set keep-alive | `GGML_METAL_NO_RESIDENCY=1` still left cycle 8 89096360 over cycle 1. |
| In-flight command buffer | Synchronize-before-free and a waited blit after `vm_deallocate` both missed 33554432 within 8 cycles. Those patches stayed reverted. |
| Default malloc zone | `malloc_size_in_use` was 36577264 after cycle 1 and 36520176 after cycle 100. |
| ORT arena | The session dies before whisper loads. Silero's file is 2327524 bytes. |
| Missing `whisper_free` | Both cycles in the vmtrace freed the 487014400-byte buffer before `after_whisper_free`. |

## Why there is no purge in this tree

The purge that matches the holder is inside the pinned ggml, not in taco. `llama_backend_free` does not null the static `ggml_backend_metal_reg` device. Calling `ggml_metal_device_free` from taco on the registry's live pointer is a use-after-free on the next `prove_native`. The llama checkout is `TACO_LLAMA_SRC`. It is not in this repo. A Linux stub that pretends to drop an `MTLBuffer` would not be that device.

`MTLDevice.currentAllocatedSize` was not read. Acceptance stays open until a 100-cycle run on the same host as #32/#49 shows cycle 100 ≤ cycle 1 teardown + 33554432. This document does not record that run.

- [ ] **M0-AC2:** Unchecked. Silero stayed `ep=CPU`. No purge was added. The engine printed `ac2=0`.
- [ ] **M0-AC3:** Unchecked. Teardown does not return `phys_footprint` to the cycle-1 baseline. The engine printed `ac3=0`.
