# Planning work and review record

This checklist tracks **documentation work**, not implementation completion. The
authorized write scope is `docs/adr/` and `docs/roadmap/` only. The charter remains
unchanged. Product milestone checkboxes live in the individual milestone files.

## Plan

- [x] Establish repository identity and inspect the charter and current state.
- [x] Present an initial architecture graph and ask five clarification questions.
- [x] Record a failing baseline for missing planning artifacts.
- [x] Research primary sources for the platform and native model integrations.
- [x] Write proposed architecture decisions with alternatives and validation gates.
- [x] Write dependency-ordered milestones with goals, non-goals, and acceptance criteria.
- [x] Map every charter scope item, invariant, and completion condition to evidence.
- [x] Independently review feasibility, lifecycle, quality, and release claims.
- [x] Resolve review findings and verify document consistency and local links.
- [x] Verify that all changes are inside the two authorized directories.

## Baseline

On 2026-10-16 the checkout at `/Users/jwu/workspace/syndai/cap` had no commits,
no remote, and no source or build manifest. Its only project file was an untracked
`CHARTER.md` titled **taco — Charter**. No project-specific parent `AGENTS.md` was
found. The user supplied workflow instructions in the conversation.

The initial presence check failed for both documentation indexes, the acceptance
plan, and this review record. Those failures establish a documentation baseline;
they are not failing executable product tests. Future milestones specify the
behavioral tests that must fail before their implementations are written. No source,
test suite, build configuration, `tasks/` file, or model weight is added by this work.

Original charter SHA-256:
`4aaa9bf0e8561eed164a1190d210f961593ec9580c69d58ce15ed89d16119b6b`.

## Assumptions used to complete the plan

The charter resolves the `cap` directory versus `taco` product naming discrepancy.
The five clarification questions concern repository identity, first user/release,
fixed constraints, milestone dates, and treatment of unresolved decisions. In the
absence of additional answers, this plan follows the charter, uses capability gates
instead of invented dates, and records unvalidated choices as proposed ADRs.

The initial user is a person who wants to read live bilingual subtitles from one
local microphone/input, the system output, or a selected application's output in a
terminal. One source language and one target language are fixed for each session.
The north star does not expand v0.1 into simultaneous bidirectional conversation,
speech output, or a universal audio routing platform.

## Review results

Architecture review of the execution plan, 2026-10-16. No product milestone is
implemented or qualified by these documents. The charter file was not edited.
Its SHA-256 is unchanged from the baseline above.

The review treated goals, non-goals, and acceptance criteria as the contract a
later agent will actually follow. Findings below were fixed in the plan. They are
not open work.

| Finding | Why it would drift an implementation | Where it is fixed |
| --- | --- | --- |
| “Proposed” read as optional | An agent could replace the stack while claiming the ADR was only a draft | ADR index and agent contract: proposed ADRs bind until a recorded revision |
| M0-AC6 did not require replaceable stages, while coverage cited it for invariant I7 | I7 could be marked done from an unrelated checkbox | M0-AC6, M3-AC6, M4-AC7, and the fake-adapter tests |
| M4-AC6 was mapped to replaceability | Slow-output behavior was being used as proof that stages can be swapped | Coverage now says M4-AC6 is only the slow-output gate |
| 15-minute smokes were judged by the 60-minute slope and p99 rules | M4 or M5 could claim release, or block forever on an hourly fit | Separate smoke gate; M7 owns the 16-cell matrix |
| The previous hourly slope allowance contradicted the “no leak” sentence | Steady growth of about 120 MiB over the hour would have passed | Slope band, inventory attribution, and the 100-cycle footprint check |
| Release matrix was four axes implied at once, including human review of both UIs | Agents would either waive cells or fake an impossible cross product | One 16-cell rotation; human review follows the ledger, not the renderer |
| Hardware and OS were called one “profile,” and the CLI had `--profile` | Model bundles, qualification corners, and flags would collapse into one concept | Terms table; one model bundle; no `--profile` flag |
| Fetch tests rejected every redirect, while the ADR allowed approved redirects | Either open redirects or a fetcher that cannot reach a CDN | Allowlisted, counted redirects; SHA-256; no range resume |
| Job deadlines of 10s plus 10s could be treated as the latency SLO | The pipeline would “pass” timeout settings and miss p99 | Deadlines are hang bounds; `L_total` stays speech-end to presentation |
| Shutdown “10s with 2s reserve” could be implemented as 12s | Lifecycle tests would time out differently in each milestone | Drain budget is the first 8s; the last 2s are join reserve |
| Loss events lived only on a bounded channel | A full status queue could hide a capture gap | Wait-free loss counters; status drops do not drop commits or outcomes |
| Degraded could return to Running and clear the failure | A tainted hour could exit 0 | Sticky taint; exit table in ADR 0006 |
| Empty ASR output and draft IDs were unspecified | Empty strings would be translated, or the UI would keep two rows | One id; `empty_decode` is not a commit |
| Upstream Whisper VAD defaults and Hy-MT2’s CPU / 4096-token / extreme-quant examples contradict the charter | A careful agent following the model card would build the wrong system | ADR 0004 and the model-card traps in the agent contract |
| Held-out speech could be used to pick the quantization | The release score would be a training score | Seal held-out labels before the M0 comparison; M7 is the only held-out scorer |
| M2 acceptance cited the full quality rubric | The capture milestone would grow an ASR project, or claim WER it cannot measure | M2 owns endpoint coverage only |

Every milestone still has one goal, explicit non-goals, and unchecked acceptance
criteria. Non-goals do not cancel a charter requirement. In particular, system and
application capture stay mandatory, smoke runs do not fill release cells, and
identical source and target languages stay out of scope.

68 catalog tests are named, each owned by one milestone. Default-tier tests must
fail for the missing behavior before implementation. Manual and native tests are
not part of plain `cargo test`. An implementing change does not check milestone
boxes.

## Verification of this review

- Local Markdown links under `docs/` resolve.
- Every `T-…` id in the catalog is listed in its milestone, and every milestone
  test id exists in the catalog.
- Acceptance criteria M0-AC1–AC6, M1-AC1–AC6, M2-AC1–AC6, M3-AC1–AC6,
  M4-AC1–AC7, M5-AC1–AC6, M6-AC1–AC6, and M7-AC1–AC6 are defined once.
- Charter coverage still has a row for each v0.1 scope item, each invariant, and
  the done condition.
- No source, package manifest, or model artifact was added.
- Write scope remained `docs/adr/` and `docs/roadmap/`.
