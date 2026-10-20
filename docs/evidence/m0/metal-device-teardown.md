# Metal device teardown (2026-10-20)

Stop. Not a pass. M0-AC2 and M0-AC3 stay unchecked.

This is a read of the pinned ggml. It is not a Metal run. This host is Linux.
No `phys_footprint` moved. #55 tip `0829a70` named the holder. #59 tip
`ed9ee10` already refuses an AC2/AC3 checkbox while that Stop stands. This
note does not add another flip guard.

## Pin

| Tree | Commit | What is linked |
| --- | --- | --- |
| llama.cpp | `2a667032dc788c3a071d538e0b572e45a3f12731` | ggml 0.27.0. cmake on the metal probe reported `ggml commit: 2a66703`. |
| whisper.cpp | `0c7be696371069e0baa2515ed7aa08d5a5473aa6` | Vendored ggml 0.26.0 is not compiled. |

`native/CMakeLists.txt` adds llama.cpp first, so the `ggml` target exists, then
adds whisper.cpp, which skips its own ggml. One `ggml-metal` static library.
`GGML_METAL=ON`. The metal-probe record is `docs/evidence/m0/metal-link.md` on
`a8b2ebc`. The checkout is `TACO_LLAMA_SRC`. It is not vendored here, so this
tree cannot compile a ggml patch. The lines below are from that commit.

## What lives until process exit

`ggml/src/ggml-metal/ggml-metal.cpp`

`static int g_devices = 1` is file scope. `ggml_backend_metal_reg()` may
overwrite it from `GGML_METAL_DEVICES` on every call, under its mutex. The
device objects are built only while `initialized` is false. After that the
integer can change and the vectors do not. The device count is
process-lifetime.

Function-local statics in `ggml_backend_metal_reg()`:

- `static ggml_backend_reg reg`
- `static bool initialized`
- `static std::mutex mutex`
- `static std::vector<ggml_backend_device_ptr> devs`
- `static ggml_backend_metal_reg_ptr reg_ctx`

`initialized` is set true and never cleared. `ggml/include/ggml-metal.h`
exports `ggml_backend_metal_reg()` and does not export a free.

`ggml_backend_metal_device_free` only `delete`s the `ggml_backend_device`
wrapper. It does not call `ggml_metal_device_free`.

The `MTLDevice` is owned by `ggml_metal_device_get` in
`ggml/src/ggml-metal/ggml-metal-device.cpp`:

```cpp
ggml_metal_device_t ggml_metal_device_get(int device, int n_devices) {
    static std::vector<ggml_metal_device_ptr> devs;
    devs.emplace_back(ggml_metal_device_init(device, n_devices));
    return devs.back().get();
}
```

Each call appends. Nothing removes. The `unique_ptr` deleter calls
`ggml_metal_device_free` when that static vector is destroyed, which is
process exit. `ggml_metal_device_init` does `MTLCreateSystemDefaultDevice()`
and `newCommandQueue`. `ggml_metal_device_free` releases the fusion info,
residency sets, Metal library, command queue, and device, then `free`s the
struct.

Three buffer-type caches in the same `ggml-metal.cpp` (`shared`, `private`,
`mapped`) each keep their own `static bool initialized` and store
`ggml_backend_reg_dev_get(ggml_backend_metal_reg(), i)` in a static vector.
First use freezes those device pointers for the process.

`ggml/src/ggml-backend-reg.cpp`: under `GGML_USE_METAL`, the
`ggml_backend_registry` constructor calls
`register_backend(ggml_backend_metal_reg())` and copies each
`ggml_backend_dev_t` into its device vector. That registry is a function-local
static (`get_reg()`). Its destructor says backends cannot be safely unloaded,
because threads may still be using the resources, and it does not destroy
them. `unload_backend` only erases the pointer. It does not reset
`ggml_backend_metal_reg`'s statics, and a static link has no `dl` handle to
drop.

`llama_backend_free` in `src/llama.cpp` is `ggml_quantize_free()`. It does not
touch Metal.

`ggml_backend_metal_free` waits, calls `ggml_metal_free` on that backend
context, and `free`s the backend. That is the path `whisper_free` and
`llama_free` already take. It releases that context's command buffers. The
comment on the queue release is `TAG_QUEUE_PER_BACKEND` and the release is
compiled out. The device in `ggml_metal_device_get` stays.

## Why a teardown from taco is a use-after-free

`taco_engines_prove` calls `llama_backend_free()` after `free_mt`. That matches
the pin: the call does not drop the device. #32/#49 already showed the
footprint staying up after that call. Cycle 2 `virtual_size` at
`after_whisper_free` was still 498401280 above `before_load` after the
487014400-byte `vm_deallocate`.

Calling `ggml_metal_device_free` on the pointer `ggml_metal_device_get`
returned:

1. The static `unique_ptr` still owns it and frees it again at process exit.
2. The registry wrappers, the three buffer-type statics, and
   `ggml_backend_registry::devices` still hold that device.
3. The next `prove_native` calls `ggml_backend_metal_reg()` and gets the same
   `initialized` registry.
4. `product::g_asr` and `product::g_mt` can still hold Metal backends in this
   process. A free at the end of prove use-after-frees those too. Prove's own
   `whisper_free` does not empty the statics.

Nulling only the registry pointer, which is the purge shape #55 described,
leaves the buffer-type statics and `ggml_metal_device_get`'s vector. This pin
has no function that clears those owners together. The registry destructor
refuses to invent one.

An autorelease-pool pop is the wrong lever. `ggml_metal_buffer_free` uses
`[buffer release]`, not `autorelease`. The #49 command-buffer waits already
missed the band.

## What this tree does not do

No patch under `native/patches/` for this device. No `ggml_metal_device_free`
call. No Linux path that pretends an `MTLDevice` was released. The prove line
stays `ac2=0 ac3=0`. The residency harness
(`tests/t_m0_residency.rs`) samples `phys_footprint` and does not release the
device. #59 tip `ed9ee10` is the checkbox lock.

## Mac measurement

Same host as #32/#49/#55: Apple M1 Pro, 34359738368 bytes, macOS 27.0.1
(26A434). One process. Do not start a new process per cycle. Do not treat one
cycle's dip as a pass.

```text
TACO_LLAMA_SRC=<pinned llama.cpp at 2a667032dc788c3a071d538e0b572e45a3f12731> \
TACO_WHISPER_SRC=<pinned whisper.cpp at 0c7be696371069e0baa2515ed7aa08d5a5473aa6> \
TACO_ORT_ROOT=<the ORT 1.31 static build> \
TACO_MODEL_DIR=<pin cache> \
TACO_RESIDENCY_CYCLES=8 \
cargo test --features native --test t_m0_residency -- --nocapture --test-threads=1
```

The harness prints `phys_footprint` at `after_teardown`. The engine prints
`residency_phase` at `before_load`, `after_whisper_free`, and
`after_backend_free`. Beside those three, sample
`MTLCreateSystemDefaultDevice().currentAllocatedSize` and
`recommendedMaxWorkingSetSize` for the process device.

That sample names the holder. It does not create a safe free.

- If `currentAllocatedSize` tracks the ~90 MiB cycle-2 step, the retainer is
  this process-lifetime device. Stop. Do not null `ggml_backend_metal_reg`
  from taco.
- If `currentAllocatedSize` stays flat while `phys_footprint` grows, the
  system default device is not the retainer. Interpose `mach_vm_map` /
  `vm_map` (the #49 probe only interposed `vm_allocate`) or take
  `vmmap -summary` at `after_whisper_free` and name the region of about
  487014400 bytes.

A later ggml that wants the bytes back has to destroy every owner in one
place: `ggml_metal_device_get`'s vector, the registry statics, the three
buffer-type statics, and `ggml_backend_registry::devices`, and only while no
whisper context, llama context, or `ggml_backend_t` is live. This pin says
that function does not exist.

Eight cycles first. Continue to 100 only if cycle 8 is within 33554432 of
cycle 1. `link_three` stays 1. The engine keeps printing `ac2=0` and `ac3=0`.
Acceptance stays open until cycle 100 on that host is ≤ the cycle-1 teardown
+ 33554432. This document does not record that run.

## Still open

- [ ] **M0-AC2:** Unchecked. #55 tip `0829a70` stands. #59 tip `ed9ee10` keeps the checkbox locked. No device was released here. The engine prints `ac2=0`.
- [ ] **M0-AC3:** Unchecked. `teardown=1` does not return `phys_footprint` to the cycle-1 baseline. The engine prints `ac3=0`.
