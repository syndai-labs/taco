# Launchd user agent run

Cited from `cursor/m0-launchd-host-b19b` tip `0c880fb` (PR #43). This branch does not replay `scripts/m0-launchd-host.sh`, the plist, or the `freeze.rs` edit that sets `ALTERNATE_HOST_EXERCISED`. No acceptance box is checked.

On that tip the alternate host named in [os-floor.md](os-floor.md) was loaded, ran, and removed. `ALTERNATE_HOST_EXERCISED` is true there for this load, run, and teardown only. The OS floor stays Partial. C2 stays open. This is not a TCC allow or deny. No microphone button was pressed. No Metal load was run. `taco run` was not started.

Label `ai.syndai.taco.m0-launch-host`, domain `gui/501`, type LaunchAgent. `launchctl bootstrap` exited 0. `RunAtLoad` started one run. The log has no `kickstart` line. The program was `scripts/m0-launchd-host-stub.sh`, which execs that worktree's `target/debug/taco notices` and exits. `KeepAlive` is false. `launchctl print` after the run: `state = not running`, `runs = 1`, `last exit code = 0`. Job stdout is the `notices` text. Job stderr was empty. `launchctl bootout gui/501/ai.syndai.taco.m0-launch-host` exited 0. A following `launchctl print` exited 113.

That binary is not the signed `ai.syndai.taco` acceptance artifact. `codesign -dv` reported `Identifier=taco-a8e556d459e2954c`, `Signature=adhoc`, `flags=0x20002(adhoc,linker-signed)`, `Info.plist=not bound`, `TeamIdentifier=not set`. SHA-256 `b1d9be233a01d1a26b42b9a2241c4b09502167c899664b20cd6a609c549911f8`.

Host recorded on that tip: hostname `syndai-macbook-pro-01`, `ProductVersion` 27.0.1, `BuildVersion` 26A434, uname `Darwin syndai-macbook-pro-01 27.0.0 Darwin Kernel Version 27.0.0: Tue Aug 11 21:22:49 PDT 2026; root:xnu-13432.1.9~1/RELEASE_ARM64_T6000 arm64`. Start `2026-10-16T16:59:04Z`. Script exit 0. The note does not record a memory byte count and does not name C2. An earlier bootstrap in the same minute was removed while launchd still reported `state = xpcproxy` and `execs = 0`. That attempt is not this measurement. After bootout, `ps -p 76134` still showed `taco run --source default --from en --to zh`.

- [ ] **M0-AC1:** No microphone button was pressed.
- [ ] **M0-AC4:** The launchd load is not a release cell and not a signed install.
