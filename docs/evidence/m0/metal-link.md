# M0 Metal link probe (2026-10-16)

Measured in worktree `~/workspace/syndai/taco-m0-metal` on
`cursor/m0-metal-link-b19b`, parent `5e9649c` (`cursor/m0-pin-apply-b19b`).
One static Mach-O links whisper.cpp and llama.cpp on llama.cpp's ggml with
Metal shaders embedded. The probe does not load the pinned models and does not
cancel a worker. M0-AC2 and M0-AC3 stay open. This binary is not `taco`.

## Host

| Field | Value |
| --- | --- |
| Chip | Apple M1 Pro |
| Memory | 34359738368 bytes (32 GiB) |
| OS | macOS 27.0.1, build 26A434 |
| Arch | arm64 |
| rustc | 1.99.0 (`b940084d7`, 2026-09-28) |
| clang | Apple clang 21.0.0 (`clang-2100.3.34.2`) |
| cmake | 4.4.4 |
| When | 2026-10-17 00:02 KST |

This machine is not C1 and it is not a named C2.

## Engine commits

Shallow checkouts. Not vendored in git. `/.cache/` stays ignored.

| Tree | Commit | Note |
| --- | --- | --- |
| llama.cpp | `2a667032dc788c3a071d538e0b572e45a3f12731` | 2026-10-16, `2a66703`, ggml 0.27.0. cmake reports `llama.cpp version: 0.7.0-dev`. `src/models/hunyuan-dense.cpp` is in this tree. |
| whisper.cpp | `0c7be696371069e0baa2515ed7aa08d5a5473aa6` | 2026-10-12, `0c7be69`. cmake reports `whisper.cpp version: 1.9.5-dev`. Vendored ggml is 0.26.0 and was not compiled. |

Publisher card rechecked 2026-10-16 still says the GGUF depends on llama.cpp
PR 22836. `gh api repos/ggml-org/llama.cpp/pulls/22836` returned
`state=open`, `merged=false`, `merge_commit_sha=null`, head
`1e411d8f5a1e23525fa3265dfb4bd76265465397`, title
`ggml-cpu : add STQ1_0 ternary quantization with ARM NEON vec_dot kernel`.
That pull request is a CPU NEON kernel. It is not merged and it is not a Metal
backend.

The pinned `Hy-MT2-1.8B-Q4_K_M.gguf` header (GGUF v3, 354 tensors) has
`general.architecture=hunyuan-dense`, `general.file_type=15`, and tensor types
F32 (129), Q4_K (192), and Q6_K (33). No other type id is present. The file
was not loaded, so `2a66703` is a candidate that contains hunyuan-dense and
those quant types. It is not a demonstrated load of this GGUF.

Digests stay the ones in `pin-measurement.md`. A fresh `shasum -a 256` of
`~/workspace/syndai/taco-m0-pins-cache` matched those three lines. The probe
does not open the files.

## Build

`native/CMakeLists.txt` adds llama.cpp first, so the `ggml` target exists, then
adds whisper.cpp, which skips its own ggml. `BUILD_SHARED_LIBS=OFF`.
`GGML_METAL=ON` and `GGML_METAL_EMBED_LIBRARY=ON`. Examples, tools, server, and
app targets are off. Deployment target `14.2`.

```text
cmake -S native -B .cache/m0-link/build -DCMAKE_BUILD_TYPE=Release \
  -DLLAMA_SRC=$PWD/.cache/m0-engines/llama.cpp \
  -DWHISPER_SRC=$PWD/.cache/m0-engines/whisper.cpp
cmake --build .cache/m0-link/build -j8
```

Configure printed `ggml version: 0.27.0`, `ggml commit: 2a66703`,
`Including METAL backend`, then `whisper.cpp version: 1.9.5-dev`, then
`Configuring done`. The build log ends at `Built target taco_link_probe` and
`EXIT:0`. One `ggml-metal` static library is linked. There is no second ggml.

## Probe

`taco_engines_linked` calls `llama_supports_gpu_offload` and returns. It does
not call `llama_model_load_from_file` or `whisper_init_from_file_with_params`.

```text
$ file .cache/m0-link/build/taco_link_probe
.cache/m0-link/build/taco_link_probe: Mach-O 64-bit executable arm64

$ otool -L .cache/m0-link/build/taco_link_probe
	/System/Library/Frameworks/Accelerate.framework/Versions/A/Accelerate
	/usr/lib/libSystem.B.dylib
	/System/Library/Frameworks/Foundation.framework/Versions/C/Foundation
	/System/Library/Frameworks/Metal.framework/Versions/A/Metal
	/System/Library/Frameworks/MetalKit.framework/Versions/A/MetalKit
	/usr/lib/libc++.1.dylib
	/System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
	/usr/lib/libobjc.A.dylib
```

No `.dylib`, `.so`, `.metal`, or `.metallib` sits next to that executable.
The first run compiled Metal libraries from embedded data in 4.390 sec. A
rerun of the same binary printed:

```text
ggml_metal_library_init: using embedded metal library
ggml_metal_library_compile_all: loaded 36 libraries from embedded data in 0.035 sec
ggml_metal_device_init: GPU name:   MTL0 (Apple M1 Pro)
ggml_metal_device_init: GPU family: MTLGPUFamilyApple7  (1007)
ggml_metal_device_init: has unified memory    = true
ggml_metal_device_init: recommendedMaxWorkingSetSize  = 26800.60 MB
taco_engines_linked gpu_offload=1
```

Exit status 0. `gpu_offload=1` means the binary was built with GPU offload.
It does not mean a Hy-MT2 layer or a Whisper graph ran on the GPU.

## What this does not accept

- [ ] **M0-AC2:** No ASR pass and no translation pass. The pinned ONNX, ggml-small, and Q4_K_M files were not loaded.
- [ ] **M0-AC3:** No cancel during init or decode, and no join inside the 2 s reserve. `T-M0-CANCEL-LIFECYCLE` is still the closed panic.
- The `taco` binary from `cargo build` does not link these engines. `cargo test` does not build this probe.
- `T-M0-LINK-THREE-MODELS` and `T-M0-CANCEL-LIFECYCLE` stay fail-closed.
- A helper process, a sidecar dylib, and `-ngl 0` were not used, and they remain rejected.
