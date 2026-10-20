# Metal device refcount (2026-10-20)

Stop. Not a pass. M0-AC2 and M0-AC3 stay unchecked.

This reads pinned llama.cpp `2a667032dc788c3a071d538e0b572e45a3f12731` again, after #63 tip `232ea78` and #55 tip `0829a70`. It asks whether a refcount can free the `MTLDevice` once no backend context remains, and whether cycle 2 of `prove_native` can use that device again. It cannot, not from this tree. This host is Linux. No Mac `phys_footprint` was recorded for this refcount. No `currentAllocatedSize` was sampled. Nothing here is a Metal run.

#63 already refused `ggml_metal_device_free` from taco: the static `unique_ptr` in `ggml_metal_device_get` frees the same struct at process exit, and the registry, the three buffer-type caches, and `ggml_backend_registry::devices` still hold it. This note is the next shape, a refcount, and why that shape fails on the same pin. The charter test is `metal_refcount_stop_cites_63_and_55`. It is not `ac2_ac3_cannot_flip_while_stop_shas_stand`.

## What a refcount would have to count

`ggml_backend_metal_free` is not a device free. It synchronizes that backend, calls `ggml_metal_free`, and `free`s the backend struct. The command queue is the device queue. The per-backend queue release is compiled out (`TAG_QUEUE_PER_BACKEND`).

Live users of one `ggml_metal_device_t` are not the backend alone:

- `ggml_metal_t` stores `dev`, and also `lib` and `finfo` taken from the device at `ggml_metal_init`.
- `ggml_metal_buffer_t` stores `dev` and the `MTLBuffer`s. A buffer can still be alive after that backend returns.
- `ggml_metal_rsets` runs a `dispatch_group_async` heartbeat that calls `requestResidency` until `d_stop`. `ggml_metal_rsets_free` asserts the set array is empty, then joins the thread. A non-empty array aborts the process.
- `product::g_asr` and `product::g_mt` are Metal backends in this same process. A count that only watches prove's `whisper_free` / `free_mt` drops the device under those.

`llama_backend_free` cannot host the count. It is `ggml_quantize_free()`.

## Why "free the struct at zero" is a use-after-free

`ggml_metal_device_get` does not look up a device. It appends one:

```cpp
ggml_metal_device_t ggml_metal_device_get(int device, int n_devices) {
    static std::vector<ggml_metal_device_ptr> devs;
    devs.emplace_back(ggml_metal_device_init(device, n_devices));
    return devs.back().get();
}
```

The deleter calls `ggml_metal_device_free`. That runs once, at process exit, unless the vector is cleared earlier. Clearing it frees the struct. The next `prove_native` does not call `ggml_metal_device_get` again: `ggml_backend_metal_reg`'s `initialized` flag stays true, and `register_backend` returns immediately when the same `ggml_backend_reg_t` is already present. Cycle 2 still holds the pointer captured on cycle 1.

These owners are that pointer, not a fresh lookup:

| Owner | What it keeps |
| --- | --- |
| `ggml_backend_metal_reg` static `devs` | `unique_ptr` of the `ggml_backend_device` wrapper. Its `context` is the `ggml_metal_device_t`. The wrapper deleter only `delete`s the wrapper. |
| `reg_ctx->devices` | the same wrapper pointers |
| `ggml_backend_registry::devices` | a copy taken in the constructor under `GGML_USE_METAL` |
| buffer types `shared`, `private`, `mapped` | `buft.device` from `ggml_backend_reg_dev_get` on first use; `initialized` is never cleared |

`~ggml_backend_registry` says backends cannot be unloaded, because threads may still be using them. It does not destroy them. `unload_backend` only erases the pointer. There is no `dl` handle on this static link.

`ggml_metal_fusion_info_free` is `delete finfo`. `ggml_metal_device_free` calls it. A taco free that leaves the `unique_ptr` in place frees the fusion info twice at exit.

## Why "release MTL objects, keep the struct" still breaks cycle 2

The less wrong shape leaves the wrapper and the `ggml_metal_device` struct allocated, releases `library`, `mtl_queue`, and `mtl_device` when the count hits zero, and rebuilds them on the next use. The pin has no rebuild.

Cycle 2 reaches the device before it creates a backend. `llama_prepare_model_devices` calls `ggml_backend_dev_get_props` (llama.cpp around the device list, and again when it logs `memory_free`). Metal's `ggml_backend_metal_device_get_props` always calls `ggml_metal_device_get_memory`:

```objc
*total     = dev->mtl_device.recommendedMaxWorkingSetSize;
size_t cur = dev->mtl_device.currentAllocatedSize;
```

There is no nil check. A released device reports `0` bytes free, because a message to `nil` returns zero. Device selection on that cycle is then a lie.

The next `ggml_metal_init` is undefined, not a clean failure. It logs the name before it tests the queue:

```objc
id<MTLDevice> device = ggml_metal_device_get_obj(dev);
GGML_LOG_INFO("%s: picking default device: %s\n", __func__, [[device name] UTF8String]);
id<MTLCommandQueue> queue = ggml_metal_device_get_queue(dev);
if (queue == nil) {
```

`[nil name]` is nil. `%s` of the following `UTF8String` is undefined. The queue check never saves that. `ggml_metal_init` also keeps `res->lib` and `res->finfo` from the device it was given. Those pointers die with the release.

Rebuilding without a second `ggml_metal_device_get` means splitting `ggml_metal_device_init`. That function `calloc`s the struct, calls `MTLCreateSystemDefaultDevice`, compiles the library, probes the tensor API, and starts the residency heartbeat. It is not re-entrant on an existing struct. A second heartbeat or a second library on the same struct is the leak. A missed nil on `mtl_device` is the use-after-free. `ggml_metal_rsets_free` still aborts if any set is left, so the release cannot run while a buffer's residency set exists, and it must join the heartbeat before `[mtl_device release]`.

That rebuild would be a fork of `ggml-metal-device.m`, `ggml-metal-context.m`, and `ggml-metal.cpp` inside `TACO_LLAMA_SRC`. The checkout is not vendored. This tree cannot compile it. Linux must not pretend the second `prove_native` cycle was run.

## What this tree does not do

No patch under `native/patches/`. CMake does not apply one. `taco_runtime.cpp` does not call `ggml_metal_device_free`. The prove line stays `ac2=0 ac3=0`. The residency harness still only samples `phys_footprint`.

Upstream has to add the missing function, on this pin or a later one, before a Mac remeasure means anything. One function, under the metal registry mutex, and only when the count of live `ggml_metal_t` and `ggml_metal_buffer_t` is zero and the residency-set array is empty:

- join the heartbeat, then release the library, the queue, and the `MTLDevice`, and nil those fields
- leave the `ggml_backend_device` wrapper and `ggml_backend_registry::devices` in place
- on the next `ggml_backend_dev_get_props` or `ggml_metal_init`, rebuild that same struct without calling `ggml_metal_device_get` again
- do nothing when `product::g_asr` or `product::g_mt` still holds a context or a buffer

`2a66703` does not have that function. The registry destructor is the upstream note that it is missing.

## Mac measurement

Same host as #32, #49, and #55: Apple M1 Pro, 34359738368 bytes, macOS 27.0.1 (26A434). One process. Do not start a new process per cycle. Do not apply a device-free patch first. This tree does not contain one, and applying one is the use-after-free above.

```text
TACO_LLAMA_SRC=<pinned llama.cpp at 2a667032dc788c3a071d538e0b572e45a3f12731> \
TACO_WHISPER_SRC=<pinned whisper.cpp at 0c7be696371069e0baa2515ed7aa08d5a5473aa6> \
TACO_ORT_ROOT=<the ORT 1.31 static build> \
TACO_MODEL_DIR=<pin cache> \
TACO_RESIDENCY_CYCLES=8 \
cargo test --features native --test t_m0_residency -- --nocapture --test-threads=1
```

The harness prints `phys_footprint` at `after_teardown`. The engine prints `residency_phase` at `before_load`, `after_whisper_free`, and `after_backend_free`. Beside those three, sample `MTLCreateSystemDefaultDevice().currentAllocatedSize` and `recommendedMaxWorkingSetSize`.

That sample still does not make a free safe. #63 tip `232ea78` and #55 tip `0829a70` stay in force until upstream lands the function above and a later Mac run, on that patched tree, shows cycle 100 ≤ cycle 1 teardown + 33554432, with `link_three` still 1. Eight cycles first. Continue to 100 only if cycle 8 is within 33554432 of cycle 1. The engine keeps printing `ac2=0` and `ac3=0` until that run exists. This document does not record that run.

## Still open

- [ ] **M0-AC2:** Unchecked. #63 tip `232ea78` and #55 tip `0829a70` stand. No refcount patch was applied. The engine prints `ac2=0`.
- [ ] **M0-AC3:** Unchecked. `teardown=1` does not return `phys_footprint` to the cycle-1 baseline. No Mac `phys_footprint` was recorded for this refcount. The engine prints `ac3=0`.
