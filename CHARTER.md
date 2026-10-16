# taco — Charter

**Status:** Incubating
**Target:** macOS (Apple Silicon)
**Form:** Rust single binary, CLI + TUI

## Vision

Real-time, private speech translation from any supported macOS audio source to bilingual terminal subtitles. No cloud, daemon, or external inference service.

## v0.1 Scope

- Select and capture one microphone, input device, system audio stream, or application.
- Stream ASR with voice activity detection and stable, committed transcripts.
- Translate committed segments locally with configurable source and target languages.
- Display live source text, translations, and session status in CLI/TUI.
- Download and cache model weights separately; run fully offline afterward.

## North Star

Become a universal, local-first interpreter for live audio runtime: capture conversations from any supported app or device, translate bidirectionally in real time, and deliver synchronized output stream, such as subtitles or speech wherever the user listens. The terminal is the starting point, not the product boundary.

## Non-Goals

TTS, audio playback/recording, multi-source mixing, diarization, transcript history/search, GUI, cloud inference, and cross-platform support.

## Core Stack

| Layer        | Choice                              |
| ------------ | ----------------------------------- |
| Runtime / UX | Rust, Clap, Ratatui                 |
| Capture      | CPAL, CoreAudio HAL / Process Tap   |
| VAD          | Silero VAD                          |
| ASR          | Whisper Small, whisper.cpp + Metal  |
| Translation  | Hy-MT2 1.8B GGUF, llama.cpp + Metal |
| Coordination | Tokio, bounded channels             |

## Invariants

1. Audio capture never blocks on inference; all queues and context windows are bounded.
2. Audio and transcripts stay local and are not persisted by default.
3. No network activity during sessions; downloads require an explicit command.
4. Model weights are external to the binary and loaded once per session.
5. Committed transcripts are immutable; translation consumes committed segments only.
6. Device loss, denied permissions, and model failures must not cause uncontrolled crashes.
7. Capture, ASR, translation, and UI are independently replaceable.

## Done When

A user can install one executable, fetch models, select an audio source, and view accurate bilingual subtitles during a 60-minute offline session without crashes or unbounded memory growth.

**Primary metric:** Time from the end of spoken audio to the completed translated subtitle.

## Reference

[livecaption](https://github.com/six-ddc/livecaption)
