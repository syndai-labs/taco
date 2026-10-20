# Alternate VAD is not an M0 pin (stacked on #64 `c9a6f6f`)

Stop. The VAD model family stays Silero. M0 may not pin a different ONNX
to get a graph ORT 1.31 CoreML `GetOpBuilders` can take. The pin stays
`silero_vad.onnx`
`1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`.
`device=` stays `cpu`. M0-AC2 stays unchecked.

Linux. No CoreML, no `MLComputePlan`, no Metal. No alternate file was
fetched or hashed. This note does not create a session and does not
print a device. The fail-closed line for this host is still
`vad_ort=not_linked device=cpu reason=coreml_ep_not_in_this_binary`.

This note does not copy `ac2_ac3_cannot_flip_while_stop_shas_stand`.

## What was asked

Whether M0 may replace the pinned Silero ONNX with some other small
MIT or Apache VAD whose nodes are in the ORT 1.31 CoreML builder set
(no `If`, no `LSTM`, no `Equal`). That would be a pin change. It is
not allowed. No such file is proposed, and no Mac prove plan is
attached to one.

## Silero is the model, not "a VAD"

The charter's scope sentence says the product streams ASR with voice
activity detection. That sentence names the capability. The core stack
names the model:

`| VAD          | Silero VAD                          |`

`CHARTER.md` sha256 is
`4aaa9bf0e8561eed164a1190d210f961593ec9580c69d58ce15ed89d16119b6b`.
`charter_hash_is_unchanged` fails if that file moves. A VAD-cell edit
is a charter flip, not an M0 pin.

ADR 0003: "Use Silero VAD for speech activity and endpointing."

ADR 0004: "Preserve the charter's Silero VAD". The runtime fork under
that sentence is the selected whisper.cpp Silero integration, or else
"a statically linked maintained ONNX runtime". Both are still Silero.
"Do not hand-write a VAD model, and do not require a user-installed
ONNX dylib." Core ML conversion is an ADR 0004 non-goal. ADR 0005
forbids model conversion on the user's machine, so unrolling the
pinned LSTM into a new ONNX in this tree is the same refusal.

ADR README: the charter's core stack is an input, not a decision an
implementation silently reopens. "do not substitute a cloud API,
daemon, different model family, or narrower source scope and call v0.1
complete." A later change belongs in a new ADR that supersedes 0003
and 0004.

M0 non-goals: "Do not replace a charter source category or model family
with a quieter substitute."

Register change rules: "Changes to product scope, model family, source
categories, runtime process shape, or privacy invariants require a
deliberate charter/ADR decision, outside implicit implementation
discretion."

R3 asks "Which Silero runtime", not which VAD. A maintained static
runtime may be evaluated. Batch `whisper_full` VAD, a hand-written
model, and a user-installed ONNX dylib do not pass. The acceptance
plan's model row is "Silero; multilingual Whisper small". Charter
coverage maps "Silero VAD" to the M0 runtime/format decision.
`scripts/measure-pins.sh` fetches
`https://github.com/snakers4/silero-vad/raw/master/src/silero_vad/data/silero_vad.onnx`.

Invariant I7 lists capture, ASR, translation, and UI as replaceable.
The VAD model family is not in that list. ADR 0001's replaceable
stages are internal interfaces for fakes, not a license to swap the
pinned weights.

## Stops this does not reopen

| Tip | What already stands |
| --- | --- |
| #60 `aab661c` | No published Silero ONNX is a graph ORT 1.31 CoreML can take. The pin stays. |
| #64 `c9a6f6f` | `GetOpBuilders` on `main` is the v1.31.0 blob. An ORT upgrade is out of M0. No `LSTM` or `If` builder. |
| #59 `ed9ee10` | M0-AC2 stays unchecked while #57 `88eda06` stands. |
| #65 `04dda65` | The AC2/AC3 flip guard cites #60 and #64. Checking M0-AC2 still fails while those Stops stand. |

A non-Silero file whose ops happen to be registered would not move
`device=` off `cpu` inside M0, because M0 is not allowed to load it.
#60 already closed the published Silero files. #64 already closed an
ORT bump. This note closes the remaining substitute: a different model
family.

## What a later milestone needs

A publisher Silero ONNX whose nodes the 1.31 support checks accept, or
a merged ORT CoreML builder for `LSTM` (and `If` if the sample-rate
wrapper stays), linked as the static archives `CMakeLists.txt` already
links. Then a Mac `taco run` with `TACO_ENGINES_PROVE=1` has to print
`MLGPUComputeDevice` or `MLNeuralEngineComputeDevice` from
`MLComputePlan` before `device=` may leave `cpu`. A different model
family is a charter change plus a new ADR. That revision still does
not check M0-AC2 until the same Mac line exists. No `.mlmodelc` side
path. No conversion step.

## Consequence

- [ ] **M0-AC2:** Still open. The pin is unchanged. ASR and MT on MTL0 are
  unchanged. Silero stays on the CPU execution provider.
