# M0 pin measurement (2026-10-16)

Host digests measured on this Mac. A follow-up commit records them in `freeze::PINS`
with `PINS_MEASURED=true`. No Metal load was run. No TCC prompt was shown.
M0-AC2 and other acceptance boxes stay open.

## Command

Run from `~/workspace/syndai/taco-m0-pins`:

```text
./scripts/measure-pins.sh ~/workspace/syndai/taco-m0-pins-cache 2>&1 | tee /tmp/taco-measure-pins.txt
```

The script printed `dir=/Users/jwu/workspace/syndai/taco-m0-pins-cache`.
A second `shasum -a 256` of the three cached files matched the lines below.

## Host

```text
$ sw_vers
ProductName:		macOS
ProductVersion:		27.0.1
BuildVersion:		26A434

$ uname -m
arm64
```

This machine is not C1 and it is not a named C2.

## Measured sha256 lines

Copied from the script stdout:

```text
role_file=ggml-small.bin sha256=1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b bytes=487601967
role_file=silero_vad.onnx sha256=1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3 bytes=2327524
role_file=Hy-MT2-1.8B-Q4_K_M.gguf sha256=dc5f44fcf1fa496ee7ad725982c0c8c553a4de00259b53af84c4b89fb0c06699 bytes=1133080448
```

The script also printed:

```text
note=digests above are measured on this host; copy into freeze::PINS only after review
note=do not flip PINS_MEASURED until filenames and digests are recorded together
```

## Acceptance

- [x] Filenames and these digests are recorded together in `freeze::PINS`, and `PINS_MEASURED` is set in that same change.
- [ ] **M0-AC2:** The combined native spike performs ASR and translation with these exact files on Metal.
- [ ] **M1-AC1:** Explicit fetch publishes only complete verified artifacts.
- [ ] **M1-AC3:** One native load of the pinned bundle succeeds with connectivity unavailable.
- [ ] Metal use is verified from instrumentation. These files were hashed, not loaded.
