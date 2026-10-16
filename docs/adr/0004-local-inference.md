# ADR 0004: Pinned native inference with controlled Metal scheduling

Status: **Proposed**. Validation: M0 combined native spike and model selection;
M3 ASR; M4 translation; M7 exact-artifact qualification.

## Context and goals

Preserve the charter's Silero VAD, Whisper Small through whisper.cpp/Metal, and
Hy-MT2 1.8B GGUF through llama.cpp/Metal. Run all inference in the executable.
Upstream support for a model or Metal does not prove the Rust bindings, chosen
quantization, two-engine linkage, cancellation, or sustained combined workload.

## Non-goals

No Python installation at runtime, subprocess inference CLI, llama server, cloud
fallback, model training, user-supplied arbitrary model plugins, Core ML conversion,
automatic model switching, or use of Whisper's translation mode for target-language
translation. The ASR stage must transcribe in the selected source language.

## Decision

Use multilingual Whisper **small**, not `small.en`. Pin an exact whisper.cpp-compatible
artifact and digest. Whisper's converted ggml format and translation's GGUF format
are separate contracts; a shared suffix or generic model loader must not conflate them.

Use the publisher's **Hy-MT2-1.8B-GGUF** repository, not HY-MT1.5, an MLX artifact,
or the sibling 1.25-bit and 2-bit repositories. Those extreme quants are not a quiet
way to pass the memory ceiling. M0 selects one quantization from the main GGUF
repository by measuring quality, memory, and latency together. Do not invent a
filename or digest. If no published quantization meets both gates, R4 stays open.

For Silero, prefer the selected whisper.cpp revision's Silero integration when it
exposes incremental frame-by-frame state: fixed input frames, bounded RNN state, and
no requirement to reprocess the whole session. Using `whisper_full` batch VAD, or
copying whisper.cpp's CLI defaults for overlap, minimum speech, and silence-only
splits, does not meet the segment contract. Otherwise assess a statically linked
maintained ONNX runtime. Do not hand-write a VAD model, and do not require a
user-installed ONNX dylib. Freeze the exact model, format, runtime, sample rate,
and frame width in M0.

As of the 2026-10-16 model-card check, Hy-MT2 GGUF may require a llama.cpp revision
that contains the publisher's STQ kernel. M0 rechecks that requirement and pins the
commit. The card's `llama-completion` example and its `-ngl 0` CPU setting are not
the product. v0.1 runs in process, on Metal, with GPU layers loaded. A build that
only works through a helper CLI or only on CPU fails M0.

Each adapter exposes initialize, bounded infer, cooperative cancel, and close.
Cancel through the engine's abort callback and join the worker. Do not use
`pthread_cancel`, detach, or process exit as cancellation. Validate dimensions,
sizes, language IDs, and model compatibility before native calls. Keep unsafe
pointer lifetimes in small audited wrappers. Choose maintained Rust bindings only
after checking that they expose these APIs; write narrow bindings if necessary.
Do not lock a crate version by guesswork. Link whisper.cpp and llama.cpp in one
process, either on one shared ggml or with symbol isolation that stays inside the
single Mach-O. Two dylibs on disk and a helper process are both failures. Metal
shader sources used by the engines are embedded in that Mach-O.

## Scheduling and cancellation

Start with one shared GPU admission permit across ASR and translation, with priority
for final ASR and fairness that prevents translation starvation. Draft ASR is optional
and lowest priority. Model initialization is serialized; both model contexts remain
resident once loaded. Compare this policy against controlled concurrent execution
in M0 using identical fixtures. Accept the policy that satisfies total latency and
memory gates, and record it; do not infer performance from isolated throughput.

Admission priority cannot preempt an already-running native call. Bound inference
input and output; exercise engine abort hooks at supported checkpoints and measure
their response under Metal. A logical timeout marks work expired but does not prove
the worker stopped or release its buffers. Keep resources owned until completion
or actual cancellation; never launch an unbounded replacement worker. If a chosen
backend cannot provide a safe bounded shutdown, M0 fails pending a design change.

For translation, use one user turn and no system prompt. The publisher states that
Hy-MT2 has no default system prompt. v0.1 starts from the publisher's English
default instruction, with `{target_lang}` set to the full English language name
from that model card (`English`, `Chinese`), not the user tag `en` or `zh`. The
manifest maps tags to ASR ids and to those names. M0 may compare that instruction
only with the publisher's matching Chinese default instruction, on the development
corpus, and records the winner. Do not adopt terminology, style, or personalization
templates, and do not keep previous turns.

Treat transcript text as data. There are no tools. Reset the context every segment.
The live token budget is the acceptance cap; the card's `max_tokens: 4096` is for
document translation and is not the subtitle cap. M0 compares the card's recommended
sampling (temperature 0.7, top_p 0.6, top_k 20, repetition penalty 1.05) with a
seeded low-temperature arm on the same development fixtures, then pins one. GPU
kernels may still disagree across machines. Protocol tests assert ledger properties.
Quality tests score the raw completion after pinned stop sequences, trimmed only of
surrounding whitespace. An instruction echo or explanation is `bad_output`, not text
for a cleaner to delete.

## Alternatives and consequences

Concurrent GPU execution may improve throughput but can worsen tail latency and
peak unified memory. Pure serialization may make translation wait behind long ASR
jobs. The tradeoff must be measured. Runtime CPU fallback is not promised: unsupported
Metal or allocation failure produces an actionable startup/session error. CPU modes
used in developer tests do not establish product support.

## Acceptance criteria

- [ ] One executable loads VAD, multilingual small, and the exact Hy-MT2 GGUF once,
  performs a bilingual example offline, and releases all native contexts on exit.
- [ ] The build records Rust/SDK/C++ toolchain, binding and upstream revisions,
  shared ggml linkage strategy, backend flags, model digests, prompt and decoder settings.
- [ ] Metal use is verified from instrumentation; concurrent engines neither collide
  at link/load time nor silently load a different model/backend.
- [ ] Cancellation, corrupt/incompatible model input, allocation failure, oversized
  segments, and decoder exhaustion have bounded, observable outcomes.
- [ ] The selected scheduling policy meets the shared quality/resource gates on the
  reference machine; isolated microbenchmarks are supporting evidence only.

## Primary references

Upstream documents establish available components, not taco performance:
[whisper.cpp](https://github.com/ggml-org/whisper.cpp),
[Silero VAD](https://github.com/snakers4/silero-vad),
[llama.cpp](https://github.com/ggml-org/llama.cpp),
[Hy-MT2 publisher repository](https://github.com/Tencent-Hunyuan/Hy-MT2), and the
[official 1.8B GGUF model card](https://huggingface.co/tencent/Hy-MT2-1.8B-GGUF).
Recheck and pin these at M0; model/runtime licenses are recorded separately.
