# Charter-to-delivery coverage

This matrix maps the entire [charter](../../CHARTER.md) to proposed decisions and
observable evidence. “Covered” means planned, not implemented. All product gates are
currently **not run**. The [acceptance plan](acceptance.md) defines the common verdicts.

## Product form and v0.1 scope

| Charter requirement | Owning design | Milestone criteria | Evidence needed |
| --- | --- | --- | --- |
| macOS Apple Silicon | ADR 0002, 0007 | M0-AC1/4; M7-AC1/2 | Both named qualification corners and the actual source/UI runs |
| Rust single binary, CLI + TUI | ADR 0001, 0004, 0006 | M0-AC2/3; M6-AC1/3/6; M7-AC1 | Native dependency inspection, clean install, both terminal modes |
| One microphone/input/system/application | ADR 0002 | M2-AC1; M5-AC1/2; M7-AC2 | Four-source matrix, isolation signals, explicit selectors |
| Streaming ASR with VAD and stable commits | ADR 0003, 0004 | M2-AC4; M3-AC1/2/3 | VAD/forced-boundary corpus, draft revision and commit invariants |
| Local committed-segment translation with configurable languages | ADR 0003, 0004 | M4-AC1/2/3 | Draft exclusion, exact committed input, per-direction quality |
| Live source, translation, and status in CLI/TUI | ADR 0006 | M6-AC1/2/3/5 | Paired event fixtures, PTY and real-terminal runs |
| Separate model download/cache, offline thereafter | ADR 0005 | M1-AC1/2/3; M7-AC4 | Fetch faults, integrity, P1 zero-attempt and P2 offline evidence |

## All seven invariants

| Invariant | Owning design | Milestone criteria | Falsification test |
| --- | --- | --- | --- |
| I1: Capture never blocks on inference; all queues/context bounded | ADR 0001, 0003, 0004 | M2-AC2/3; M3-AC4; M4-AC2/5; M6-AC5; `T-M2-CALLBACK-WAITFREE`; `T-M2-STALL-FRESH`; `T-M3-QUEUE-CAP`; `T-M4-OVERLOAD-SKIP`; `T-M6-SLOW-SINK` | Stall each consumer; wait-free loss counters stay readable when status is dropped; inventory caps sum under the memory ceiling |
| I2: Audio/transcripts local, not persisted by default | ADR 0005, 0006 | M1-AC5/6; M2-AC6; M7-AC4; `T-M1-NO-CONTENT-LOG`; `T-M2-NO-PERSIST` | P3 sentinel inspection through success, native errors, and cancellation. No dump flag in the release binary |
| I3: No session network; explicit downloads only | ADR 0005 | M1-AC2/3; M7-AC4 | P1 observe attempts with network up, P2 run disconnected; test missing-model path |
| I4: External weights, one load per session | ADR 0004, 0005 | M1-AC4; M4-AC4; M7-AC1/3 | Inspect package, count model initialization through long runs and failures |
| I5: Immutable commits; translation consumes commits only | ADR 0003 | M3-AC1/2/6; M4-AC1; `T-M3-DRAFTS-NOT-SUBMITTED`; `T-M4-EXACT-SOURCE-TEXT`; `T-M4-NO-ROLLING-CONTEXT` | Draft storm, duplicate/late callbacks, exact submitted text, one terminal outcome, no chat history |
| I6: Device loss, permission denial, model failure avoid uncontrolled crashes | ADR 0002, 0004, 0006 | M2-AC5; M5-AC3/4/5; M6-AC4; M7-AC3 | F2–F5 on actual backend/OS, including partial initialization and saturation |
| I7: Capture, ASR, translation, UI replaceable | ADR 0001 | M0-AC6; M2-AC6; M3-AC6; M4-AC7; M6-AC2; `T-M2-FAKE-SOURCE`; `T-M3-FAKE-ASR`; `T-M4-FAKE-PIPELINE`; `T-M6-TUI-EVICT-PENDING-LAST` | Substitute each fake adapter and preserve the ledger. M4-AC6 is the slow-output gate, not this invariant |

## Core stack, north star, and exclusions

| Commitment | Plan treatment |
| --- | --- |
| Rust, Clap, Ratatui | M0 contract freeze, M6 parser/terminal implementation |
| CPAL, CoreAudio HAL/process tap | M0 actual-artifact proof, M2 input adapter, M5 taps |
| Silero VAD | M0 runtime/format decision, M2 stateful endpointing |
| Whisper Small, whisper.cpp + Metal | M0 pin/coexistence proof, M3 multilingual transcribe-only ASR. Not Whisper's translation mode, and not batch VAD as the session segmenter |
| Hy-MT2 1.8B GGUF, llama.cpp + Metal | M0 artifact/quantization proof on Metal. Not a llama-server, not the card's CPU example, not the 1.25-bit or 2-bit sibling repos |
| Tokio, bounded channels | M2 control/data infrastructure; native callbacks and blocking work remain outside async executor tasks |
| Universal local interpreter north star | Replaceable stage boundaries support later outputs; v0.1 remains one source/pair/session and terminal subtitles |
| No TTS, playback/recording, multi-source mixing | No product command or pipeline stage for these; fixture playback is external test equipment only |
| No diarization, history/search, GUI | No speaker identity model, persisted transcript store, or graphical application |
| No cloud inference or cross-platform support | No remote inference adapter or non-macOS runtime claim; offline/arm64 release evidence required |
| No daemon/external inference service | Single-process dependency/process observation in M0 and M7 |

## Charter done condition and primary metric

| User-visible condition | Final proving gate |
| --- | --- |
| Install one executable | M7-AC1, D1/D2 on a clean environment |
| Fetch models | M1-AC1/3 and M7-AC1 against pinned artifacts |
| Select a source | M2-AC1 and M5-AC1/2; all categories in M7-AC2 |
| Accurate bilingual subtitles | M3-AC3 and M4-AC3 on the development corpus; M7 held-out review and the release matrix, not a smoke run |
| 60-minute offline session | The 16-cell matrix under P2, M7-AC2/4. A 10- or 15-minute smoke does not fill a cell |
| No crashes or unbounded memory growth | Static bounds + F1/F3/F5 + actual hourly footprint/peak traces, M7-AC3 |
| Speech end to completed translated subtitle | Annotated sample clock through final delivery, required quantiles/counts and missing-outcome accounting in M7-AC2/5 |

## Acceptance criteria for this coverage map

- [ ] At implementation review, every row links to current reproducible evidence.
- [ ] No required category/invariant is replaced by a smaller demo or merely a source
  inspection; unrun hardware/quality/release gates remain visibly unrun.
- [ ] A charter change updates the affected ADRs, milestones, acceptance plan, test
  catalog, and this matrix together before it changes a release verdict.
