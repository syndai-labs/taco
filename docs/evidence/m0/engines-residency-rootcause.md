# Engine residency root cause (2026-10-20)

Stop. Not a pass. M0-AC2 and M0-AC3 stay unchecked.

This stacks on the #49 vmtrace (`37ff031`, [engines-residency-retry.md](engines-residency-retry.md)) and the 100-cycle record (`a0e1614`, [engines-residency.md](engines-residency.md)). No new Mac run was taken. This host is Linux and does not link Metal. Nothing below is a 100-cycle pass.

## Holder

The residual `phys_footprint` is the process-lifetime Metal device inside `ggml_backend_metal_reg`, not a live `vm_allocate` from the prove path.

`ggml_metal_buffer_free` releases each `MTLBuffer`, then `vm_deallocate`s the host mapping only when `is_shared && owned`. That is the free the #49 interposer matched. On cycle 2, after that 487014400-byte free had already returned, `virtual_size` at `after_whisper_free` was still 498401280 bytes above `before_load`. One weight-sized mapping was still in the task. It was not created by the interposed `vm_allocate` (threshold 8388608, `live_n=0` at process exit). It is the Metal-side mapping: `newBufferWithBytesNoCopy` on the shared host buffer, or a private `newBufferWithLength`, owned by the static device `ggml_backend_metal_reg` installs on first use.

`llama_backend_free` does not destroy that registry. The device, its pipeline cache, and any `MTLBuffer` it still retains stay mapped until process exit. Shared-mode unified memory is counted in `internal` / `phys_footprint`. It is not counted in `ledger_tag_graphics_footprint`, which stayed 4390912 after the first teardown. That is why the graphics ledger looked flat while the footprint did not.

Cycle 1 is the exception that names the same object. `after_whisper_free` retained about 110838072 bytes, then the first aborted MT load (`after_mt_cancel_init`) dropped the footprint. Cycle 2's aborted MT load did not. The first `llama_model_load_from_file` is what brings the singleton up and drains whisper's buffers under that allocation. Later cycles reuse the warm device, so the same teardown leaves the mapping in place. From cycle 2 the after-teardown series is already 89604264 bytes over cycle 1, then stair-steps to 252346632 by cycle 100. The late +16384 steps are page-sized, not another model load.

## Ruled out

| Candidate | Why it is not the holder |
| --- | --- |
| App `vm_allocate` ≥ 8 MiB, including the 487014400-byte weight buffer and the whisper graph buffers | #49 matched every free. `exit_sum=0`. |
| Metal residency-set keep-alive | `GGML_METAL_NO_RESIDENCY=1` logged `use residency sets = false` and cycle 8 was still 89096360 over cycle 1. `GGML_METAL_RESIDENCY_KEEP_ALIVE_S=1` was worse (104890560). |
| In-flight command buffer | Synchronize-before-free and a waited blit after `vm_deallocate` both missed 33554432 within 8 cycles. Those patches stayed reverted. |
| Default malloc zone | `malloc_size_in_use` was 36577264 after cycle 1 and 36520176 after cycle 100. |
| ORT arena | `Ort::Session` and `Ort::Env` die inside `taco_vad_ort` before whisper loads. The phase step is `after_vad_ort` → `after_whisper_free` (64111336 → 174949408 on cycle 1). Silero's file is 2327524 bytes. |
| Page cache of the weight file | #32's phase log showed `internal` moving with `phys_footprint` and `compressed` at 0. A clean file cache would show up as external, and `vm_deallocate` of the weight buffer would not leave a 498401280-byte `virtual_size` gap. |
| Missing `whisper_free` | The success path calls `whisper_free`. The init-fail path is patched to call it too. Both cycles in the vmtrace freed the 487014400-byte buffer before `after_whisper_free`. |

## Why there is no purge in this tree

The holder is inside the pinned ggml, not in `taco_runtime.cpp`. [metal-device-teardown.md](metal-device-teardown.md) reads llama.cpp `2a667032dc788c3a071d538e0b572e45a3f12731`. `g_devices` and the `MTLDevice` from `ggml_metal_device_get` stay until process exit. `llama_backend_free` is `ggml_quantize_free()` and does not destroy `ggml_backend_metal_reg`.

Calling `ggml_metal_device_free` from taco, or nulling only the registry pointer, is a use-after-free on the next `prove_native`. The buffer-type statics and `ggml_backend_registry::devices` still hold that device, and `ggml_metal_device_get`'s `unique_ptr` would free it again at exit. Whisper and llama backends in this process (`product::g_asr`, `product::g_mt`) would see the same freed device. This pin has no function that drops all of those owners together. A Linux stub that pretends to drop an `MTLBuffer` would not be that device.

An autorelease-pool pop around prove/cancel is the wrong lever for this free path. `ggml_metal_buffer_free` uses `[buffer release]`, not `autorelease`. The #49 waits already covered in-flight command buffers. #59 tip `ed9ee10` already locks the AC2/AC3 checkbox while #55 tip `0829a70` stands. This tree does not add a second lock.

## Mac measurement still required

Same machine as #32/#49 (M1 Pro, 34359738368 bytes, macOS 27.0.1 build 26A434). One process. Do not start a new process per cycle.

1. At `before_load`, `after_whisper_free`, and `after_backend_free`, read `MTLCreateSystemDefaultDevice().currentAllocatedSize` (and `recommendedMaxWorkingSetSize`) next to `phys_footprint`.
2. If `currentAllocatedSize` tracks the ~90 MiB cycle-2 step, the retainer is the process-lifetime device named above. Do not null `ggml_backend_metal_reg` from taco and do not call `ggml_metal_device_free`. That sample does not make a free safe. The procedure is in [metal-device-teardown.md](metal-device-teardown.md).
3. If `currentAllocatedSize` stays flat while `phys_footprint` grows, the system device is not the retainer. Interpose `mach_vm_map` / `vm_map` (the #49 probe only interposed `vm_allocate`) or take `vmmap -summary` at `after_whisper_free` and name the region of about 487014400 bytes.
4. Eight cycles first. Continue to 100 only if cycle 8 is within 33554432 of cycle 1. `link_three` has to stay 1. The engine keeps printing `ac2=0` and `ac3=0`.

Acceptance stays open until a 100-cycle run on that host shows cycle 100 ≤ cycle 1 teardown + 33554432. This document does not record that run.

## Still open

- [ ] **M0-AC2:** Unchecked. The #32/#49 probes loaded pins in process on MTL0 and missed the 33554432 band. Silero stayed `ep=CPU`. No purge was added here. The engine printed `ac2=0`.
- [ ] **M0-AC3:** Unchecked. Teardown still sets `teardown=1` and does not return `phys_footprint` to the cycle-1 baseline. The engine printed `ac3=0`.
