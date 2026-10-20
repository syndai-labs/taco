# M0-AC2 and M0-AC3 stay Partial

Status: Partial.

Cites PR #76 tip `2d9679f` (`2d9679f1cf1d1b083536a91c81ff256756e69cd5`). #76 owns `ac2_ac3_cannot_flip_while_stop_shas_stand`. This note cites that guard and does not replace it. #76 is not an ancestor of measure tip `2766f79`. No acceptance box is checked. Partial is not an accept.

M0-AC2 and M0-AC3 stay unchecked. This tip records no CoreML execution and no Metal device release.

- [ ] **M0-AC2:** open while the Silero CPU Stops below stand.
- [ ] **M0-AC3:** open while the Metal residency Stops below stand.

Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while these Stop tip SHAs stand.

## Silero CPU Stops

| Stop | Tip | Why AC2 stays unchecked |
| --- | --- | --- |
| #37 | `d194012` | CoreML is linked. Silero still runs `device=cpu`. |
| #57 | `88eda06` | ORT 1.31 `GetOpBuilders` lacks `LSTM` and `If`. `device=` stays `cpu`. |
| #60 | `aab661c` | The Silero pin stays. No published Silero ONNX is CoreML-acceptable under ORT 1.31. |
| #64 | `c9a6f6f` | ORT `main` still has no CoreML `LSTM` or `If` builder. |
| #69 | `2ff5c4f` | The VAD stays Silero-only. The charter forbids an alternate VAD pin. |

## Metal residency Stops

| Stop | Tip | Why AC3 stays unchecked |
| --- | --- | --- |
| #32 | `a0e1614` | Residency Stop. The 100-cycle `phys_footprint` misses the band. |
| #49 | `37ff031` | Freeing the weight buffer leaves cycle 2 above cycle 1. |
| #55 | `0829a70` | Residual footprint is the process-lifetime Metal device `ggml_backend_metal_reg`. |
| #63 | `232ea78` | No safe release of that process-lifetime device without a use-after-free. |
| #70 | `e06ea0e` | A Metal device refcount free is still a use-after-free on prove cycle 2. |
| #74 | `30fd669` | A Metal helper process violates single Mach-O, in-process Metal, and LINK-THREE, and it does not close residency. |

## Flip guards

| Guard | Tip | What it locks |
| --- | --- | --- |
| #41 | `a8e2eca` | First consequence freeze. |
| #59 | `ed9ee10` | Consequence for #55 and #57. |
| #65 | `04dda65` | Consequence for #60 and #63. |
| #72 | `f16f0bb` | Consequence for #69 and #70. |
| #76 | `2d9679f` | Consequence for #74. Owns `ac2_ac3_cannot_flip_while_stop_shas_stand`. |

## Out of M0

ORT CoreML LSTM/If upgrade is out of M0. Metal device process-lifetime release is out of M0. Neither is a pass on this tip.

## Prove path

#23 tip `7f98ab1` and #30 tip `e2ec751` closed prove reachable from `taco run` and printed `ac2=0` `ac3=0`. That path does not check M0-AC2 or M0-AC3.
