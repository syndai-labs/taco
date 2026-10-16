# M7 — Offline endurance and distributable release qualification

Status: **Not started**. Depends on: M1–M6.
Accountable: release owner; reviewers: capture, inference, quality, and privacy owners.
Decision: [ADR 0007](../adr/0007-qualification.md).

## Goal

Prove the charter's done condition against the exact artifact a user would install:
one executable, explicit model acquisition, one selected source, accurate bilingual
subtitles, and a 60-minute offline session without crashes or unbounded memory growth.

## Non-goals

No new product capability, model family change, broader OS/hardware/language claim,
performance claim extrapolated from newer hardware, or publishing before the release
gate is complete. This planning task neither implements nor publishes a release.

## Work packages

1. Freeze the release candidate, build provenance, manifest/model digests, prompt and
   decoder settings, support matrix and qualification corpus. Any relevant fix changes
   artifact identity and invalidates affected evidence; rerun the affected gates.
2. Produce one arm64 Mach-O named `taco`, with code and shaders embedded and with
   system-framework dependencies only. A zip or disk image may wrap that file so a
   notarization ticket can travel with it, and the archive may include licenses and
   a readme. The archive must not add a second executable, an app helper, a dylib,
   or a shader the binary needs at runtime. Test the downloaded and quarantined
   Mach-O on a clean user and machine without developer tools or a preinstalled
   inference runtime.
3. Execute the full [acceptance matrix](acceptance.md): deterministic contracts, native
   failures, source/permission isolation, UI lifecycle, joint quality/latency/memory,
   offline operation, no-network-attempt observation, and no default content persistence.
4. Review the report against [charter coverage](charter-coverage.md), reproduce the
   install-to-subtitles path, document supported limits and explicit recovery steps,
   and make a go/no-go decision with named reviewers and evidence links.

## Tests to write first

Write these [catalog](test-catalog.md) tests before treating any soak as evidence.
`T-M7-FAIL-CLOSED` must reject a forged pass. The matrix runner does not infer a
pass from a missing cell.

- `T-M7-FAIL-CLOSED`
- `T-M7-MATRIX`
- `T-M7-HELDOUT-ONCE`

## Acceptance criteria

- [ ] **M7-AC1:** A clean installation of the exact signed/distributed executable needs
  no compiler, package manager, Python, inference server, extra code file, or virtual
  driver. Explicit model fetch is the only required product network step; existing
  verified models support a fully offline start.
- [ ] **M7-AC2:** Every release-matrix cell and the attached human-review rules in the
  [acceptance plan](acceptance.md) pass together on the held-out corpus and the
  candidate digest. Smoke runs and development-corpus scores do not fill a cell.
- [ ] **M7-AC3:** Fault and soak evidence establishes bounded queues/context, no sustained
  resource growth, reliable cancellation/cleanup, and no uncontrolled crash. No required
  failure case is omitted because it is inconvenient to automate.
- [ ] **M7-AC4:** Available-network observation shows zero session network attempts,
  including startup/errors; disconnected runs succeed. Content-sentinel observation
  finds no taco-owned audio/transcript/prompt persistence outside the requested terminal.
- [ ] **M7-AC5:** Release metadata, executable/model digests, OS/hardware names, corpus,
  decoder settings, score denominators, stage timing, raw aggregate measurements,
  failures, and reviewer decisions form a reproducible evidence bundle without private
  user audio or transcripts.
- [ ] **M7-AC6:** Installation, permissions, source selection, model repair, known limits,
  offline/privacy behavior, and normal/error stopping are documented and verified from
  help/runbook instructions. No failed or unrun mandatory gate is waived implicitly.

## Release decision and later work

Release only when every criterion above passes. Mark failed/unrun cells explicitly,
retain the candidate identity, and return to the owning milestone. Requalify after
model, prompt, decoder, capture, native runtime, packaging, or lifecycle changes
according to their affected gates; do not reuse stale exact-artifact results.

After v0.1, consider richer language qualification, improved latency, and then
separately chartered synchronized outputs/bidirectional interpretation. TTS, mixing,
history, GUI, and cross-platform work require new scope decisions. No post-v0.1 item
is a dependency that delays a fully qualified v0.1 release.
