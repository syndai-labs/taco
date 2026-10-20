# M0-AC4 stays Partial

Status: Partial.

Joined onto PR #94 tip `fe08f47` (`fe08f47484d903fe33a8fa76635fd5e9e9dc4b8e`). Source is PR #54 tip `6a839de` (`6a839deeeacbe2eff987844c9bb322c2ed3d7d6a`), which is not an ancestor of this tip. #54 was stacked on PR #42 tip `8794303` (`879430322fceb8a0bb15bc26c1b201d1b105a5c1`). The other tips below are cited. This branch does not replay their corner strings, launchd scripts, signed-install evidence file, or Metal runs.

No Metal load is recorded here. M0-AC4 stays unchecked. Partial is not an accept. This tip does not claim a Developer ID or a second machine.

- [ ] **M0-AC4:** open while `SIGNED_INSTALL=None` or `C2_BLOCKED` stands.

Charter accept is blocked: M0-AC4 cannot be checked while SIGNED_INSTALL=None or C2_BLOCKED stands.
The launchd user-agent run does not close M0-AC4.

## Gaps

| Gap | Tip | What it recorded | Why M0-AC4 stays open |
| --- | --- | --- | --- |
| R11 signed-install Stop | PR #38 `e7433c2` (`e7433c22f55ab9a94f24ee33c75147ecb35d7520`) | No Developer ID Application identity. `notarytool` had no `taco-notary` profile. `SIGNED_INSTALL=None`. Ad-hoc `codesign` is not an install. | No notarized install artifact. Weight distribution stays blocked. |
| None guards | PR #42 `8794303` (`879430322fceb8a0bb15bc26c1b201d1b105a5c1`) | Charter guards refuse a filled `SIGNED_INSTALL` while the Stop phrases remain. | The Stop cannot be checked away in prose. |
| Second machine | PR #27 `ad8d418` (`ad8d418610d22a8302c73f3cc8b990be1c1fd224`) | `C2_BLOCKED` is true. No second Apple Silicon Mac. `os_build` unset. | Dual-machine acceptance cannot close. This branch sets `C2_BLOCKED` and does not rename C1. |
| OS floor | PR #19 `8ebd0fc` (`8ebd0fc31d7bfe25031ee6b73d259011202ec6f4`) | Floor revised to the only measured build, macOS 27.0.1 (26A434). Status Partial. Launchd named, not exercised on that tip. | Not a support promise. macOS 14.2 was not measured. |
| Launchd host | PR #43 `0c880fb` (`0c880fb8e0014c3e7ad8d6743fd6d950f0763f04`) | User agent `ai.syndai.taco.m0-launch-host` loaded, ran `taco notices`, and was removed. `Signature=adhoc`. `TeamIdentifier=not set`. | An ad-hoc launchd run is not a signed install and does not close M0-AC4. Not a TCC allow or deny. |

## Consequence

M0-AC4 stays open. Checking the box requires `SIGNED_INSTALL` to be a real notarized install id and `C2_BLOCKED` to be false because a different Apple Silicon Mac was measured. Clearing only one of those two still leaves the box unchecked. The launchd user-agent run does not close M0-AC4.
