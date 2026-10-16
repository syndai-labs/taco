# Launchd user agent run (2026-10-16)

The alternate terminal host named in [os-floor.md](os-floor.md) was loaded, ran, and removed. M0-AC1 and M0-AC4 stay unchecked. This is not a TCC allow or deny. No microphone button was pressed. No Metal load was run. `taco run` was not started. The existing `taco-m0-tap` capture process was not signaled.

`ALTERNATE_HOST_EXERCISED` is true for this load, run, and teardown only. The OS floor stays Partial. C2 stays open.

## What ran

Label `ai.syndai.taco.m0-launch-host`, domain `gui/501`, type LaunchAgent. `launchctl bootstrap` loaded the rendered plist. `RunAtLoad` started one run. The log has no `kickstart` line. The program was `scripts/m0-launchd-host-stub.sh`, which execs this worktree's `target/debug/taco notices` and exits. `KeepAlive` is false.

That binary is not the signed `ai.syndai.taco` acceptance artifact. `codesign -dv` reported `Identifier=taco-a8e556d459e2954c`, `Signature=adhoc`, `flags=0x20002(adhoc,linker-signed)`, `Info.plist=not bound`, `TeamIdentifier=not set`. SHA-256 `b1d9be233a01d1a26b42b9a2241c4b09502167c899664b20cd6a609c549911f8`.

`launchctl print` after the run: `state = not running`, `runs = 1`, `last exit code = 0`. Job stdout is the `notices` text. Job stderr was empty.

`launchctl bootout gui/501/ai.syndai.taco.m0-launch-host` exited 0. A following `launchctl print` exited 113: `Could not find service "ai.syndai.taco.m0-launch-host" in domain for user gui: 501`.

After bootout, `ps -p 76134` still showed `/Users/jwu/workspace/syndai/taco-m0-tap/target/debug/taco run --source default --from en --to zh`.

An earlier bootstrap in the same minute was removed while launchd still reported `state = xpcproxy` and `execs = 0`. That attempt produced no job stdout and is not this measurement. This run started with the service absent (`pre_print_exit=113`).

## Host

| Field | Value |
| --- | --- |
| hostname | syndai-macbook-pro-01 |
| uname | Darwin syndai-macbook-pro-01 27.0.0 Darwin Kernel Version 27.0.0: Tue Aug 11 21:22:49 PDT 2026; root:xnu-13432.1.9~1/RELEASE_ARM64_T6000 arm64 |
| sw_vers | macOS 27.0.1 (26A434) |
| start | 2026-10-16T16:59:04Z |
| bootstrap | 2026-10-16T16:59:04Z, exit 0 |
| print | 2026-10-16T16:59:05Z, exit 0 |
| bootout | 2026-10-16T16:59:05Z, exit 0 |
| print after bootout | 2026-10-16T16:59:05Z, exit 113 |
| end | 2026-10-16T16:59:05Z |
| job exit | 0 |

Command: `scripts/m0-launchd-host.sh` from `cursor/m0-launchd-host-b19b`, based on `8ebd0fc`. Script exit 0.

## Transcript

```text
timestamp_start=2026-10-16T16:59:04Z
hostname=syndai-macbook-pro-01
uname=Darwin syndai-macbook-pro-01 27.0.0 Darwin Kernel Version 27.0.0: Tue Aug 11 21:22:49 PDT 2026; root:xnu-13432.1.9~1/RELEASE_ARM64_T6000 arm64
ProductName:		macOS
ProductVersion:		27.0.1
BuildVersion:		26A434
uid=501
domain=gui/501
label=ai.syndai.taco.m0-launch-host
stub=/Users/jwu/workspace/syndai/taco-m0-launchd-host/scripts/m0-launchd-host-stub.sh
pre_print_exit=113
pre_print_stderr=Bad request.
Could not find service "ai.syndai.taco.m0-launch-host" in domain for user gui: 501
timestamp_build=2026-10-16T16:59:04Z
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
taco_bin=/Users/jwu/workspace/syndai/taco-m0-launchd-host/target/debug/taco
taco_sha256=b1d9be233a01d1a26b42b9a2241c4b09502167c899664b20cd6a609c549911f8
timestamp_codesign=2026-10-16T16:59:04Z
Executable=/Users/jwu/workspace/syndai/taco-m0-launchd-host/target/debug/taco
Identifier=taco-a8e556d459e2954c
Format=Mach-O thin (arm64)
CodeDirectory v=20400 size=31438 flags=0x20002(adhoc,linker-signed) hashes=979+0 location=embedded
Signature=adhoc
Info.plist=not bound
TeamIdentifier=not set
Sealed Resources=none
Internal requirements=none
timestamp_plutil=2026-10-16T16:59:04Z
/Users/jwu/workspace/syndai/taco-m0-launchd-host/target/m0-launchd-host/ai.syndai.taco.m0-launch-host.plist: OK
plutil_exit=0
rendered_plist_begin
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>Label</key>
	<string>ai.syndai.taco.m0-launch-host</string>
	<key>Comment</key>
	<string>M0 alternate terminal host. User agent. Substitute absolute paths for the stub, stdout, and stderr placeholders before bootstrap. The stub execs this worktree's target/debug/taco notices and exits. It does not open a capture device. Do not bootstrap this template.</string>
	<key>ProgramArguments</key>
	<array>
		<string>/Users/jwu/workspace/syndai/taco-m0-launchd-host/scripts/m0-launchd-host-stub.sh</string>
	</array>
	<key>RunAtLoad</key>
	<true/>
	<key>KeepAlive</key>
	<false/>
	<key>ProcessType</key>
	<string>Background</string>
	<key>StandardOutPath</key>
	<string>/Users/jwu/workspace/syndai/taco-m0-launchd-host/target/m0-launchd-host/stdout.log</string>
	<key>StandardErrorPath</key>
	<string>/Users/jwu/workspace/syndai/taco-m0-launchd-host/target/m0-launchd-host/stderr.log</string>
</dict>
</plist>
rendered_plist_end
timestamp_bootstrap=2026-10-16T16:59:04Z
bootstrap_exit=0
timestamp_print=2026-10-16T16:59:05Z
print_exit=0
saw_last_exit_code=1
print_begin
gui/501/ai.syndai.taco.m0-launch-host = {
	active count = 0
	path = /Users/jwu/workspace/syndai/taco-m0-launchd-host/target/m0-launchd-host/ai.syndai.taco.m0-launch-host.plist
	type = LaunchAgent
	state = not running

	program = /Users/jwu/workspace/syndai/taco-m0-launchd-host/scripts/m0-launchd-host-stub.sh
	arguments = {
		/Users/jwu/workspace/syndai/taco-m0-launchd-host/scripts/m0-launchd-host-stub.sh
	}

	stdout path = /Users/jwu/workspace/syndai/taco-m0-launchd-host/target/m0-launchd-host/stdout.log
	stderr path = /Users/jwu/workspace/syndai/taco-m0-launchd-host/target/m0-launchd-host/stderr.log
	inherited environment = {
		SSH_AUTH_SOCK => /var/run/com.apple.launchd.RsPFxzpUDh/Listeners
	}

	default environment = {
		PATH => /usr/bin:/bin:/usr/sbin:/sbin
	}

	environment = {
		OSLogRateLimit => 64
		XPC_SERVICE_NAME => ai.syndai.taco.m0-launch-host
	}

	domain = gui/501 [100019]
	asid = 100019
	minimum runtime = 10
	exit timeout = 5
	runs = 1
	last exit code = 0

	resource coalition = {
		ID = 14806
		type = resource
		state = active
		active count = 1
		name = ai.syndai.taco.m0-launch-host
	}

	jetsam coalition = {
		ID = 14807
		type = jetsam
		state = active
		active count = 1
		name = ai.syndai.taco.m0-launch-host
	}

	spawn type = background (5)
	jetsam priority = 40
	jetsam memory limit (active) = (unlimited)
	jetsam memory limit (inactive) = (unlimited)
	jetsamproperties category = daemon
	jetsam thread limit = 32
	cpumon = default
	sanitizer flags = 0x0

	properties = partial import | runatload | inferred program
}
print_end
timestamp_bootout=2026-10-16T16:59:05Z
bootout_exit=0
timestamp_print_after=2026-10-16T16:59:05Z
print_after_exit=113
print_after_stderr_begin
Bad request.
Could not find service "ai.syndai.taco.m0-launch-host" in domain for user gui: 501
print_after_stderr_end
timestamp_end=2026-10-16T16:59:05Z
job_stdout_begin
taco shows subtitles in this terminal and does not upload them.
Model digests are TBD. Terminal scrollback can keep what is displayed.
usage keys linked: true
job_stdout_end
job_stderr_begin
job_stderr_end
job_exit_code=0
```

## What this does not show

- No operator allow and no operator deny. `responsibility_spawnattrs_setdisclaim` was not called. That disclaim path was not measured.
- Cursor.app (`com.todesktop.230313mzl4w4u92`) is still not an acceptance host.
- Apple Terminal was not a completed parent. A second GUI terminal was not measured.
- C2 still has no chip, memory, or OS build. This Mac is not a second machine.
- Release cells and host-swap sessions stay not run.
