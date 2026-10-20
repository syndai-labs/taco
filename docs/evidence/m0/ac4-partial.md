# M0-AC4 stays Partial

Cited from PR #54 tip `6a839de` (`6a839deeeacbe2eff987844c9bb322c2ed3d7d6a`). This branch does not replay a signed install or a second machine. No Metal load is recorded here. M0-AC4 stays unchecked. Partial is not an accept.

- [ ] **M0-AC4:** open while `SIGNED_INSTALL=None` or `C2_BLOCKED` stands.

Charter accept is blocked: M0-AC4 cannot be checked while SIGNED_INSTALL=None or C2_BLOCKED stands.
The launchd user-agent run does not close M0-AC4.

`freeze::SIGNED_INSTALL` is `None`. `freeze::C2_BLOCKED` is true. C2 still has no chip, memory, or OS build.

| Gap | Tip | Why M0-AC4 stays open |
| --- | --- | --- |
| R11 signed-install Stop | PR #38 `e7433c2` | No Developer ID. `SIGNED_INSTALL=None`. Ad-hoc `codesign` is not an install. |
| None guards | PR #42 `8794303` | `SIGNED_INSTALL` stays `None` while the Stop record stands. |
| Second machine | PR #27 `ad8d418` | `C2_BLOCKED`. No second Apple Silicon Mac. |
| OS floor | PR #19 `8ebd0fc` | Partial macOS 27.0.1 (26A434). Not a support promise. |
| Launchd host | PR #43 `0c880fb` | Ad-hoc load/run/teardown. `Signature=adhoc`. Not a signed install. |

Checking the box requires `SIGNED_INSTALL` to be a real notarized install id and `C2_BLOCKED` to be false because a different Apple Silicon Mac was measured. Clearing only one of those two still leaves the box unchecked. The launchd user-agent run does not close M0-AC4.
