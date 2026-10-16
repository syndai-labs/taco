# M6 — Complete CLI and TUI sessions

Status: **Not started**. Depends on: M4; M5 required for full-source acceptance.
Accountable: terminal UX owner; reviewers: runtime and accessibility/quality owners.
Decision: [ADR 0006](../adr/0006-session-and-terminal.md).

## Goal

Make the install/fetch/select/run flow understandable in a terminal, with clear
bilingual association, pending/error status, and reliable restoration after stopping
or failing. Finish the public command contract without changing pipeline semantics.

## Non-goals

No GUI, transcript history/search/export, recording, live source/language switching,
custom terminal theme system, machine-readable API promise, or TUI-only feature that
is required to use basic translation.

## Work packages

1. Finish the frozen Clap command grammar, help, version/build identity, source/model
   listings, actionable preflight errors, and explicit language/source selection.
2. Complete append-only CLI output with stdout/stderr separation and non-TTY behavior.
   Show source and terminal translation outcomes with stable segment association.
3. Implement Ratatui over the same bounded state: source, language pair, readiness,
   current draft, committed subtitles, pending target, and degradation/error counters.
4. Complete keyboard stop, interrupt, resize, terminal sanitation, bounded writer,
   lifecycle summaries, terminal restoration, and user help for privacy/permissions.

## Tests to write first

Write these [catalog](test-catalog.md) tests. Parser and PTY cases are default
tier. Real terminal hosts are manual.

- `T-M6-CLI-GRAMMAR`
- `T-M6-EXIT-PRECEDENCE`
- `T-M6-LEDGER-ON-STDOUT`
- `T-M6-SANITIZE-TERMINAL`
- `T-M6-RESTORE-TTY`
- `T-M6-SLOW-SINK`
- `T-M6-TUI-EVICT-PENDING-LAST`
- `T-M6-TAINT-STICKY`
- `T-M6-HOSTS`

## Acceptance criteria

- [ ] **M6-AC1:** A user following help alone can explicitly fetch models, discover and
  select each supported source category, select a qualified language direction, and
  run CLI or TUI without undocumented environment variables or sidecar programs.
- [ ] **M6-AC2:** `T-M6-TUI-EVICT-PENDING-LAST` keeps source and translation associated
  under delayed translation, failures, draft updates, and row eviction. CLI and TUI
  report the same outcome multiset. Pending or failed is not shown as an empty
  success, and counters retain evicted outcomes.
- [ ] **M6-AC3:** Non-TTY CLI output is append-only and readable, status uses stderr,
  and TUI fails cleanly without a terminal. Unicode/control-sequence tests pass.
- [ ] **M6-AC4:** Normal stop, Ctrl-C, second interrupt, source/model failure, broken
  pipe, and deadline failure preserve the documented exit semantics and restore local
  terminal modes within the shared shutdown budget. Responsive sinks also restore
  cursor/alternate screen; blocked sinks have bounded best-effort visual restoration
  with an explicit failure outcome.
- [ ] **M6-AC5:** UI rows/text and writer buffers remain bounded; a slow terminal
  cannot block audio callbacks or supervisor cancellation. Presentation latency is
  included in metrics and delivery failures are visible.
- [ ] **M6-AC6:** `T-M6-HOSTS` covers Apple Terminal and the named alternate as the
  [acceptance matrix](acceptance.md) assigns them, including the four short
  host-swap sessions. Keyboard-only use works, and source, target, and failure
  remain distinguishable with color removed.

## Evidence and handoff

Deliver CLI contract/PTY test results, terminal restoration observations, manual UX
check results, and final command/help examples. All four source categories and both
UIs are now feature-complete, but the product remains unreleased until M7 qualification.

The acceptance plan's named alternate is the launchd TCC launch parent in
[os-floor.md](../evidence/m0/os-floor.md). It is not a second GUI terminal.
`T-M6-HOSTS` has no measured second presentation host. M6-AC6 stays open.
