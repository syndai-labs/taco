# ORT Mach-O guards (2026-10-16)

ADR 0004 and the agent contract require one Mach-O. A shared `libonnxruntime`
dylib is not a ship path. Dynamic dependencies stay system frameworks. ORT, if
linked, is static archives inside that Mach-O.

`cursor/m0-metal-link-b19b` at `a8b2ebc` linked ONNX Runtime 1.31.0 as static
CPU archives (`onnxruntime_BUILD_SHARED_LIB=OFF`) into the probe. `otool -L`
listed only system frameworks. The official `onnxruntime-osx-arm64` archive ships `libonnxruntime.1.31.0.dylib` and was not linked. No sidecar dylib sat
next to that probe. Silero's provider there was CPU. This note does not re-run
that Mac build.

`native/CMakeLists.txt` still builds the product static library only. It does
not link ORT. `T-M0-LINK-THREE-MODELS` and `T-M0-CANCEL-LIFECYCLE` still panic.
Probe output is not a pass of those tests. M0-AC2 and M0-AC3 stay open.
