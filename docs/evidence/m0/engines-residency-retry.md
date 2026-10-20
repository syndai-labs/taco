# Engine residency retry (2026-10-17)

Stop. Not a pass. M0-AC2 and M0-AC3 stay unchecked.

The holder is named in [engines-residency-rootcause.md](engines-residency-rootcause.md): the process-lifetime Metal device in `ggml_backend_metal_reg`. This file is the vmtrace. It is not a pass.

The catalog rule is still the one in `docs/roadmap/acceptance.md`: after 100
in-process start/stop cycles, with models released on each teardown,
`phys_footprint` after cycle 100 ≤ the cycle-1 teardown + 33554432 bytes.
Keeping Whisper, llama, or ORT loaded across cycles would not meet that rule.
This retry did not do that.

The 100-cycle record on `a0e1614` stands: cycle 100 was 252346632 bytes above
cycle 1. See [engines-residency.md](engines-residency.md). The probes below
are shorter. Each one is already past 33554432 by cycle 2 or cycle 8, so none
of them can become a 100-cycle pass unless a later teardown gives the bytes
back. The 100-cycle series did not.

## Host

Same machine as the 100-cycle run: Apple M1 Pro, 34359738368 bytes RAM,
macOS 27.0.1 (26A434). Not a qualified corner. `proves_c1` stays false.
No capture was started. `Taco.app` was left running.

## What the vmtrace shows

`DYLD_INSERT_LIBRARIES` interposed `vm_allocate` / `vm_deallocate` on the
unpatched residency binary (`t_m0_residency-19dd4402484d0b01` from the
`a0e1614` build). Threshold 8388608 bytes. Two `prove_native` cycles, one
process, pid 87834.

Every tracked mapping was freed. At process exit `live_n=0` and
`exit_sum=0`. The 487014400-byte weight buffer (the MTL0 load reports
487.01 MB) was matched by `vm_deallocate` before `after_whisper_free` on
both cycles. The KV and graph buffers (18874368, 56623104, 20840448,
37978112, 78348288, 95698944) were matched in the same burst. The Hy-MT2
scratch buffers (33554432 and 66191360) were matched before
`after_backend_free`.

`task_vm_info` did not follow those frees on cycle 2.

| Point | phys_footprint | virtual_size |
| --- | ---: | ---: |
| cycle 2 `before_load` | 80643032 | 500906934272 |
| cycle 2 `after_whisper_free`, after the 487014400-byte free | 173180032 | 501405335552 |
| cycle 2 `after_backend_free` | 175342720 | 501400748032 |

Virtual size at `after_whisper_free` is 498401280 bytes above `before_load`.
The summary line for that process:

```text
residency_summary pid=87834 measured_cycles=2 catalog_cycles=100 after_1=80610264 after_last=175309952 delta_last_minus_1=94699688 max_after=175309952 min_after=80610264 tolerance_bytes=33554432 within_tolerance=0 prove_predicates=1
```

94699688 > 33554432. Default-zone malloc stayed near 36 MiB on the 100-cycle
run, so the retained bytes are not the default malloc zone. `GGML_METAL_NO_RESIDENCY=1`
still missed. The residency-set keep-alive is not the retainer.

Cycle 1 still drops during the aborted llama load (`after_mt_cancel_init`).
Cycle 2 does not, and later llama load, decode, and `llama_backend_free` do
not give the mapping back. That split was already in the phase table on
`a0e1614`.

## vm_live lines

Weight and graph traffic for both cycles. Backtraces were recorded in the
probe log and are not reproduced here. `kind=v` is `vm_allocate`.

```text
vm_live op=alloc kind=v addr=0x116278000 size=487014400 live_n=1
vm_live op=alloc kind=v addr=0x1332ec000 size=18874368 live_n=2
vm_live op=alloc kind=v addr=0x1344ec000 size=56623104 live_n=3
vm_live op=alloc kind=v addr=0x137f6c000 size=20840448 live_n=4
vm_live op=alloc kind=v addr=0x139438000 size=37978112 live_n=5
vm_live op=alloc kind=v addr=0x13b8f0000 size=78348288 live_n=6
vm_live op=alloc kind=v addr=0x1403a8000 size=95698944 live_n=7
vm_live op=free kind=v addr=0x116278000 size=487014400 matched=0x116278000 matched_size=487014400
vm_live op=free kind=v addr=0x1332ec000 size=18874368 matched=0x1332ec000 matched_size=18874368
vm_live op=free kind=v addr=0x1344ec000 size=56623104 matched=0x1344ec000 matched_size=56623104
vm_live op=free kind=v addr=0x137f6c000 size=20840448 matched=0x137f6c000 matched_size=20840448
vm_live op=free kind=v addr=0x139438000 size=37978112 matched=0x139438000 matched_size=37978112
vm_live op=free kind=v addr=0x13b8f0000 size=78348288 matched=0x13b8f0000 matched_size=78348288
vm_live op=free kind=v addr=0x1403a8000 size=95698944 matched=0x1403a8000 matched_size=95698944
vm_live op=alloc kind=v addr=0x11c7b0000 size=487014400 live_n=1
vm_live op=free kind=v addr=0x11c7b0000 size=487014400 matched=0x11c7b0000 matched_size=487014400
vm_live op=exit live_n=0
vm_live op=exit_sum bytes=0
```

The second block is cycle 2's whisper free. The exit lines are after both
cycles, including the Hy-MT2 frees omitted above.

## Probes that missed the band

Tolerance is 33554432. `within_tolerance` was 0 on every row. `prove_predicates=1`
(`link_three`, `vad_ort`, `gpu_offload`, `teardown`). Silero's execution
provider was still CPU (`ep=CPU`). The engine still printed `ac2=0` and `ac3=0`.

| Probe | Cycles | after cycle 1 | after last | delta | Notes |
| --- | ---: | ---: | ---: | ---: | --- |
| Unpatched binary | 8 | 80364504 | 181896320 | 101531816 | pid 81168. Same pattern as the 100-cycle run: cycle 2 never returns. |
| `GGML_METAL_NO_RESIDENCY=1` | 8 | 82232208 | 171328568 | 89096360 | pid 81473. Log said `use residency sets = false`. |
| `GGML_METAL_RESIDENCY_KEEP_ALIVE_S=1` | 8 | 85607336 | 190497896 | 104890560 | pid 81726. |
| vmtrace, unpatched | 2 | 80610264 | 175309952 | 94699688 | pid 87834. All tracked `vm_allocate`s freed. |
| Synchronize before `whisper_free` buffer release | 8 | 83755968 | 181929064 | 98173096 | pid 96930. Reverted. Not in this tree. |
| Waited 4096-byte blit on the ggml queue after `vm_deallocate` | 8 | 81069016 | 184140952 | 103071936 | pid 98711. Reverted. Not in this tree. The earlier 6-cycle blit after `endResidency` (delta 98812072) stays reverted too. |

The synchronize moved `ggml_backend_synchronize` ahead of the weight-buffer
`vm_deallocate`, and freed the whisper state before those buffers. The blit
ran after `vm_deallocate` on the same `MTLCommandQueue`, then
`waitUntilCompleted`. Neither one moved cycle 8 inside 33554432 of cycle 1.

## Consequence

Repeated `prove_native` teardown is not a bounded-residency result.
`vm_deallocate` returning is not the same as `phys_footprint` returning.
Do not treat any row above as meeting `T-M0-LINK-THREE-MODELS` or the
acceptance.md 100-cycle rule. Do not start M1 on a claim that engine teardown
returns to the cycle-1 baseline.

- [ ] **M0-AC2:** Unchecked. Pins ran in process on MTL0 in these probes. Silero stayed `ep=CPU`. `phys_footprint` did not stay within 33554432 of the cycle-1 teardown.
- [ ] **M0-AC3:** Unchecked. Each prove set `teardown=1`. Resident footprint did not return to the post-first-teardown baseline.
