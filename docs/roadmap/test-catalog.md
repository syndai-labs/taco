# Test catalog

These IDs are the tests that keep an implementation on the charter. The
assertion under each ID is normative. Tiers and evidence rules live in the
[agent contract](agent-contract.md). Numeric caps live in
[acceptance.md](acceptance.md); this catalog repeats a number only to make the
assertion auditable.

Show each default-tier test failing for the missing behavior before the
implementation that makes it pass. Manual scripts are repository files named
`scripts/qual/<id>`, not a paragraph copied into a report.

## M0 — contracts and feasibility

### T-M0-REJECT-INPUTS

Tier: default. AC: M0-AC6.

Reject, before capture allocation: two sources in one invocation; a bare device
name or bundle id; `stdin` or a filesystem path as a source; identical `--from`
and `--to`; a tag outside the frozen table; source or target text over the byte
cap; a second terminal outcome for one segment id; a transition that skips the
session state machine. Each case names the closed error code.

### T-M0-LIMIT-VECTORS

Tier: default. AC: M0-AC4, M0-AC6.

One fixture lists every numeric cap from the acceptance plan, including queue
depths, the 15-second model window, pre-roll, endpoint silence, token budget,
retention, shutdown arithmetic, and fetch retry/redirect limits. Later tests
import this fixture. It contains no model weights.

### T-M0-LINK-THREE-MODELS

Tier: native. AC: M0-AC2, M0-AC3.

Load the pinned VAD, multilingual Whisper small, and Hy-MT2 GGUF in one process,
run one ASR pass and one translation pass, and read back the active GPU backend.
Fail on a subprocess, a CPU-only configuration, a missing shader loaded from a
sibling file, or a digest other than the pin. Repeat initialization and teardown
for several cycles; resident native allocations return to the post-first-teardown
baseline within the cycle tolerance.

### T-M0-CANCEL-LIFECYCLE

Tier: native. AC: M0-AC3.

Cancel during model initialization and during an active Metal decode. The worker
joins inside the cancellation reserve, late completions are ignored, and no
replacement worker is spawned. A wrapper timeout with a still-running thread fails
the test.

### T-M0-ONE-MACHO

Tier: default once a binary exists; the spike evidence is manual. AC: M0-AC2, D2.

Inspect the artifact: one Mach-O, no non-system dynamic dependency, no adjacent
`.metal`, `.so`, `.dylib`, model weight, or runtime helper required to launch.
Embedded usage strings for microphone and audio capture are both present.

### T-M0-SPIKE-SOURCES

Tier: manual. AC: M0-AC1.

From the signed spike, record allow and deny for microphone, system output, and
one application, then cleanup. Reserve a named external input device for M2; do
not claim that device was qualified here.

## M1 — models and offline preflight

### T-M1-PUBLISH-ATOMIC

Tier: default. AC: M1-AC1.

Against a local fake transport and a temporary cache: truncated body, extra
bytes, wrong SHA-256, size mismatch, full disk, read-only destination, timeout,
stale lock, crash before rename, and a non-allowlisted host. None of these leaves
a ready artifact. A ready artifact's bytes match the manifest digest.

### T-M1-REDIRECT-POLICY

Tier: default. AC: M1-AC1, M1-AC3.

Allow at most the frozen number of redirects, and only to manifest-listed hosts.
Reject an open redirect, an HTTP downgrade, a redirect loop, and TLS verification
disabled. There is no insecure flag to turn this test off.

### T-M1-KEEP-GOOD

Tier: default. AC: M1-AC1.

A failed or concurrent fetch does not replace a previously good artifact.
Concurrent fetches of the same artifact converge to one verified file. A stolen
stale lock may delete a partial file and must not delete a published file.

### T-M1-OFFLINE-COMMANDS

Tier: default, plus one manual observation with the network up. AC: M1-AC2, P1.

`help`, `version`, `notices`, `sources list`, `models list`, `models verify`, and
every preflight failure perform zero fetch-client calls. The manual run observes
zero process-attributed DNS or connection attempts while connectivity exists.
A missing model names `taco models fetch` and exits before permission or audio
initialization.

### T-M1-SESSION-TAKES-HANDLES

Tier: default. AC: M1-AC2, I3.

The session constructor accepts verified handles and language/source
configuration only. It has no parameter for a URL, an HTTP client, or the fetch
command.

### T-M1-LOAD-ONCE

Tier: native. AC: M1-AC4, I4.

A session initializes each selected model once. Cache repair and verify in
another command do not invalidate those live handles. A later process may load
again. The test-harness forward pass is not a user-facing translate command.

### T-M1-NO-CONTENT-LOG

Tier: default. AC: M1-AC5, P3.

Feed sentinel audio bytes and sentinel transcript strings through error and log
paths. Resulting logs, lock files, and manifests do not contain the sentinels.
Paths with spaces and non-ASCII characters still verify.

### T-M1-NOFOLLOW-SYMLINK

Tier: default. AC: M1-AC6, R10.

A managed artifact that is a symlink, or a path that escapes the cache via a
symlink, is not ready. Opening the published file does not follow a replaced
symlink. Writes stay inside the test-owned cache; nothing is left in the shared
temp directory.

## M2 — input capture and speech windows

### T-M2-PIN-SOURCE

Tier: default for the resolver; manual for the named devices. AC: M2-AC1.

`default` resolves once and does not follow a later default-device change.
Two devices with the same display name stay distinct by UID. Selecting one UID
never opens the other. The manual run uses the built-in microphone and the named
external input.

### T-M2-RESAMPLE-20MS

Tier: default. AC: M2-AC1.

Timed 44.1 kHz and 48 kHz mono and stereo fixtures, including opposite channel
polarity, convert to mono 16 kHz finite f32. Owned-boundary error is within
20 ms. Non-finite and out-of-range PCM are rejected or clamped by the documented
rule before inference, without allocating in the callback.

### T-M2-CALLBACK-WAITFREE

Tier: default. AC: M2-AC2, I1.

The callback test double fails if the callback allocates, takes a blocking lock,
sends on a waiting channel, logs, resamples, enters Tokio, or runs a model. A
full ring increments the wait-free loss counter and returns.

### T-M2-STALL-FRESH

Tier: default. AC: M2-AC3.

Freeze the consumer until the ring and the freshness limit are exceeded. Memory
stays within the inventory caps, dropped ranges are visible in counters even if
the status ring overflows, VAD state resets, and the next emitted window is fresh
audio rather than a silent catch-up of the stale backlog.

### T-M2-FORCE-15S

Tier: default. AC: M2-AC4.

Continuous speech or tone with no silence gap, longer than the model window,
produces a boundary at the window cap with reason `max_duration`. The next window
starts at that sample and does not reuse owned samples. This test fails an
implementation that only splits when upstream VAD finds 98 ms of silence.

### T-M2-SILENCE-NO-GROWTH

Tier: default. AC: M2-AC4.

Digital silence longer than the pre-roll produces no speech window and does not
grow utterance storage. A short annotated word below the upstream 250 ms example
default is still eligible; do not inherit that default as a discard rule.

### T-M2-GAP-EPOCH

Tier: default. AC: M2-AC3, M2-AC5.

A sequence gap discards the open utterance, bumps the epoch, and does not commit
audio from both sides of the gap as one window. Already closed windows stay
immutable.

### T-M2-NO-STDIN-SOURCE

Tier: default. AC: M2-AC1.

The source parser rejects stdin, `-`, and audio file paths. Piping bytes into the
process does not create a capture session.

### T-M2-LIFECYCLE-100

Tier: default with a fake device; manual smoke on the two real inputs. AC: M2-AC5.

One hundred start/stop cycles, including forced failures on each initialization
step, leave no capture callback running and no growth beyond the cycle tolerance
after teardown. Permission denial and device removal return typed errors, prompt
at most once, and do not use freed callback memory.

### T-M2-FAKE-SOURCE

Tier: default. AC: M2-AC6, I7.

A fake source drives the same normalizer, VAD, and window types as the CPAL
backend. No test in this tier opens a real device.

### T-M2-SLEEP-WAKE

Tier: manual. AC: M2-AC5.

Sleep and wake during an input session end in the documented stop or
discontinuity outcome, with no callback use-after-free and no silent move to a
different device.

### T-M2-NO-PERSIST

Tier: default. AC: M2-AC6, I2.

After success, stall, and device-loss paths, taco-owned files do not contain the
fixture audio. The release binary has no flag that opts into saving it.

## M3 — immutable transcripts

### T-M3-ONE-COMMIT

Tier: default. AC: M3-AC1, I5.

A fake ASR emits several draft revisions, then one final. The ledger contains one
commit. The id equals the open-utterance id. A repeated final callback does not
create a second commit. Draft text after the commit does not change the commit.

### T-M3-DRAFTS-NOT-SUBMITTED

Tier: default. AC: M3-AC1, I5.

The translation sink records its inputs. Draft revisions produce zero submissions.
The single submission equals the committed source text exactly, including
punctuation and case.

### T-M3-EMPTY-NOT-COMMIT

Tier: default. AC: M3-AC1.

An empty or whitespace-only final becomes `empty_decode` on the untranscribed
ledger and submits nothing to translation.

### T-M3-NO-OWNED-OVERLAP

Tier: default. AC: M3-AC2.

Forced splits, stop flush, and silence endpoints produce half-open owned
intervals that do not overlap and do not span a gap. On the scripted boundary
fixture, displayed source text does not duplicate the boundary word. Natural-speech
duplication outside this fixture is a quality result, not a pass on this test.

### T-M3-STALE-EPOCH

Tier: default. AC: M3-AC2.

A completion carrying an old epoch or a cancelled job token cannot commit and
cannot fill an outcome slot.

### T-M3-QUEUE-CAP

Tier: default. AC: M3-AC4, I1.

Saturate the final-ASR queue with a slow fake. The next completed utterance is
untranscribed and its audio storage is released. Capture callbacks continue.
Draft requests coalesce to one pending draft and do not delay the final jobs.

### T-M3-SILENCE-10MIN

Tier: native. AC: M3-AC3.

Ten minutes of digital silence and ten minutes of the frozen non-speech fixtures
produce zero source commits.

### T-M3-DEV-ASR

Tier: native. AC: M3-AC3.

Score the development corpus only, per source language, with the frozen tokenizer.
English uses WER and Chinese uses CER. Do not pool the languages. Report
untranscribed annotated speech in the denominator. This test does not read the
held-out corpus.

### T-M3-FAKE-ASR

Tier: default. AC: M3-AC1, I7.

Replacing the native ASR adapter with the fake does not change commit,
discontinuity, or draft rules. The fake module does not link whisper.cpp.

## M4 — bilingual slice

### T-M4-ONE-OUTCOME

Tier: default. AC: M4-AC1.

Every admitted commit ends with exactly one top-level outcome. Shutdown reconciles

```text
commits = complete + failed + skipped_overload + timed_out + cancelled
pending = 0
```

A late native completion cannot replace that outcome or rewrite the source text.

### T-M4-EXACT-SOURCE-TEXT

Tier: default. AC: M4-AC1, I5.

The translation request bytes equal the committed source bytes. Control
sequences, quotes, and instruction-like text are payload.

### T-M4-NO-ROLLING-CONTEXT

Tier: default. AC: M4-AC2.

After several commits, the prompt contains the current segment only. It has no
system role, no previous target text, and no chat history. Token accounting
reserves the completion budget before the native call.

### T-M4-TOO-LONG-FAILS

Tier: default. AC: M4-AC2.

A segment that cannot fit the prompt budget, or a completion that hits the output
cap, is `failed` with `too_long` or `token_exhausted`. It is not `complete` and
not a truncated "success."

### T-M4-RAW-COMPLETION

Tier: default. AC: M4-AC3.

The stored translation is the raw completion trimmed of surrounding whitespace.
A completion that echoes the instruction or adds an explanation is `failed` with
`bad_output`, not a subtitle with the extra text deleted by a cleaner.

### T-M4-LEDGER-EQUATION

Tier: default. AC: M4-AC1.

Owned intervals do not overlap. Each captured sample in an epoch is in exactly
one of: the live ring, a gap, one owned interval, an untranscribed interval, or
non-owned pre-roll/trailing context. Commit counts alone cannot satisfy this test.

### T-M4-OVERLOAD-SKIP

Tier: default. AC: M4-AC5.

Fill the translation queue, then admit one more commit. That commit is
`skipped_overload`, the queue length does not grow, and the callback still does
not block. After the stall ends, new speech is processed and skipped ids are not
replayed. Counters still show the skips.

### T-M4-STUCK-WORKER-STOPS

Tier: default. AC: M4-AC5, I4, I6.

A native fake that ignores cancellation past the job deadline stops the session
with a typed error. The test fails if a second worker starts or the model is
loaded again.

### T-M4-PROMPT-IS-DATA

Tier: default. AC: M4-AC2.

Source text that says to ignore instructions, change the target language, or run
a tool still uses the pinned target language and does not add a tool or process
invocation. There is nothing for the model to execute.

### T-M4-DEV-MT

Tier: native. AC: M4-AC3.

On the development corpus, score reference-source translation and ASR-source
translation separately, per direction, with two reviewers when the human gate is
in scope. Do not read held-out labels. Do not replace the human gate with a
single automatic metric.

### T-M4-SMOKE-15

Tier: manual. AC: M4-AC4.

A real input session of at least 15 measured minutes meets the smoke gate: at
least 50 timed utterances, none over the 10-second completion bound, zero
drops or failed outcomes, one load of each model, no network attempts, and no
content persistence. It does not claim the hourly slope or the release quantile.

### T-M4-FAKE-PIPELINE

Tier: default. AC: M4-AC7, I7.

Fakes for capture, VAD, ASR, translation, and view run one scripted bilingual
session without CoreAudio and without model files. The ledger matches the
script. The test does not open a capture device, load a model file, or call fetch.

## M5 — system and application sources

### T-M5-REVERSE-CLEANUP

Tier: default. AC: M5-AC5.

Fake CoreAudio objects fail independently on create, start, stop, and destroy.
Rollback releases only objects this attempt created, in reverse order, once.

### T-M5-ISOLATION-SIGNALS

Tier: manual. AC: M5-AC2.

Play different licensed signals in application A, application B, and the
microphone. `app:` or `pid:` for A excludes B and the microphone. `system`
matches the declared output scope. One of the applications is multi-process and
one is single-process.

### T-M5-NO-BROADEN

Tier: default for the resolver; manual for restart. AC: M5-AC2.

An ambiguous bundle id, an unknown id, or a pid that exits does not retarget
system audio. A new process with the same bundle id is not attached without a
new invocation.

### T-M5-PERMISSION-ONCE

Tier: manual. AC: M5-AC3.

Allow, deny, and revoke on each declared OS corner, from the shipped identity in
the terminal host assigned to that corner. The prompt is not repeated in a loop.
Recovery text names the OS setting without starting capture.

### T-M5-ROUTE-AND-SLEEP

Tier: manual. AC: M5-AC4.

Same-format route changes, format-changing route changes, app exit, and
sleep/wake stop or discontinue the tap session as specified. An unrelated output
route change does not stop a pinned microphone session. This is the paired case
for [T-M2-SLEEP-WAKE](#t-m2-sleep-wake).

### T-M5-SILENCE-NOT-PERMISSION

Tier: manual. AC: M5-AC4.

A granted tap that receives silence stays `listening`. It is not reported as
permission denial.

### T-M5-CYCLES-100

Tier: manual on a real tap; default for the fake rollback. AC: M5-AC5, F5.

One hundred start/stop and partial-start cycles leave no taco-owned tap,
aggregate device, or listener, and do not change the default output device or
mute playback.

### T-M5-SAME-CONTRACT

Tier: default. AC: M5-AC1.

System and application frames run through the M2 normalizer and M3/M4 ledger
tests. A second resampler or commit type fails review.

### T-M5-SMOKE-15

Tier: manual. AC: M5-AC6.

Each of system and application passes the same smoke gate as
[T-M4-SMOKE-15](#t-m4-smoke-15) on every declared OS corner.

## M6 — CLI and TUI

### T-M6-CLI-GRAMMAR

Tier: default. AC: M6-AC1.

The frozen command list parses without loading models or opening devices.
Unknown flags and missing required arguments exit 2. A well-formed but
unsupported language or source exits 1 with a closed error code. Help, version,
and notices exit 0 and do not touch the network module.

### T-M6-EXIT-PRECEDENCE

Tier: default. AC: M6-AC4.

Cover the exit table in ADR 0006, using first-match order: clean `q`, tainted `q`,
clean SIGTERM, tainted SIGTERM, usage error, SIGINT that finishes inside the
budget (130 even if the session is tainted), SIGINT or any signal that misses the
teardown budget (1), and a second SIGINT. Deadline failure outranks 130.

### T-M6-LEDGER-ON-STDOUT

Tier: default. AC: M6-AC2, M6-AC3.

CLI stdout is the escaped commit/outcome record format and nothing else. Stderr
carries status codes and no sentinel transcript text. Redirected stdout has no
ANSI cursor sequences. Parsing stdout reconstructs the ledger, including failed
and skipped outcomes.

### T-M6-SANITIZE-TERMINAL

Tier: default. AC: M6-AC3.

Fixtures include ESC, OSC title and clipboard sequences, bidi controls, CJK, and
combining characters. The PTY title and clipboard do not change. CJK is not
replaced. Invalid UTF-8 is not emitted as a split character.

### T-M6-RESTORE-TTY

Tier: default. AC: M6-AC4.

After normal stop, SIGINT, resize, and a startup error, local termios is
restored. A responsive sink also leaves the primary screen. A sink that accepts
no bytes still finishes local restoration inside the shutdown budget and records
visual restoration as failed.

### T-M6-SLOW-SINK

Tier: default. AC: M6-AC5, F1.

A stdout or TUI sink that blocks past the output deadline does not block the
audio callback and does not prevent supervisor cancellation. The session ends
with `output_stalled`. Queued-but-unwritten finals are not reported as presented.

### T-M6-TUI-EVICT-PENDING-LAST

Tier: default. AC: M6-AC2.

Beyond the row cap, completed rows leave memory first. Pending rows remain until
a terminal outcome. Cumulative counters still include evicted completes. CLI and
TUI fed the same events report the same outcome multiset.

### T-M6-TAINT-STICKY

Tier: default. AC: M6-AC4.

A skipped or dropped interval moves the session to Degraded and sets taint.
Draining the queues returns the state to Running while taint and counters stay.
A later clean `q` exits 1.

### T-M6-HOSTS

Tier: manual. AC: M6-AC6.

On each OS corner, run the assigned endurance host and the short host-swap
sessions from the acceptance matrix. Keyboard-only operation works. Source,
target, and failure remain distinguishable with color removed.

## M7 — release qualification

### T-M7-FAIL-CLOSED

Tier: default. AC: M7-AC2, M7-AC5.

The verdict parser rejects a cell with a missing digest, a digest that differs
from the candidate, a short duration, an omitted failure, a quantile computed
after dropping timeouts, an unknown outcome count, or a thermal-invalid latency
sample marked pass. Silence from the parser is not a pass.

### T-M7-MATRIX

Tier: manual. AC: M7-AC1 through M7-AC6.

Execute the acceptance matrix on the exact signed candidate, including offline
soaks, available-network observation, persistence sentinels, and clean-machine
install. Every mandatory cell is pass, fail, or not run.

### T-M7-HELDOUT-ONCE

Tier: native plus human review. AC: M7-AC2.

Score the held-out corpus only under this ID. The runner refuses the development
label files. Each evaluation appends an immutable record of settings, digests,
and scores. A second score for the same settings and corpus revision fails the
runner.
