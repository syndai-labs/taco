# Signed install attempt (2026-10-17)

Host: Apple Silicon, macOS 27.0.1 (26A434), arm64. `rustc 1.99.0 (b940084d7 2026-09-28)`. Xcode 27.1 (27A9275). `notarytool 1.1.3 (42)`. Tree `607724e`. Frozen by PR #38 tip `e7433c2`. `cargo build --locked` produced `target/release/taco` and `target/debug/taco`. Copies used for signing lived under `/tmp/taco-m0-signed-install-copies`. The binaries were not executed. Nothing was copied to an install path. No M0-AC checkbox is checked. `freeze::SIGNED_INSTALL` stays `None`.

## Identities

```text
$ security find-identity -v -p codesigning
  1) 36DE9F66E808688C3EBE00A33C23EA6197889104 "Apple Development: ops@syndai.com (CT93T5LALG)"
     1 valid identities found
```

`security find-identity -v -p codesigning /Library/Keychains/System.keychain` printed `0 valid identities found`. `security find-certificate -a -c "Developer ID Application"` found no certificate. The login keychain has `Developer ID Certification Authority` certificates. Those are CA certificates, not a `Developer ID Application` signing identity. `security find-generic-password -s com.apple.gke.notary.tool` printed `SecKeychainSearchCopyNext: The specified item could not be found in the keychain.` No `AuthKey_*.p8` was in `~/.private_keys`, `~/.appstoreconnect/private_keys`, or `~/private_keys`.

## Developer ID codesign

```text
$ codesign --force --options runtime --timestamp --sign "Developer ID Application" --identifier ai.syndai.taco /tmp/taco-m0-signed-install-copies/release-devid
Developer ID Application: no identity found
```

Exit 1. `codesign -dv` on that copy afterward was still the linker signature: `Identifier=taco-2dbcae575739b681`, `Signature=adhoc`, `Info.plist=not bound`, `TeamIdentifier=not set`, `flags=0x20002(adhoc,linker-signed)`, `CDHash=8cbdede4d623af994f2b1437bdf17a36111d883c`.

`scripts/sign-install.sh` prints the same identity list and exits 2 with `fail: Developer ID Application: no identity found`. It does not reach `codesign`, `notarytool submit`, or `stapler staple`.

## Notary

```text
$ ditto -c -k --keepParent /tmp/taco-m0-signed-install-copies/release-linker /tmp/taco-m0-signed-install-copies/taco.zip
$ xcrun notarytool submit /tmp/taco-m0-signed-install-copies/taco.zip --keychain-profile taco-notary --no-progress --no-wait
Error: No Keychain password item found for profile: taco-notary

Run 'notarytool store-credentials' to create another credential profile.
```

Exit 69. No submission id was returned.

## What the only signatures on these copies are

Linker-signed release (`shasum -a 256` `eda8caed2cc376605b6bae31453c0db090864bd026968787c78046cc82d18c51`):

```text
Identifier=taco-2dbcae575739b681
CodeDirectory v=20400 size=8056 flags=0x20002(adhoc,linker-signed) hashes=248+0 location=embedded
CDHash=8cbdede4d623af994f2b1437bdf17a36111d883c
Signature=adhoc
Info.plist=not bound
TeamIdentifier=not set
```

Linker-signed debug: `Identifier=taco-a8e556d459e2954c`, `CDHash=99ac345644abcf11aa40eafe0052789e8df31dc5`, `Signature=adhoc`, `Info.plist=not bound`, `TeamIdentifier=not set`.

Ad-hoc re-sign, same command on each copy:

```text
$ codesign --force -s - --identifier ai.syndai.taco
```

Release (`shasum -a 256` `32475c420c0a6c27ba906e26ac6fc0b9d31d495b3f6e04cda8311bb3d2561737`):

```text
Identifier=ai.syndai.taco
CodeDirectory v=20400 size=2151 flags=0x2(adhoc) hashes=62+2 location=embedded
CDHash=434ca964d84bac7307406cd6532044347fa2792b
Signature=adhoc
Info.plist entries=5
TeamIdentifier=not set
# designated => cdhash H"434ca964d84bac7307406cd6532044347fa2792b"
```

Debug (`shasum -a 256` `4a2d4ed1e5d1cacf8a5990110ede5360f8e48454c1c71243ea39c11f261dca61`):

```text
Identifier=ai.syndai.taco
CDHash=82d87753911f3d623f726944bffdfe45c3c3a355
Signature=adhoc
Info.plist entries=5
TeamIdentifier=not set
# designated => cdhash H"82d87753911f3d623f726944bffdfe45c3c3a355"
```

The identifier matches. The designated requirement does not. Each requirement is only that copy's cdhash.

## Gatekeeper

```text
$ spctl --assess -vv --type execute /tmp/taco-m0-signed-install-copies/release-adhoc
/tmp/taco-m0-signed-install-copies/release-adhoc: rejected
$ spctl --assess -vv --type execute /tmp/taco-m0-signed-install-copies/debug-adhoc
/tmp/taco-m0-signed-install-copies/debug-adhoc: rejected
$ spctl --assess -vv --type execute /tmp/taco-m0-signed-install-copies/release-linker
/tmp/taco-m0-signed-install-copies/release-linker: rejected
```

Each exit code was 3. `spctl` printed no source line beyond `rejected`.

## Apple Development re-sign did not finish

```text
$ codesign --force --options runtime --timestamp --sign "Apple Development: ops@syndai.com (CT93T5LALG)" --identifier ai.syndai.taco /tmp/taco-m0-signed-install-copies/release-devcert
/tmp/taco-m0-signed-install-copies/release-devcert: replacing existing signature
```

That process did not return. It was stopped after 150 seconds. A second attempt without `--timestamp` printed the same `replacing existing signature` line and did not return. It was stopped after more than three minutes. `codesign -dv` on the copy afterward:

```text
Identifier=taco-2dbcae575739b681
flags=0x20002(adhoc,linker-signed)
Signature=adhoc
Info.plist=not bound
TeamIdentifier=not set
```

No Apple Development signature was produced. The 2026-10-16 spike re-sign (`CDHash=f5455d873156271820035ef4fe0ac7bbe2e8be07`, Info.plist not bound) is a different binary and is not this install.

## Consequence

There is no signed install a fresh `taco run` can use. Developer ID signing fails before a signature exists. Notarization fails before a submission exists. Gatekeeper rejects the linker-signed Mach-O and both ad-hoc Mach-Os. The ad-hoc designated requirement is a different cdhash for the release binary and the debug binary, so those two builds are not one install identity. Distribution of weights stays blocked. ADR 0005's trust root is a manifest inside a signed executable. That executable does not exist.
