# M0-AC1 stays Partial

Status: Partial.

Stacked on PR #88 tip `2766f79` (`2766f79021c52051e8661b993c072327e9761cb6`). Base is PR #87 tip `a40fad1` (`a40fad173091a605417766e2a28433548a0481f6`). #87 owns `ac1_cannot_flip_while_full_session_tip_shas_are_none` and `process_tap_tips_refuse_ac1_flip_while_either_sha_is_none` on this measure tip. #88 owns `external_input_unqualified_refuses_ac1`. This note cites those guards and does not replace them. No acceptance box is checked. Partial is not an accept.

M0-AC1 stays unchecked. A Mac operator session is still required. This tip records no Allow, no Deny, and no Cleanup.

Allow tip SHA: none
Deny tip SHA: none
Cleanup tip SHA: none

Microphone Allow and Deny tip SHAs in `mic-operator-resume.md` are none. AudioCapture Allow and Deny tip SHAs in `audiocapture-measure.md` are none. Session Allow, Deny, and Cleanup tip SHAs in `ac1-full-session.md` are none. Process-tap Allow, Deny, and Cleanup tip SHAs in `process-tap-product.md` are none. This note does not invent an Allow tip SHA, a Deny tip SHA, or a Cleanup tip SHA.

TIMEOUT is not Deny. PR #51 tip `7c71d50` (`7c71d509b404de536bb4a10b60c254d382824403`) stays that Partial. A pair-query `denied` with no service row is not Deny. Process exit is not an outcome.

Cleanup needs a separately traced destroy. The log has to quote both `AudioHardwareDestroyAggregateDevice` and `AudioHardwareDestroyProcessTap`. Exit 0 is not a Cleanup tip SHA. Drop is not a Cleanup tip SHA.

## Locks this note does not replace

| Lock | Tip | What stays |
| --- | --- | --- |
| #51 | `7c71d50` | TIMEOUT is not Deny. |
| #62 | `8c24c99` | Owns `ac1_cannot_flip_while_operator_timeout_partial_stands`. Not an ancestor of this tip. Not copied. |
| #81 | `ca9bc50` | Owns `ac1_cannot_flip_while_full_session_tip_shas_are_none`. |
| #82 | `637459f` | Owns `process_tap_tips_refuse_ac1_flip_while_either_sha_is_none`. |
| #87 | `a40fad1` | Those guards on the measure tip. Tip SHAs stay none. |
| #88 | `2766f79` | `external_input_unqualified_refuses_ac1`. `qualified_in_m0` stays false. |

`595dced`, `fb48acc`, `cd61eaf`, `dc1b9be`, `cd9a4c8`, `a40fad1`, and `2766f79` are harness commits. They are not operator tip SHAs.

- [ ] **M0-AC1:** microphone, system, and app tap Allow and Deny, plus a separately traced process-tap cleanup. Open while the tip SHA fields above are `none`.
