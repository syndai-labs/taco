# M0-AC4 stays Partial

Cited from `cursor/m0-ac4-partial-freeze-b19b` tip `6a839de` (PR #54). That tip is stacked on PR #42 tip `8794303` (`879430322fceb8a0bb15bc26c1b201d1b105a5c1`), the tip among #42, #43, and #51 that already carries the R11 `SIGNED_INSTALL=None` guards. The other tips below are cited. This branch does not replay `freeze.rs`, corner strings, launchd scripts, or a Metal load. `C2_BLOCKED` stays on PR #54 and is not copied here. No Metal fake.

No Metal load is recorded here. M0-AC4 stays unchecked. Partial is not an accept.

- [ ] **M0-AC4:** open while `SIGNED_INSTALL=None` or `C2_BLOCKED` stands.

Charter accept is blocked: M0-AC4 cannot be checked while SIGNED_INSTALL=None or C2_BLOCKED stands.
The launchd user-agent run does not close M0-AC4.

PR #52 tip `05a34ab` cites #49 tip `37ff031` and #47 tip `ce5690f`. PR #53 tip `f634977` cites #50 tip `fdc201f` and #51 tip `7c71d50`. Neither cite checks an M0-AC box.

## Gaps

| Gap | Tip | What it recorded | Why M0-AC4 stays open |
| --- | --- | --- | --- |
| R11 signed-install Stop | PR #38 `e7433c2` (`e7433c22f55ab9a94f24ee33c75147ecb35d7520`) | No Developer ID Application identity. `notarytool` had no `taco-notary` profile. `SIGNED_INSTALL=None`. Ad-hoc `codesign` is not an install. | No notarized install artifact. Weight distribution stays blocked. |
| None guards | PR #42 `8794303` (`879430322fceb8a0bb15bc26c1b201d1b105a5c1`) | Charter guards refuse a filled `SIGNED_INSTALL` while the Stop phrases remain. | The Stop cannot be checked away in prose. |
| Second machine | PR #27 `ad8d418` (`ad8d418610d22a8302c73f3cc8b990be1c1fd224`) | `C2_BLOCKED` is true on PR #54. No second Apple Silicon Mac. `os_build` unset. | Dual-machine acceptance cannot close. This branch does not replay `C2_BLOCKED` and does not rename C1. |
| OS floor | PR #19 `8ebd0fc` (`8ebd0fc31d7bfe25031ee6b73d259011202ec6f4`) | Floor revised to the only measured build, macOS 27.0.1 (26A434). Status Partial. Launchd named, not exercised on that tip. | Not a support promise. macOS 14.2 was not measured. |
| Launchd host | PR #43 `0c880fb` (`0c880fb8e0014c3e7ad8d6743fd6d950f0763f04`) | User agent `ai.syndai.taco.m0-launch-host` loaded, ran `taco notices`, and was removed. `Signature=adhoc`. `TeamIdentifier=not set`. | An ad-hoc launchd run is not a signed install and does not close M0-AC4. Not a TCC allow or deny. |

## Consequence

M0-AC4 stays open. Checking the box requires `SIGNED_INSTALL` to be a real notarized install id and `C2_BLOCKED` to be false because a different Apple Silicon Mac was measured. Clearing only one of those two still leaves the box unchecked. The launchd user-agent run does not close M0-AC4.
