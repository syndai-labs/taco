# M0 process tap wiring

AC1 stays open. Rust now calls `native/process_tap.m` for `system`, `app:`, and `pid:` through the `taco-tap` crate. That crate is the `unsafe` boundary. `taco` still has `forbid(unsafe_code)`. The `.m` file is compiled only on macOS and uses CoreAudio process taps (`AudioHardwareCreateProcessTap`, private aggregate), not a stub.

`TACO_CAPTURE_SPIKE` gates every real open, same as the microphone path. Unset, `start_capture` returns `capture_not_available` and does not call the tap. On a non-macOS host the spike env returns `platform_unsupported` before any open. Linux `cargo test` does not set the env and does not open a device.

Tap open creates the process tap and private aggregate, then `Drop` destroys them. It does not start device IO. This tree does not record a TCC prompt.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.

## Mac follow-up

Still needs a signed arm64 binary, `TACO_CAPTURE_SPIKE=1`, and `taco run` for `default`, `system`, and `app:<bundle>` (or `pid:`) to show the real TCC / process-tap prompt, allow, deny, and cleanup. Do not check AC1 from this wiring alone.
