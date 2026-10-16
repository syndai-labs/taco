# Architecture decisions

These decisions implement the direction in the [taco charter](../../CHARTER.md).
The repository is greenfield. **Proposed** means a recommended design awaiting the
specified implementation evidence; it does not mean that software exists, that a
dependency combination works, or that the user has approved a product change.

| ADR | Decision | Status | First validation |
| --- | --- | --- | --- |
| [0001](0001-process-and-pipeline.md) | One process, replaceable stages, bounded data flow | Proposed | M0, then M2–M4 |
| [0002](0002-audio-capture.md) | Explicit source selection and native macOS capture | Proposed | M0, M2, M5 |
| [0003](0003-segment-contract.md) | Bounded utterances and immutable transcript commits | Proposed | M0, M3 |
| [0004](0004-local-inference.md) | Pinned native models and controlled Metal scheduling | Proposed | M0, M3–M4 |
| [0005](0005-models-and-privacy.md) | Explicit acquisition and offline session boundary | Proposed | M1, M7 |
| [0006](0006-session-and-terminal.md) | Supervised lifecycle and shared CLI/TUI state | Proposed | M2–M6 |
| [0007](0007-qualification.md) | Hardware-specific quality, latency, and release gates | Proposed | M0, M7 |

## Decision rules

The charter's core stack and invariants are inputs, not decisions silently reopened
by this plan. **Proposed ADRs bind implementation.** Evidence may force a recorded
revision; it does not let an agent treat the ADR as a suggestion. Minimum OS,
quantization, operating budgets, and the qualified language pairs stay open only
where this plan names an M0 experiment. A failed feasibility gate blocks dependent
milestones. Record the finding and revise the ADR deliberately; do not substitute a
cloud API, daemon, different model family, or narrower source scope and call v0.1
complete.

Implementers also follow the [agent contract](../roadmap/agent-contract.md) and the
[test catalog](../roadmap/test-catalog.md). Those files constrain how the ADRs are
built. They do not override the charter or an ADR.

When accepting an ADR, add the approving role, date, concrete pinned versions or
artifacts where relevant, and links to evidence. Keep later changes in a new ADR
that supersedes the earlier decision. An accepted design still needs runtime and
release qualification; it is not a test result.

See the [roadmap](../roadmap/README.md), [acceptance plan](../roadmap/acceptance.md),
and [decision register](../roadmap/risks-and-decisions.md) for delivery dependencies.
