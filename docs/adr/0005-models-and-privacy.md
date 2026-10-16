# ADR 0005: Explicit model acquisition and offline private sessions

Status: **Proposed**. Validation: M1 artifact lifecycle and negative tests;
M7 network/filesystem observation of the shipped artifact.

## Context and goals

The executable excludes model weights. Users deliberately fetch them once and then
run offline. Audio and transcripts stay local, are not persisted by default, and no
network activity occurs during sessions. These properties must hold for dependency
code, error reporting, and startup paths as well as the happy-path inference loop.

## Non-goals

No model marketplace, arbitrary URL loader, runtime update check, telemetry upload,
online language detection, transcript export/history, encrypted recording store,
account system, or claim to prevent a user from redirecting terminal output.

## Decision

Provide explicit model management commands separate from session commands. Maintain
a versioned allowlisted manifest embedded as metadata in the binary. For each VAD,
ASR, and translation artifact record publisher, immutable upstream revision, exact
filename, expected size and cryptographic digest, format, compatible engine revision,
license/notice links, and logical model ID. All weights, including VAD, are external.
No manifest field is allowed to remain unresolved in a released binary.

`models fetch` is the only network-enabled product command. It takes no URL. It
displays the bundle, total expected bytes, origin, and license notices. Downloads
use HTTPS with certificate verification. Digests are SHA-256 of the exact bytes
plus the expected size. At most three transport attempts may retry a dropped
connection. A digest or size mismatch is not retried. At most three redirects are
followed, and only to hosts named in the manifest. There is no `--insecure` flag,
no range resume, no remote code execution, and no model conversion on the user's
machine. TLS uses rustls or Apple Security.framework, not an OpenSSL or libcurl
dynamic library. A manifest inside the signed executable is the trust root; a hash
downloaded beside the file does not establish it.

Use a private user model directory under macOS Application Support. `--model-dir`
is the only path override, on fetch, verify, and run. Avoid shared temporary
directories. Write a unique partial file on the same filesystem, verify size and
SHA-256, then atomically publish the immutable artifact. Open the published file
without following a symlink. Coordinate concurrent fetches with bounded waiting and
lock ownership that records pid and start time. A stale lock whose pid is dead may
be stolen and may delete a partial file; it must not delete a published artifact.
Interrupted downloads restart from the beginning and are never reported as ready.

At session preflight, open and verify the exact model files and compatibility before
capture begins. Pass verified handles into the session. The session type has no
fetch client and no URL parameter. If an adapter can only load by path, M0/M1 must
keep the verified inode open or use an equivalent race-free local boundary and
record that limit. Do not re-download inside that boundary.
Reject unexpected file types and symlink escapes from managed entries. A missing,
corrupt, or incompatible model gives the explicit fetch/repair instruction and exits;
it does not call the network. Never delete a known-good artifact to repair a failed
new download. Cache management cannot remove artifacts in use by a session.

## Privacy boundary

Session code has no HTTP client or updater capability. Audit native engines for
remote model resolution, RPC backends, environment-triggered remote behavior, and
logging callbacks; disable those paths in the build and adapter configuration.
Help, version, notices, source listing, model listing, verification, and sessions
are offline. Only explicit fetch may contact the allowlisted origin or CDN hosts.

Default logs contain bounded operational metadata and reason codes, never raw audio,
transcript text, translation text, full prompts, or native tensor dumps. Avoid
user-sensitive device/app names in persistent diagnostics. Model caches contain
weights and non-content manifests only. Runtime metrics are bounded in memory and
shown on request; persistent benchmark traces use synthetic/licensed consented
fixtures under a separate developer harness, not hidden production recording.

Terminal output is an intentional local disclosure. Shell redirection and terminal
scrollback can retain it outside taco's control; explain this in user help. The
application does not promise secure erasure from OS swap, third-party terminal logs,
or system crash reports. It must not create its own content-bearing crash uploads or
persist session content on success, cancellation, or ordinary errors.

## Alternatives and consequences

Bundled weights would simplify setup but violate the external-weight charter choice.
Implicit downloads improve first-run convenience but violate the explicit network
boundary. Per-session integrity checking adds cold-start cost, reported separately
from subtitle latency; cache verification cannot be skipped by trusting a filename.

## Acceptance criteria

- [ ] Interrupted fetch, bad digest/size, unexpected redirect, full disk, unwritable
  path, concurrent fetch, path substitution, and stale partial files never produce a
  ready artifact or damage an existing good copy.
- [ ] Every non-fetch command makes zero attempted network connections/DNS requests,
  including missing-model and error paths, with the network available and observed.
- [ ] Sessions also succeed with the network unavailable after explicit acquisition.
- [ ] Filesystem and log observation using sentinel fixture text finds no taco-owned
  audio/text persistence outside the requested terminal stream.
- [ ] Each model has separately recorded usage/redistribution/license obligations;
  the engine's license is never used as a substitute for the model's license.
