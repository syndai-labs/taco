#!/bin/sh
# Measure M0 model digests on the host that will pin them.
# Prints sha256 + filename only. Does not edit freeze.rs and does not invent digests.
# Run on macOS Apple Silicon with network; record output in docs/evidence/m0/.
set -eu

dir=${1:-${TMPDIR:-/tmp}/taco-m0-pins}
mkdir -p "$dir"
cd "$dir"

fetch() {
  name=$1
  url=$2
  if [ ! -f "$name" ]; then
    echo "fetch $name" >&2
    curl -fL --retry 3 -o "$name.partial" "$url"
    mv "$name.partial" "$name"
  fi
  sum=$(shasum -a 256 "$name" | awk '{print $1}')
  size=$(wc -c <"$name" | tr -d ' ')
  printf 'role_file=%s sha256=%s bytes=%s\n' "$name" "$sum" "$size"
}

# Whisper small multilingual ggml (whisper.cpp models).
fetch ggml-small.bin \
  "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin"

# Silero ONNX used by whisper.cpp / common VAD path. Prefer the revision whisper.cpp pins once linked.
fetch silero_vad.onnx \
  "https://github.com/snakers4/silero-vad/raw/master/src/silero_vad/data/silero_vad.onnx"

# Publisher main-repo GGUF (not 1.25-bit / 2-bit siblings). Q4_K_M is a measurement candidate only.
fetch Hy-MT2-1.8B-Q4_K_M.gguf \
  "https://huggingface.co/tencent/Hy-MT2-1.8B-GGUF/resolve/main/Hy-MT2-1.8B-Q4_K_M.gguf"

echo "dir=$dir"
echo "note=digests above are measured on this host; copy into freeze::PINS only after review"
echo "note=do not flip PINS_MEASURED until filenames and digests are recorded together"
