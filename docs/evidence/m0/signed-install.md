# Signed install attempt

Cited from `cursor/m0-signed-install-b19b` tip `e7433c2` (PR #38). This branch does not replay `scripts/sign-install.sh`. No acceptance box is checked. The note does not name C2.

Stop. Measured on macOS 27.0.1 (26A434), tree `607724e`. Copies lived under `/tmp`. The binaries were not executed. Nothing was installed. `freeze::SIGNED_INSTALL` stays `None` on that tip.

`security find-identity -v -p codesigning` listed one identity: `Apple Development: ops@syndai.com (CT93T5LALG)`. No `Developer ID Application` certificate. `codesign --sign "Developer ID Application"` printed `Developer ID Application: no identity found` and exited 1. `xcrun notarytool submit --keychain-profile taco-notary` printed `No Keychain password item found for profile: taco-notary` and exited 69. Ad-hoc `codesign -s - --identifier ai.syndai.taco` bound `Info.plist entries=5` with `TeamIdentifier=not set`. `spctl --assess -vv --type execute` printed `rejected` and exited 3 for the linker-signed release binary and both ad-hoc copies. An Apple Development re-sign did not return and was stopped. That copy stayed linker-signed (`Info.plist=not bound`).

PR #42 tip `8794303` guards this Stop. `SIGNED_INSTALL` stays `None`. A `Some` claim matches only when this record's failure phrases are replaced by `Authority=Developer ID Application:`, `source=Notarized Developer ID`, and `status: Accepted`, and the id is not a failed CDHash from the Stop. This branch does not replay `signed_install_matches`. No new Mac measurement.

- [ ] **M0-AC4:** There is no Developer ID signed install. Weight distribution stays blocked.
