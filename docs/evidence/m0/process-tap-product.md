# Process taps on the product path

Status: **Partial**. `system`, `app:`, and `pid:` already run through product `taco run`. CoreAudio open, operator Allow/Deny, and a separately traced destroy stay Mac-only. Linux tap success is a Stop. This file is the measure contract. It is not a Mac session. M0-AC1 stays unchecked.

Allow tip SHA: none
Deny tip SHA: none
Cleanup tip SHA: none

M0-AC1 stays unchecked while Allow, Deny, or Cleanup tip SHA is still none. When a Cleanup tip SHA or Destroy tip SHA line is present, `none` on that line also refuses the flip.

Product-path freeze is #75 tip `dc1b9be` (`dc1b9bef6735d37315e4a32ef0893dd2ac20df11`), brought onto this session branch. That SHA is not an Allow, Deny, or Cleanup tip.

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
- `EXTERNAL_INPUT.display_name` is `syndai-iphone-02 Microphone`, frozen at tip `cd9a4c8`. `qualified_in_m0` stays false. The reserved name does not prove external input. No iPhone capture ran on this tip.
- `T-M0-SPIKE-SOURCES` stays `verdict=not_run`.
- Exit 0 is not a Cleanup tip SHA. The note on that tip says Drop destroys the tap and the private aggregate, and that destroy was not separately traced in a log. Drop is not a Cleanup tip SHA.

## Same sheet as AudioCapture `system`

`scripts/m0-audiocapture-measure.sh system` launches Taco.app (`ai.syndai.taco`) with `taco run --source system`. That is `open_system_tap`. The sheet is `kTCCServiceAudioCapture`. An Allow and a Deny on that system launch are the system process-tap TCC clicks.

They are not copied into the tip SHA lines above. This tree did not show the sheet. A second system Allow/Deny after that pair is the same service, not a new decision. `scripts/m0-ac1-full-session.sh` therefore runs `scripts/m0-process-tap-measure.sh after-pair` only once the AudioCapture pair is complete and no `taco run --source default` is left. `after-pair` does not reset TCC and does not open `system` again.

The app source on that step is `app:com.google.Chrome`, the HAL client recorded in [process-tap.md](process-tap.md). Terminal is not a HAL client. A missing sheet while AudioCapture is already granted or denied is the same service, not Deny. `capture_not_available` is not Deny.

Standalone `scripts/m0-process-tap-measure.sh --arm` is the other entry: system Allow, then Don't Allow, then one Chrome launch. Do not run it after the full session. That would prompt the same system sheet again.

## Cleanup is a third tip

`taco_process_tap_open` calls `AudioDeviceStop` before it returns. `Drop` of the tap value calls `AudioHardwareDestroyAggregateDevice` and `AudioHardwareDestroyProcessTap` (`taco_process_tap_close`). Neither call writes a log. Process exit is not a trace of those calls. Drop implied by exit 0 is not a Cleanup tip SHA.

A Cleanup tip SHA is a git SHA of a Mac commit whose log quotes both destroy calls from that run. Linux `cargo test`, `platform_unsupported`, and the #11 exit 0 do not qualify. `destroy_traced` in the measure record stays `no` until that Mac commit exists.

A hex Cleanup tip SHA without `destroy_traced: yes` refuses a checked M0-AC1. Missing either `AudioHardwareDestroyAggregateDevice` or `AudioHardwareDestroyProcessTap` refuses it too. Cleanup tip SHA stays `none` on this tip.

## When to run

Watcher does not click. `OPEN_MS` is `300000`. `HOLD_MS` is `1800000`. Both are pinned in the script. Inherited values are ignored. The tap window is `OPEN_MS`. `HOLD_MS` is passed so the Taco.app launch matches the mic harness. The tap does not read it.

Off macOS, `--arm`, `gate`, `after-pair`, and every selector exit 2 and print `platform_unsupported`. That is not a CoreAudio result.

One Mac worker start, only when no `taco run --source default` is live:

```sh
cargo build && ./scripts/sign-debug.sh
./scripts/m0-ac1-full-session.sh --arm
```

That packs `packaging/Taco.app`, runs the mic resume, then the AudioCapture system pair, then `after-pair`. If the mic watch is still live, the script does not start the process-tap measure, does not kill that pid, and does not reset TCC.

Direct use, only when that mic process is gone and the AudioCapture system pair has not already been clicked:

```sh
./scripts/m0-process-tap-measure.sh gate
./scripts/m0-process-tap-measure.sh --arm
```

`gate` prints `mic settled` when the mic process is gone. If it prints `mic LIVE`, stop.

Legal click outcomes are Allow, Deny, and TIMEOUT. TIMEOUT is not Deny. A pair-query `denied` with `ai.syndai.taco` absent from `tccutil list -s AudioCapture` is not Deny. Exit 0 is not Allow.

## Record

The launch prints this block. Leave it blank in this file. `process_exit` is not an outcome. `destroy_traced: no` is not a Cleanup tip SHA.

```text
outcome:
bundle: ai.syndai.taco
source:
open_ms: 300000
hold_ms: 1800000
click:
tcc_list:
tcc_pair:
row_present:
process_exit:
destroy_traced: no
drop_is_cleanup: no
cleanup_tip_sha: none
tip_sha:
```

| Run | outcome | source | click | row_present | destroy_traced | tip_sha |
| --- | --- | --- | --- | --- | --- | --- |
| Allow | | | | | | |
| Deny | | | | | | |
| Cleanup | | | | | no | |

## Still Mac-only

- Compiling and calling `AudioHardwareCreateProcessTap` (`native/process_tap.m`).
- Reproducing the #11 system and Chrome `app:` / `pid:` exit 0.
- An operator Allow and an operator Deny from `taco run` on Taco.app.
- A log that separately traces `AudioHardwareDestroyAggregateDevice` and `AudioHardwareDestroyProcessTap`.

## Consequence

Partial — process taps are on taco run; Mac CoreAudio and operator Allow/Deny stay open; Linux tap success is Stop.

Checking M0-AC1 requires a Mac Allow tip SHA, a Mac Deny tip SHA, and a Mac Cleanup tip SHA written above. All three are `none`. A Linux `cargo test`, `platform_unsupported`, process exit, Drop, or the #11 exit 0 does not fill any line.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
