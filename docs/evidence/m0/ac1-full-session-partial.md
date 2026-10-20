# M0-AC1 full session stays Partial

Status: **Partial**. No Allow was recorded here. No Deny was recorded here. This branch does not run the Mac session and does not replay a Mac log. No button was pressed. Mac was offline.

Cited from PR #77 tip `595dced` (`595dcedafa5e7f2dcc829670e41c462f8ca6dab1`) on `cursor/m0-ac1-full-session-b19b`. That tip chains mic then AudioCapture. #77 owns `ac1_cannot_flip_while_tip_shas_are_none`. This branch cites that guard and does not copy the session script.

PR #51 tip `7c71d50` stays the TIMEOUT Partial. TIMEOUT is not Don't Allow. PR #62 tip `8c24c99` stays the AC1, AC4, and spike-default lock. This note does not replace either lock.

Microphone Allow tip SHA: none
Microphone Deny tip SHA: none
AudioCapture Allow tip SHA: none
AudioCapture Deny tip SHA: none

TIMEOUT is not Deny. A mic TIMEOUT does not start another mic open and is not Don't Allow. An AudioCapture window with no service row is TIMEOUT, not Deny, and does not start another AudioCapture open. A pair-query `denied` with no service row is not Deny. Process exit is not an outcome.

Partial — mic and AudioCapture Allow/Deny tip SHAs are none; consequence AC1 open.

Checking M0-AC1, or replacing this Partial with Done, requires all four lines above to be git tip SHAs from a Mac operator click. `none`, `TIMEOUT`, `Don't Allow`, and `denied` do not fill a line. Either pair still `none` keeps M0-AC1 unchecked. Acceptance is microphone, system, and app tap.

- [ ] **M0-AC1:** microphone, system, and app tap Allow and Deny from Taco.app
