# M0 license inventory (2026-10-16)

`freeze::LICENSES` keeps five subjects. `spdx` and `notice` are set only from a
LICENSE file fetched for that subject. `notice` is the copyright line in that
file. The permission paragraph and the Apache terms were not copied into the
crate. An engine LICENSE is not a model license. No M0-AC checkbox is checked.
This inventory does not clear distribution.

Pins are the digests in [pin-measurement.md](pin-measurement.md) and the engine
commits in [metal-link.md](metal-link.md). Fetches were on 2026-10-16.

## Records

| Subject | Covers | SPDX | Notice | LICENSE fetched |
| --- | --- | --- | --- | --- |
| `silero-vad-weights` | model | `MIT` | `Copyright (c) 2020-present Silero Team` | [snakers4/silero-vad `1e261b0` LICENSE](https://github.com/snakers4/silero-vad/blob/1e261b036686cd0017d500ee96acd1c4ba572a9d/LICENSE) |
| `whisper-small-weights` | model | unset | unset | No LICENSE file in the pin repo. See the gap below. |
| `hy-mt2-1.8b-gguf` | model | `Apache-2.0` | `Copyright (C) 2026 Tencent. All rights reserved.` | [tencent/Hy-MT2-1.8B-GGUF `LICENSE.txt`](https://huggingface.co/tencent/Hy-MT2-1.8B-GGUF/blob/a0c709d9fac510f2c807aa3af52872340dc37a4a/LICENSE.txt) |
| `whisper.cpp` | engine | `MIT` | `Copyright (c) 2023-2026 The ggml authors` | [ggml-org/whisper.cpp `0c7be69` LICENSE](https://github.com/ggml-org/whisper.cpp/blob/0c7be696371069e0baa2515ed7aa08d5a5473aa6/LICENSE) |
| `llama.cpp` | engine | `MIT` | `Copyright (c) 2023-2026 The ggml authors` | [ggml-org/llama.cpp `2a66703` LICENSE](https://github.com/ggml-org/llama.cpp/blob/2a667032dc788c3a071d538e0b572e45a3f12731/LICENSE) |

The two engine notices match each other because both root LICENSE files carry
that copyright line. They are not copied onto `whisper-small-weights`,
`silero-vad-weights`, or `hy-mt2-1.8b-gguf`.

## How the SPDX identifier was read

None of the MIT files contain an `SPDX-License-Identifier` header. Each begins
`MIT License` and continues with the MIT permission grant. `MIT` is the SPDX
license-list identifier for that text.

`LICENSE.txt` on the Hy-MT2 GGUF repo says `Hy-MT2-1.8B-GGUF is licensed under
the Apache License, Version 2.0.` From `Version 2.0, January 2004` through the
end, that file matches [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt)
after stripping surrounding whitespace. `Apache-2.0` is the SPDX identifier for
that text. The model card writes the same license as `apache-2.0`.

## What was checked before a field was left unset

### Silero weights

`scripts/measure-pins.sh` still downloads `silero_vad.onnx` from `master`.
`master` on 2026-10-16 was `1e261b036686cd0017d500ee96acd1c4ba572a9d`. The ONNX
at that commit hashed to
`1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`, the pin.
The LICENSE at that commit is the MIT text above. The repo has no other
license-named file. README line 137 says the VAD is published under MIT. The
badge alt text says `License: CC BY-NC 4.0` while the badge image and link are
MIT. The alt text was not used.

### Whisper small weights

The pin URL is
`https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin`.
Repo revision `5359861c739e955e79d9a303bcbc70fb988958b1` (card
`lastModified` 2024-10-29) lists README and weight files. It does not list a
LICENSE file. The README frontmatter is `license: mit`. That tag is not license
text and was not written into `spdx` or `notice`. There is no copyright line
on the card to store.

whisper.cpp `models/README.md` at `0c7be69` points at OpenAI's model URLs and
does not state a license for `ggml-small.bin`. `openai/whisper` `main` has an
MIT LICENSE with `Copyright (c) 2022 OpenAI`. `main` is not a pin of this
file, and that LICENSE was not applied. The whisper.cpp engine LICENSE was
not applied either.

### Hy-MT2 GGUF

The weight record uses the GGUF repo's `LICENSE.txt` at
`a0c709d9fac510f2c807aa3af52872340dc37a4a`. The sibling list has no NOTICE
file. The appendix in the license is the Apache boilerplate
(`Copyright [yyyy] [name of copyright owner]`), not a second filled notice.

GitHub `Tencent-Hunyuan/Hy-MT2` `ff1903ecaa724e10951a23c16817a2413c752b35`
`LICENSE.txt` differs in the preamble only: it says `Hy-MT2`, not
`Hy-MT2-1.8B-GGUF`. The GGUF record uses the file that names the pinned
artifact. `LICENSE-IFMTBench.txt` says IFMTBench is CC-BY-4.0. IFMTBench is
not the GGUF, so that license was not applied.

### Engine trees, other than the root LICENSE

These files are in the pinned trees. Their copyright lines are not stored on
the five `LicenseRecord`s. They do not change the root SPDX of whisper.cpp or
llama.cpp, and they are not model licenses.

| Path at the pin | Copyright line read from the file |
| --- | --- |
| whisper.cpp `bindings/go/LICENSE` | `Copyright (c) 2022 David Thorpe` |
| whisper.cpp `examples/wchess/wchess.wasm/chessboardjs-1.0.0/js/chessboard-1.0.0/LICENSE.md` | `Copyright 2019 Chris Oakman` |
| whisper.cpp `tests/earnings21/normalizers/LICENSE` | `Copyright (c) 2022 OpenAI` |
| whisper.cpp `tests/librispeech/normalizers/LICENSE` | `Copyright (c) 2022 OpenAI` |
| whisper.cpp `tests/librispeech-parakeet/normalizers/LICENSE` | `Copyright (c) 2022 OpenAI` |
| llama.cpp `gguf-py/LICENSE` | `Copyright (c) 2023 Georgi Gerganov` |
| llama.cpp `licenses/LICENSE-jsonhpp` | `Copyright (c) 2013-2025 Niels Lohmann` |
| llama.cpp `vendor/cpp-httplib/LICENSE` | `Copyright (c) 2017 yhirose` |
| llama.cpp `vendor/hash/rotate-bits/LICENSE.md` | `Copyright (c) 2021 William Casarin <jb55@jb55.com>` |
| llama.cpp `vendor/hash/sha256/LICENSE` | `2010-06-11 : Igor Pavlov : Public domain` |
| llama.cpp `vendor/hash/xxhash/LICENSE` | `Copyright (c) 2012-2021 Yann Collet` |
| llama.cpp `tools/ui/src/lib/vendors/big-integer/LICENSE` | Public-domain dedication. No copyright line in the first lines. |
| llama.cpp `tools/ui/src/lib/vendors/decimal.js/LICENCE.md` | `Copyright (c) 2025 Michael Mclaughlin` |
| llama.cpp `tools/ui/src/lib/vendors/nerdamer-prime/LICENSE` | `Copyright (c) 2023 together-science` |

`llama.cpp` `cmake/license.cmake` is a CMake helper that concatenates license
files. It is not itself a license.

## Still open

- `whisper-small-weights` has no verified SPDX and no verified notice.
- `LicenseRecord.notice` is the copyright line. The MIT permission notice and
  the Apache terms are not in the crate.
- Vendor, binding, example, and test LICENSE files above are not in the five
  records.
- No signed install evidence.

M0-AC4 and M0-AC5 stay open. R11 still blocks distribution of weights.
