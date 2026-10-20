# Process taps on the product path

Status: **Partial**. `system`, `app:`, and `pid:` already run through product `taco run`. CoreAudio open and operator Allow/Deny stay Mac-only. Linux tap success is a Stop. M0-AC1 stays unchecked.

Allow tip SHA: none
Deny tip SHA: none

PR #11 tip `cd9a4c8` (`cd9a4c8e0c66727c7c66c50923419f8800950dda`, `cursor/m0-process-tap-b19b`) is an ancestor of stack base `a1c5c32` (`cursor/m0-ac1-mac-resume-b19b`). There is no second spike binary. `src/main.rs` `Commands::Run` calls `session::start_capture`. That function dispatches:

- `system` → `capture::open_system_tap`
- `app:<bundle>` → `capture::open_bundle_tap`
- `pid:<pid>` → `capture::open_pid_tap`

`taco` keeps `forbid(unsafe_code)`. The `unsafe` boundary is `taco-tap`. `native/tap/build.rs` compiles `native/process_tap.m` only when the target OS is macOS. That file calls `AudioHardwareCreateProcessTap`. It is not a stub. Unset `TACO_CAPTURE_SPIKE` returns `capture_not_available` and leaves `capture_started` false. On a host that is not macOS arm64, `taco run` returns `platform_unsupported` before any open, including when the spike env is set.

## #11 tip evidence (Mac, already recorded)

[process-tap.md](process-tap.md) on that tip. This Linux tree did not re-run it and did not click TCC.

- `TACO_CAPTURE_SPIKE=1` `--source system` exited 0. No dialog. Not Allow. Not Deny.
- `app:com.google.Chrome` and `pid:688` exited 0. `tccd` logged a microphone preflight only (`authValue=2`). No `AUTHREQ_PROMPTING`. No `kTCCServiceAudioCapture` line. Exit 0 is not an AudioCapture allow.
- `app:com.apple.Terminal` and its pid missed. CoreAudio's HAL client list does not include Terminal. That miss stays not-found. It is not a binding bug and it does not become a system tap.
- `EXTERNAL_INPUT.display_name` is `syndai-iphone-02 Microphone`. `qualified_in_m0` stays false.
- `T-M0-SPIKE-SOURCES` stays `verdict=not_run`.

## Still Mac-only

- Compiling and calling `AudioHardwareCreateProcessTap` (`native/process_tap.m`).
- Reproducing the #11 system and Chrome `app:` / `pid:` exit 0.
- An operator Allow and an operator Deny from `taco run`.
- An AudioCapture prompt and a separately traced destroy of the tap and private aggregate.

## Consequence

Partial — process taps are on taco run; Mac CoreAudio and operator Allow/Deny stay open; Linux tap success is Stop.

Checking M0-AC1 requires this Partial to be replaced by a Mac Allow tip SHA and a Mac Deny tip SHA written above. Neither SHA exists. A Linux `cargo test`, `platform_unsupported`, or the #11 exit 0 does not fill either line.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
