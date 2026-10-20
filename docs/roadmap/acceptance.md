# Acceptance profile and evidence plan

This is the only numeric specification for milestone and release verdicts.
The machine-readable copy imported by `T-M0-LIMIT-VECTORS` is
`fixtures/m0/limit-vectors.txt`. That file is not a second budget: a cap that
disagrees with this page fails the test.
**No measurement below has been collected.** Values are proposed targets for M0
to confirm or revise through the [decision register](risks-and-decisions.md).
They are not promises copied from upstream model cards or benchmarks. Other
documents point here instead of keeping a second set of limits.

The [agent contract](agent-contract.md) says how an implementation may use this
file. The [test catalog](test-catalog.md) names the tests that enforce it.

## Goals

Make “real-time,” “accurate,” “offline,” “bounded,” and “60-minute session”
independently testable, and require them to hold together on the qualified
workload. A cell with missing, skipped, or timed-out subtitles fails even when
the successful subset looks fast.

## Non-goals

No universal latency guarantee for arbitrary hardware, accents, audio, or
terminals. No medical or legal interpretation certification. No claim that one
soak proves memory safety. No private or unlicensed recording used as benchmark
evidence. No config file, stdin audio source, or debug flag that persists
session content.

## Terms

| Term | Meaning |
| --- | --- |
| Qualification corner | One named machine plus one named OS build. Not a CLI flag. |
| Model bundle | The single v0.1 set of VAD, ASR, and translation weights. Not a profile and not a `--profile` flag. |
| Development corpus | The only labeled speech M0–M6 may use to tune or to pass M3/M4 quality gates. |
| Held-out corpus | Disjoint speech used by M7. Tuning milestones do not read its labels. |
| Smoke gate | A short real-device run that catches integration failure. It is not a release pass. |
| Release cell | One matrix session below. Every release gate applies together. |
| Qualified pair | A source/target direction M0 froze and M7 scored. Proposed starting pair set: `en→zh` and `zh→en`. |
| Taint | Sticky session flag set by any loss, skip, failed outcome, or empty decode. |

“Profile” in older notes means a qualification corner or this acceptance file.
It is not a model-selection flag. v0.1 fetches one bundle.

## Qualification corners to freeze in M0

| Corner | Proposed starting point | Evidence before the support claim |
| --- | --- | --- |
| C1, reference | Apple Silicon M1 class, 16 GiB unified memory, on AC power, macOS 14.2 | Native arm64 artifact, tap/permission/packaging run on that OS build, thermal notes |
| C2, second | A different Apple Silicon machine on a named current macOS build | Exact chip, memory, and OS build. “Latest macOS” is not a name |

These are two corners, not a full hardware-by-OS matrix. An M1 on the current
OS, or a newer chip on 14.2, is unqualified until it has its own evidence.
Results from C2 do not stand in for C1.

Status: **Partial**. M0-AC4 stays unchecked. `C2_BLOCKED` is true. `SIGNED_INSTALL` is `None`.
Charter accept is blocked: M0-AC4 cannot be checked while SIGNED_INSTALL=None or C2_BLOCKED stands.
The launchd user-agent run does not close M0-AC4.
Record: [ac4-partial.md](../evidence/m0/ac4-partial.md).
PR #95 tip `0ee1065`, PR #96 tip `008cacb`, PR #97 tip `d0877e0`, PR #98 tip `7f8ab75`, PR #99 tip `e2b7b01`, PR #100 tip `c32735f`, PR #101 tip `58f0fe5`, PR #102 tip `a0b6d95`, PR #103 tip `f5494b4`, PR #105 tip `e6fd0a9`, PR #106 tip `7f11ab8`, PR #107 tip `eba8eb1`, PR #108 tip `32986ad`, PR #110 tip `022cec3`, PR #111 tip `35155fc`, PR #112 tip `d66894e`, PR #113 tip `7a5884e`, PR #114 tip `237347f`, PR #115 tip `8b82bbd`, PR #116 tip `7fe1985`, PR #117 tip `0a962b2`, PR #118 tip `86e9393`, PR #119 tip `11ffe70`, PR #120 tip `5c833d8`, PR #121 tip `59deb5f`, PR #122 tip `7b14d7b`, PR #123 tip `a437589`, PR #124 tip `6d85580`, PR #125 tip `4fa97cc`, PR #126 tip `275f945`, PR #127 tip `d6b0c1c`, PR #128 tip `a2a810a`, PR #129 tip `5701575`, PR #130 tip `7157c5f`, PR #131 tip `321274f`, PR #132 tip `f24ded0`, PR #133 tip `d543baf`, PR #134 tip `a2ad5b2`, PR #135 tip `43397ff`, PR #136 tip `5ab4f8f`, PR #137 tip `e101b38`, PR #138 tip `55fa38f`, PR #139 tip `2ecb83f`, PR #140 tip `0db1082`, PR #141 tip `7fbe6e2`, PR #142 tip `7e308bf`, PR #143 tip `e97e74e`, PR #144 tip `97e3827`, PR #145 tip `46c6787`, PR #146 tip `49a8fb6`, PR #147 tip `20a7d0b`, PR #148 tip `e7406cf`, PR #149 tip `2c6668c`, PR #150 tip `b0f6171`, PR #151 tip `01557f6`, PR #152 tip `f6fa173`, PR #153 tip `e3a64a6`, PR #154 tip `0fb62fa`, PR #155 tip `42fbaa0`, PR #156 tip `a2254fb`, PR #157 tip `7025126`, PR #158 tip `fb37980`, PR #159 tip `13f0594`, PR #160 tip `0c6f4f4`, PR #161 tip `194d740`, PR #162 tip `bb6decd`, PR #163 tip `7d2e1dd`, PR #164 tip `32a7048`, PR #165 tip `59289d6`, PR #166 tip `13c8ae4`, PR #167 tip `1781f91`, PR #168 tip `379dec0`, PR #169 tip `e4098fb`, PR #170 tip `91e57c3`, PR #171 tip `905bd66`, PR #172 tip `87f529a`, PR #173 tip `ddd6e6c`, PR #174 tip `61e2f4f`, PR #175 tip `215a7cf`, PR #176 tip `177c599`, PR #177 tip `74f7dc3`, PR #178 tip `88a3dd3`, PR #179 tip `599ba9f`, PR #180 tip `ce1484f`, PR #181 tip `b018c3a`, PR #182 tip `f46c9f4`, PR #183 tip `6c38349`, PR #184 tip `9d8de40`, PR #185 tip `7b19eef`, PR #186 tip `dd6bd59`, PR #187 tip `6c5879d`, PR #188 tip `c4c94ea`, PR #189 tip `d210690`, PR #190 tip `167af8a`, PR #191 tip `ed45598`, PR #192 tip `68cd5ff`, PR #193 tip `b198178`, PR #194 tip `c0e16d5`, PR #195 tip `7eb431d`, PR #196 tip `17026b5`, PR #197 tip `e95c9d9`, PR #198 tip `01d24de`, PR #199 tip `726ba49`, PR #200 tip `620d3c9`, PR #201 tip `6c0e332`, PR #202 tip `d5b4799`, PR #203 tip `53c4090`, PR #204 tip `ee7fe26`, PR #205 tip `46086f9`, PR #206 tip `5bf7567`, PR #207 tip `4490817`, PR #208 tip `4a61474`, PR #209 tip `3273e7e`, PR #210 tip `8f034b1`, PR #211 tip `b28affa`, PR #212 tip `7aa8ace`, PR #213 tip `71df306`, PR #214 tip `a148b7b`, PR #215 tip `16bdd11`, , PR #217 tip `7a76a8f`, , PR #219 tip `e2f4adc`, , PR #221 tip `b9adeee`, , PR #223 tip `6c025c1`, , PR #225 tip `4d764af`, PR #227 tip `45e2e96`, PR #229 tip `de0834f`, PR #231 tip `c43a567`, PR #233 tip `e0d8e6a`, PR #235 tip `7aae198`, PR #237 tip `6fe295d`, PR #239 tip `43dde4b`, PR #241 tip `4956186`, PR #243 tip `f10a908`, PR #245 tip `115b8e7`, PR #247 tip `892279f`, PR #249 tip `171f86f`, PR #251 tip `095e5ce`, PR #253 tip `a77f7a9`, PR #255 tip `5c64818`, PR #257 tip `36f3146`, PR #259 tip `4f7a376`, PR #261 tip `4ceb145`, PR #263 tip `8cf1dac`, PR #265 tip `16324e7`, PR #267 tip `9f8ab1c`, PR #269 tip `c57e526`, PR #271 tip `9bebfb2`, PR #273 tip `721a68f`, PR #275 tip `395d7f3`, PR #277 tip `cf84771`, PR #279 tip `31fb356`, PR #281 tip `cf85013`, PR #283 tip `562db6b`, PR #285 tip `3630f49`, PR #287 tip `ad74391`, PR #289 tip `bb2f2d5`, PR #291 tip `e210c5b`, PR #293 tip `930897f`, PR #295 tip `97ea7d4`, PR #297 tip `4a1e695`, PR #299 tip `fe55d2e`, PR #301 tip `ecd0c4c`, PR #303 tip `f48bd09`, PR #305 tip `8662745`, PR #307 tip `e1ff210`, PR #309 tip `44d47ae`, PR #311 tip `5539133`, PR #313 tip `081208f`, PR #315 tip `e8ad6be`, PR #317 tip `73c1fb4`, PR #319 tip `7fca483`, PR #321 tip `b33953f`, PR #323 tip `6a07fce`, PR #325 tip `931a2a2`, PR #327 tip `f2dae26`, PR #329 tip `7dcacc2`, PR #331 tip `b6e26aa`, PR #333 tip `4089500`, PR #335 tip `a9b56ec`, PR #337 tip `0f15214`, PR #339 tip `9d64515`, PR #341 tip `6247bd6`, PR #343 tip `1980bf6`, PR #345 tip `8aaf2d7`, PR #347 tip `54e3a8a`, PR #349 tip `933fcd7`, PR #351 tip `24a58fa`, PR #353 tip `bb7ac2e`, PR #355 tip `88857b7`, PR #357 tip `288dd0a`, PR #359 tip `c7a6dcb`, PR #361 tip `7e3018d`, PR #363 tip `6094fea`, PR #365 tip `f2bc791`, PR #367 tip `f210007`, PR #369 tip `aa29a14`, PR #371 tip `37f184c`, PR #373 tip `fe5e23b`, PR #375 tip `549e4fc`, PR #377 tip `5a53b23`, PR #379 tip `6857cea`, PR #381 tip `b10baf0`, PR #383 tip `fa258a7`, PR #385 tip `737bff8`, PR #387 tip `7038359`, PR #389 tip `b936126`, PR #391 tip `2130493`, PR #393 tip `50d3ca1`, PR #395 tip `1dbe966`, PR #397 tip `669cb97`, PR #399 tip `38abcc3`, PR #401 tip `73f23de`, PR #403 tip `0eecb5d`, PR #405 tip `efbe3e3`, PR #407 tip `5f5a0a4`, PR #409 tip `b0d49da`, PR #411 tip `6f63d57`, PR #413 tip `50f1c6a`, PR #415 tip `fca6d04`, PR #417 tip `56415a3`, PR #419 tip `77ac23e`, PR #421 tip `2e23e31`, PR #423 tip `4763bb2`, PR #425 tip `b6d33db`, and PR #426 tip `542dc1c` do not name C2. Mac prefer tip is #426 `542dc1c`, not #424. This cite does not replace it. None of those tips is a Metal pass.

| Dimension | Proposed starting point | Required evidence |
| --- | --- | --- |
| Models | Silero; multilingual Whisper small; Hy-MT2 1.8B GGUF from the main GGUF repository | Publisher revision, format, quantization, SHA-256, license, llama.cpp commit, Metal settings |
| Language table | Closed. Proposed qualified pairs `en→zh` and `zh→en` | Manifest maps each user tag to the ASR id and the publisher’s full prompt language name |
| Experimental pairs | None | An empty list is valid. Adding a pair is an M0 decision, not an implied model feature |
| Terminal hosts | Apple Terminal, plus one named alternate | Named before M6. Unnamed “a modern terminal” is not evidence |
| Corpus | Licensed or consented development set and a disjoint held-out set | Fixture hashes, speakers, timings, exclusions, reviewer names |

At least two directions, with two source languages, are required. Identical
source and target tags fail preflight. There is no auto-detect. The publisher’s
language inventory is not taco’s language support. Reviewer availability is a
gate; it does not justify skipping human review or scoring only English.

M0 may revise a number in this file by a decision record that names the
measurement, the rejected alternative, and the invalidated tests. M0 may raise
the process memory ceiling only up to 8 GiB on the 16 GiB reference machine. A
higher ceiling changes the support floor and stays unresolved until that
decision exists. No revision may redefine `t_audio_end`, drop failures from a
denominator, or replace human review with an automatic metric alone.

## Operational limits

Use checked arithmetic. Reject invalid settings before allocation. These limits
are the qualification contract. They do not all become CLI flags.

| Resource | Cap | Proof |
| --- | --- | --- |
| Native input | 1–8 channels, 8–192 kHz, supported PCM | Reject outside the range before buffer allocation. Qualify 44.1 and 48 kHz, mono and stereo |
| Raw ring | 2 seconds, further bounded by 16 MiB | `capacity_frames = min(2s × rate × channels, 16 MiB / bytes_per_frame)`. For every supported format the 2-second term binds. One preallocated ring |
| Freshness | 2 seconds | Discard older queued audio, record the range, reset continuity. No silent catch-up |
| Normalized audio | Mono 16 kHz finite f32 | Resampler state exists only inside one gap-free epoch. Timeline error ≤20 ms |
| Model window | ≤15.0 s of normalized audio | Includes at most 300 ms non-owned pre-roll and at most 500 ms non-owned trailing silence |
| Pre-roll | ≤300 ms, and only samples not already owned | Used when speech starts after silence or a gap. Zero pre-roll overlap after `max_duration` |
| Endpoint silence | 500 ms initial setting | Counts in latency because it delays the endpoint. It is not part of the owned speech interval |
| Owned interval | Half-open `[start, end)` of spoken samples inside the window | Partitions speech. Does not include pre-roll silence or trailing endpoint silence |
| ASR queue | 2 waiting final utterances + 1 active final | At cap, mark the new interval untranscribed and release its audio. At most 1 pending draft |
| Draft cadence | ≤1 Hz, and only while a final slot is free | A newer draft replaces the pending draft. Draft latency is not the release metric |
| Translation queue | 4 waiting commits + 1 active job | The next commit is `skipped_overload`. No blocking capture and no extra slot |
| Segment text | ≤8 KiB UTF-8 source and ≤8 KiB UTF-8 target | Over the cap is `failed` / `too_long`, not a truncated complete |
| Translation budget | ≤2,048 tokens per segment, with 512 reserved for completion | Current segment only. No rolling chat context. Publisher `max_tokens: 4096` does not apply |
| UI retention | ≤200 completed rows + pending rows + 1 draft, and ≤4 MiB text | Evict completed rows first. Pending rows stay until a terminal outcome |
| CLI backlog | ≤64 unwritten final records and ≤1 MiB | A final that cannot be written within 2 s is `output_stalled` and stops the session |
| TUI present | Latest frame only; coalesce redraws | A frame that cannot be written within 2 s is the same output failure. Do not queue a second transcript log |
| Status memory | Fixed counters + ≤128 retained status events | This bounds memory, not how many utterances may occur. Dropping old status drops no commit, outcome, or loss count |
| Job deadline | ≤10 s for one ASR call and ≤10 s for one translation call | Hang bound. Not the latency SLO and not a license to spend 10 s + 10 s |
| Shutdown | 10 s from the first stop signal | The last 2 s are the cancellation and join reserve. Drain budget is the first 8 s |
| Fetch | SHA-256 and exact size; ≤3 transport attempts; ≤3 redirects | No retry after a digest or size mismatch. Redirects only to manifest hosts |
| Memory ceiling | `phys_footprint` ≤6 GiB proposed, on every corner | Measured with `task_vm_info.phys_footprint`. RSS is diagnostic only |

### Window and clock

The model sees at most 15.0 seconds. That window may include up to 300 ms of
pre-roll and, only for a `silence` endpoint, 500 ms of trailing silence. Both are
context, not owned speech. A forced split fires when the window reaches 15.0
seconds even if the speaker has not paused, and it does not add trailing silence
past that cap. `stop_flush` commits immediately and does not wait out the silence
timer. The next window begins on the next unowned sample and does not repeat
owned samples.

```text
t_audio_end = monotonic time of the last spoken sample in the owned interval
t_endpoint  = time the frontend finalizes that utterance
t_commit    = time immutable source text is emitted
t_mt_start  = time translation begins native inference
t_mt_done   = time the complete target text is known
t_present   = time that complete target is delivered to the active view
L_total     = t_present - t_audio_end
```

Map sample time onto a monotonic clock and report the mapping error. A capture
timestamp that ignores buffering is not sample time. `t_audio_end` is not the
end of the trailing silence and not the time translation was called.

CLI delivery is a successful write of the complete outcome record to the output
endpoint. TUI delivery is a successful frame that contains the complete
translation. These are instrumentation points, not proof of monitor scanout.
Redirected stdout does not by itself prove a person saw the line. Keep one
real-terminal timing check per host. Report queue wait, endpoint delay, decode,
and presentation separately.

Annotated utterances stay in the denominator when VAD misses them or splits them
differently. Missing content is a coverage or quality failure, not a discarded
timing sample. Playback-alignment uncertainty above 20 ms makes that timing
sample ineligible. If more than 5% of annotated utterances in a cell are
ineligible, the cell is invalid, not a pass. Product failures are never the
samples removed for alignment.

### Latency gates

For natural utterances of 1–10 seconds under normal load:

| Stat | Target |
| --- | --- |
| p50 | ≤2 s |
| p95 | ≤4 s |
| p99 | ≤8 s |
| max | ≤10 s |

Use nearest rank, `ceil(p/100 × N)`, and publish N, min, and max. Forced
boundaries are a separate distribution: p95 ≤5 s from the synthetic split, and
every forced segment still finishes within 10 s of its boundary. A 15-second
window does not count as 15 seconds of user-visible wait before its boundary;
report that limitation beside the boundary chart.

Any missing, empty, skipped, failed, timed-out, or cancelled outcome fails a
normal-load cell before quantiles are computed. Do not publish a passing
percentile of the successful remainder. Stress runs that intentionally exceed
the queues use the resilience verdict and are not normal-load evidence.

The 10-second job deadline is a hang detector. Normal-load `L_total` still has
to meet the table while both models share the GPU. Cold model load and time to
the first draft are separate numbers and cannot substitute for `L_total`.

Warmup is unscored. It lasts 5 minutes before the measured window and is outside
that window. Thermal pressure `serious` or `critical` for more than 60
consecutive seconds during the measured window invalidates latency. The cell is
rerun or marked not run. It is not a pass.

### Memory and inventory

Primary metric: peak `phys_footprint` during the measured window. The proposed
ceiling is 6 GiB on every corner, including C2. A faster machine does not receive
a larger budget unless this file is revised for every corner together.

After the unscored warmup, on each release cell:

- fitted slope of `phys_footprint` ≤0.25 MiB/min
- last-five-minute median ≤ first-five-minute median + 64 MiB
- every inventory high-water mark ≤ its cap
- growth that cannot be assigned to a named capped buffer fails, even inside the
  slope band
- a rise that continues through the last 20 minutes of the hour fails even when
  the fitted slope is inside the band

M0 may widen the slope band only after measuring a no-speech idle hour on C1, and
only up to 1 MiB/min. That revision is a decision record. It is not permission to
restore a multi-mebibyte-per-minute leak budget.

The inventory names owner, bytes, lifetime, and overflow action for the raw ring,
normalized audio, utterance slots, draft snapshot, native scratch and mel or
token buffers, pending outcomes, UI text, CLI backlog, and status ring. The sum
of those caps, plus the measured resident model and Metal cost, must sit under
the ceiling. Native callback buffers are OS-owned; record the largest observed
callback and do not copy past the ring cap.

One hundred in-process start/stop cycles, models released on each teardown:
`phys_footprint` after cycle 100 ≤ footprint after cycle 1 teardown + 32 MiB.
A new process per cycle does not satisfy this test. RSS and allocator counters
are reported next to `phys_footprint` and do not replace it.

### Fetch and writable paths

SHA-256 covers the exact artifact bytes. Three transport attempts may recover a
dropped connection. A digest mismatch, a size mismatch, or an HTTP downgrade
fails immediately. At most three redirects, and only to hosts written in the
manifest. Restart an interrupted artifact from the beginning; do not implement
range resume in v0.1.

The release and session paths may create files only in the user-owned model
directory. Those files are weights, non-content manifests, and lock or partial
files. Stdout and stderr are the user’s terminal. A developer harness may write
its own traces outside the product binary. OS Metal caches and crash reporters
are not taco content stores; putting sentinel transcript or audio bytes into
them still fails the privacy gate.

## Speech schedule and corpora

M0 freezes the development corpus, the held-out corpus, the critical set, and
the normal-load schedule before optimization. Record hashes, references,
speaker and accent tags, conditions, utterance bounds, normalization, and
exclusions. Do not scrape unlicensed media and do not record people who have
not consented. Synthetic silence and the external harness are allowed.

For each qualified direction the corpus contains at least 200 natural
utterances and 30 minutes of speech, at least 5 speakers, and at least 20 long
passages that force segmentation. The 200 utterances and the long passages are
included in the 30 minutes, not added on top of them. Cover conversation,
technical terms, names, dates, numbers, negation, questions, accents, noise,
short words, and pauses inside sentences.

The normal-load hour mixes natural pauses, 50–70% speech occupancy, and one
10-minute sustained-speech block. That block produces at least 30 forced
segments and counts toward the 30 minutes of speech. Its pieces are not required
to also count as the 200 natural utterances.

| Gate | Release threshold | Method |
| --- | --- | --- |
| ASR, clean | WER ≤15% for the proposed English source; CER ≤15% for the proposed Chinese source | Frozen tokenizer per language. If M0 changes languages, it records WER or CER for each language here before scoring. Do not insert spaces into Chinese so that WER looks defined |
| ASR, noise | WER or CER ≤25% on the frozen noisy stratum | Do not pool with clean speech |
| Coverage | ≥98% of annotated speech duration owned by some commit; zero eligible utterances omitted entirely | Misses stay in the denominator |
| Non-speech | Zero commits on 10 minutes digital silence and 10 minutes frozen non-speech | Includes music and noise |
| Adequacy | ≥95% of reviewed outputs rated ≥4 on the 1–5 scale, both reference-source and ASR-source | Two bilingual reviewers; keep disagreements |
| Critical meaning | Zero critical errors on the frozen ≥50-case set per direction | Negation, critical numbers, invented entities or actions, omitted instructions |
| Long speech | Same adequacy bar; report boundary loss and duplication separately | Review the passage, not only the lucky segment |

Adequacy: 5 preserves meaning naturally; 4 preserves core meaning with minor
wording defects; 3 has a material omission or ambiguity; 2 changes a major
meaning; 1 is unrelated, missing, or unusable. Empty, failed, and skipped
outputs score 1. The critical set is a regression gate, not a safety certificate
for unseen speech. An automatic translation metric may be reported beside human
scores and cannot replace them.

The clean or noisy threshold follows the fixture label M0 froze, not the source
category and not the score. A clean fixture captured by a tap stays in the clean
stratum. Relabeling it after the result is a failed cell, not a revision.

M3 and M4 may meet these numeric thresholds on the development corpus only.
M7 repeats them on the held-out corpus and on the matrix below. A failing
critical case is not removed. A case may be withdrawn only when the reference
itself is wrong, with a reviewer note. Every held-out evaluation is appended to
an immutable record. Seeing a held-out score and then tuning requires a new
decision, and the earlier score stays visible.

M2 owns endpoint coverage of annotated speech and the silence-window behavior.
It does not own WER or translation adequacy.

## Smoke gate

Smoke is mandatory integration evidence and is not the release matrix.

| Smoke | Where | Pass |
| --- | --- | --- |
| Source smoke | M3, ≥10 measured minutes, one real input | Progressive commits, ledger identities hold, no crash. No WER claim |
| Bilingual smoke | M4, ≥15 measured minutes, one real input, one qualified direction | ≥50 timed utterances, each `L_total` ≤10 s, report p50 and p95, zero loss or bad outcomes, one model load, no network attempt, no content persistence |
| Tap smoke | M5, the bilingual smoke on system and again on application, each corner | Same bilingual smoke bar |

Smoke does not claim p99 with N≥200, the hourly slope, or held-out adequacy.

## Release matrix

One signed candidate. Every cell is that digest. A cell is pass, fail, or not
run. There is no provisional pass.

Sixteen endurance cells: 4 source categories × 2 UI modes × 2 corners. Each
cell is 5 unscored warmup minutes plus at least 60 measured minutes, one frozen
direction, and the normal-load schedule. Each cell contains at least 200 timed
natural utterances so its own latency quantiles are valid. C1 cells run in
Apple Terminal. C2 cells run in the named alternate host.

Direction assignment, with d1 and d2 the ordered qualified pairs. This rotation
puts both directions on every source category and every corner. Do not collapse a
row to one direction:

| Source | C1 CLI | C1 TUI | C2 CLI | C2 TUI |
| --- | --- | --- | --- | --- |
| Built-in microphone | d1 | d2 | d2 | d1 |
| External input | d2 | d1 | d1 | d2 |
| System output | d1 | d2 | d1 | d2 |
| Selected application | d2 | d1 | d2 | d1 |

C1 application cells use the named multi-process application. C2 application
cells use the named single-process application. Short isolation tests still play
both applications on both corners and are not extra endurance cells.

Build each hour so the 200 natural utterances and the sustained block fit inside
the 50–70% occupancy band. Do not stack them on top of an hour that is already
full, and do not drop below 200 to make the clock fit.

Each cell, taken alone, must pass loss accounting, crash, offline operation,
memory, and latency. Quality review is attached as follows, because the renderer
does not change the ledger:

- Full human adequacy for every direction on every corner, using that corner’s
  microphone cell ledger (four full reviews for the proposed pair set).
- Automated ASR, coverage, latency, and loss on every cell.
- The critical set replayed through every source category on C1, for every
  direction. These are short scripted passes, not extra 60-minute cells. Repeat
  that human critical set on C2 when that corner’s automated error rate for the
  same source and direction is more than 5 points worse than C1, or when C1 did
  not pass.

M6 also runs one CLI session and one TUI session of at least 10 minutes on the
microphone with the host that the endurance cells did not use, on each corner.
Those four host-swap sessions check presentation, input, and restoration. They
do not replace an endurance cell and do not repeat the full corpus.

A built-in microphone and an external input are different categories. System
and application are different categories. One microphone hour does not prove a
tap. M0 reserves the observed non-default input `syndai-iphone-02 Microphone`
(`system_profiler SPAudioDataType` on 2026-10-17: not the default input, transport
Unknown, one input channel) for M2. That profiler record had no device uid.
`EXTERNAL_INPUT.display_name` stores the name. `qualified_in_m0` stays false.
That name is not an M0 qualification. UI fidelity tests compare the presented ledger with the pipeline ledger;
they do not require a second human translation review of the same strings.

## Fault, privacy, and distribution gates

| ID | Required evidence | Owner |
| --- | --- | --- |
| F1 | Stall capture, ASR, translation, and the terminal separately. Bounded memory, visible loss, fresh recovery, no blocked callback | M2, M3, M4, M6 |
| F2 | Deny and revoke permission, remove the device, exit or restart the app, change route, sleep and wake. Stop cleanly and do not broaden the source | M2, M5 |
| F3 | Missing, corrupt, or incompatible model; allocation failure; native error; cancellation under Metal. No uncontrolled crash and no replacement worker | M0, M1, M4 |
| F4 | Stop during preflight, starting, running, and teardown, including full queues and late callbacks. Join workers. Restore the local terminal mode. Blocked sinks have a bounded visual-restore failure | M2, M6 |
| F5 | ≥100 in-process start/stop and partial-start cycles. No retained taps, devices, workers, or handles, and footprint within the cycle tolerance | M2, M5, M6 |
| P1 | With connectivity up, zero process-attributed DNS or connection attempts for every non-fetch command, full session, missing model, and error path | M1, M7 |
| P2 | After a successful fetch, disconnect the network and pass every endurance cell | M7 |
| P3 | Sentinel audio and text do not appear in taco-owned files or logs after success, native errors, or cancellation | M1, M7 |
| D1 | Clean download of the signed candidate, trust or notarization as applicable, both hosts, exact fetch, permissions | M0, M7 |
| D2 | Mach-O inspection: no non-system dylib, no sidecar code or shader, no weights inside the executable, no helper inference process | M0, M7 |

P1 and P2 are different facts. A machine that cannot reach the network can still
have attempted a connection. Attribute OS trust and notarization traffic
separately from the session. P3 uses test-owned paths and consented fixtures; it
does not inspect unrelated user files.

An uncontrolled crash is an abort, a fatal signal, a panic that ends the
process, or a hang past the shutdown budget. A typed error that exits 1 is not
a crash. OS jetsam is a failed cell.

## Normal-load reconciliation

A normal-load cell has no crash, no unexpected stop, no dropped audio, no
skipped commit, no failed, timed-out, or cancelled translation, no model reload,
and no untallied utterance. Drain within the shutdown budget:

```text
commits = complete + failed + skipped_overload + timed_out + cancelled
pending = 0
```

Under a passing normal-load cell the failed, skipped, timed-out, and cancelled
counts are zero. The equation still has to be reported so a later stress run can
use it. Also reconcile captured samples, gaps, owned speech, and untranscribed
intervals. Commit counts do not detect a VAD miss.

## Evidence and verdict

Each run records gate ids, command, executable and model digests, hardware, OS
build, terminal, source id, corpus version, settings version, start and duration,
expected and actual counts, thresholds, stage timings, result, failure reason,
and reviewer. Keep aggregate metrics and non-sensitive fixture examples. Do not
archive private live sessions.

Pass, fail, or not run. A missing identity, a short duration, an unavailable
observer, or an unknown count is not run or invalid, never pass. M0 fills the
corner and corpus fields with experiments. M7 fills results. This planning
review supplies neither.
