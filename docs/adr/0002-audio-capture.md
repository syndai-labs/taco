# ADR 0002: Explicit native macOS source capture

Status: **Proposed**. Validation: M0 distribution/permission spike; M2 inputs;
M5 system and application sources.

## Context and goals

The charter requires selecting one microphone, input device, system stream, or
application. These are four qualification categories, with microphone and other
inputs sharing an adapter where practical. A system stream may contain multiple
applications because the OS mixes its output; taco itself still selects one source
and does not mix independent captures.

## Non-goals

No virtual audio driver installation, capture of protected content, microphone plus
system mixing, silent source fallback, automatic application relaunch, or capture
on unsupported OS versions. File replay belongs to the development test harness,
not a v0.1 user recording/playback feature.

## Decision

Use CPAL/CoreAudio inputs for microphones and input devices. Use CoreAudio process
taps with a private aggregate device for system/application output. The proposal
of macOS 14.2 on Apple Silicon was not measured. The acceptance floor is Partial:
macOS 27.0.1 (26A434), the only build in the C1 record. That is not a support
promise. Do not claim every macOS version or every application is supported.
See [os-floor.md](../evidence/m0/os-floor.md).

Apple's sample describes system/process taps on macOS 14.2 and later, aggregate
devices, and the `NSAudioCaptureUsageDescription` privacy declaration. Microphone
access needs its own `NSMicrophoneUsageDescription` declaration. Apple's
code-signing documentation also permits embedding an Info.plist in a single-file tool. Neither
fact alone proves that taco's signed CLI will receive the intended TCC attribution
in every terminal host; that is an explicit hardware/distribution gate.

Before a session, enumerate capabilities, resolve exactly one source, verify models
and language settings, then acquire permission/start capture. A listing command must
not start recording. If enumeration requires permissions on a tested OS, explain
that limitation before requesting them. Record observable behavior in the support
matrix rather than promising a prompt-free API without testing it.

## Selection and lifecycle

Selectors are only:

```text
default
input:<device-uid>
system
app:<bundle-id>
pid:<pid>
```

Display names are labels. A bare name, an unscoped bundle id, stdin, `-`, or an
audio-file path is rejected. Syntax that is not in the list above exits 2. A
syntactically valid selector that does not resolve exits 1. `default` is the
default input,
resolved once at session start and pinned. A later default-device change does not
move the session.

- `input:<device-uid>` uses the stable CoreAudio UID. Duplicate names stay
  distinct. Runtime PIDs are not durable device identity.
- `app:<bundle-id>` captures one application instance. Zero matches or several
  matches fail with the instance list and do not capture. The user reruns with
  `pid:<pid>`. A failed application selection never becomes system capture.
- `pid:<pid>` pins that process for this session. Exit or replacement of the pid
  stops the session. A new pid requires a new invocation.
- Within the one selected instance, M0/M5 records which helper processes belong to
  that instance. Helpers of a different instance are out of scope. If helper
  attribution is unreliable, capture only the explicit pid and say so in status.
  Never broaden a failed application selection to system-wide capture.
- The private aggregate device is not set as the system default output. Tap capture
  does not mute or reroute playback. Protected, unsupported, silent, and vanished
  sources have distinct statuses where the OS permits diagnosis. Silence alone is
  not permission denial.
- Permission prompts occur at most once per session attempt. A denial returns a
  typed error and recovery text. Do not loop the prompt. Listing sources must not
  start capture; if enumeration itself needs permission, say so before asking, and
  still ask at most once.
- On source removal, app exit, format discontinuity, sleep/wake, or unrecoverable
  callback error, report the event and stop the session cleanly. v0.1 requires an
  explicit new session to reconnect. Existing committed outcomes remain immutable.
- A system/app output-route change also stops the session, even if sample format
  stays the same. Do not assume continuity or follow another output route silently.
  An unrelated output-route change does not stop a pinned microphone/input unless
  that input itself changes or loses continuity. Test both same-format and
  format-changing route changes.
- Teardown stops IO callbacks before releasing callback-owned buffers, then destroys
  only taco-owned aggregate devices/taps and listeners. Partial initialization rolls
  back in reverse order. Never remove unrelated OS audio objects.

Normalize supported native formats to mono 16 kHz float audio off the callback.
Specify deterministic channel downmixing, clipping/non-finite handling, and sample
clock mapping. Pin the resampler implementation in M0/M2; preserve a continuous
resampler state only within a source generation with no gap. Report discontinuities
instead of fabricating silence that would imply source continuity.

## Alternatives and consequences

ScreenCaptureKit or a virtual driver may be investigated if taps fail, but neither
is a silent substitute for the charter's selected path. Raise an ADR revision with
the deployment/permission tradeoff. CPAL alone does not establish app-output capture.
Native taps add unsafe FFI and cleanup obligations. Keeping 14.2 as a confirmed
floor would claim a build this project has not run. The Partial revision drops
that claim and leaves every build older than 26A434 outside the floor until one
is measured.

## Acceptance criteria

- [ ] The actual single-file artifact captures each promised source category on the
  minimum and a current supported OS, with permission allow/deny/revoke cases.
- [ ] Two same-name devices/processes remain distinguishable; wrong-source capture
  is absent in isolation tests using different audible fixture signals.
- [ ] App helper processes, app exit/restart, output-route changes, source removal,
  and sleep/wake have recorded, deterministic outcomes.
- [ ] Repeated start/fail/stop cycles leave no taco-owned taps, aggregate devices,
  callbacks, or listeners behind and do not change system playback routing.
- [ ] 44.1/48 kHz mono/stereo fixtures preserve timing and speech through conversion;
  callback saturation cannot corrupt buffers or cause a use-after-free.

## Primary references

- [Apple: Capturing system audio with Core Audio taps](https://developer.apple.com/documentation/coreaudio/capturing-system-audio-with-core-audio-taps)
- [Apple: Adding an Info.plist to single-file tools](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/Procedures/Procedures.html)
- [Apple: Microphone usage description](https://developer.apple.com/documentation/bundleresources/information-property-list/nsmicrophoneusagedescription)
- [CPAL upstream](https://github.com/RustAudio/cpal)

References inform feasibility; exact SDK versions and packaging evidence are M0
deliverables, not supplied by these links.
