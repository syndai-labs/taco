# M1 — Verified models and offline preflight

Status: **Not started**. Depends on: M0.
Accountable: model lifecycle owner; reviewer: runtime/privacy owner.
Decision: [ADR 0005](../adr/0005-models-and-privacy.md).

## Goal

Let a user deliberately acquire the three required model artifacts and reliably
start later sessions offline. Make cache readiness, integrity, compatibility, and
repair behavior observable before any audio is captured.

## Non-goals

No inference UX, model marketplace, arbitrary URL support, background update checks,
download during a session, model conversion, HTTP range resume, or automatic
eviction of in-use weights. Restart an interrupted artifact from the beginning.
Atomic publication is required. A harness forward-pass is not a user translate
command.

## Work packages

1. Implement the pinned manifest, private cache layout, local model listing and
   verification, and the explicit fetch command from the M0 command contract.
2. Implement bounded fetch, digest/size checking, safe partial paths, per-artifact
   locking, atomic publication, and actionable failures. Keep good entries usable
   after failed upgrades; model identity includes digest and engine compatibility.
3. Implement session preflight that validates configuration and verified local handles
   without any network capability. Initialize each selected model once per session;
   fail before capture if it is absent, incompatible, or cannot be allocated.
4. Audit logging and dependency download/telemetry paths. Include all model/runtime
   notices in the planned distribution without embedding weights in the binary.

## Tests to write first

Write these [catalog](test-catalog.md) tests against a local fake transport and a
temporary cache. No default-tier test contacts a public model host.

- `T-M1-PUBLISH-ATOMIC`
- `T-M1-REDIRECT-POLICY`
- `T-M1-KEEP-GOOD`
- `T-M1-OFFLINE-COMMANDS`
- `T-M1-SESSION-TAKES-HANDLES`
- `T-M1-LOAD-ONCE`
- `T-M1-NO-CONTENT-LOG`
- `T-M1-NOFOLLOW-SYMLINK`

## Acceptance criteria

- [ ] **M1-AC1:** Explicit fetch publishes only complete verified artifacts; every
  interrupted/error case leaves either the previous good model or a clearly absent
  model, never a partial ready entry. Concurrent fetches converge without corruption.
- [ ] **M1-AC2:** Model list/verify/help/version and all preflight failures make zero
  network attempts. Missing models name the explicit remediation command and exit
  before permission or audio initialization.
- [ ] **M1-AC3:** After acquisition, one native load of the pinned bundle succeeds with
  connectivity unavailable. That load is a harness check, not a translate command.
  Observed fetch destinations match the manifest host policy.
- [ ] **M1-AC4:** A model load counter proves one initialization per selected model per
  session; cache operations cannot invalidate active handles. A new session may reload.
- [ ] **M1-AC5:** The exact model/license/format/runtime metadata is inspectable, paths
  with spaces/non-ASCII work, and error/log fixtures contain no session text or audio.
- [ ] **M1-AC6:** Cache operations stay within user-owned managed paths and leave no
  unexpected content in shared temporary or working directories.

## Evidence and handoff

Deliver cache state-transition test results, fault-injection results, a real pinned
fetch/load report, and network/filesystem observations. M2 receives verified model
handles and a preflight result; it does not receive a downloader. This milestone is
not proof of 60-minute session privacy, which remains an M7 gate.
