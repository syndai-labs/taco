//! Shared taco contracts.
//!
//! Milestone 0 freezes selectors, events, outcomes, errors, and stage traits.
//! Later milestones fill those types in. They do not invent a second session path.
//! Preflight does not start capture or load models, and this build claims no qualification.
//! `capture` is a macOS spike. It is not on the session path.

#![forbid(unsafe_code)]

pub mod capture;
pub mod cli_record;
pub mod error;
pub mod freeze;
pub mod language;
pub mod ledger;
pub mod limits;
pub mod macho;
pub mod selector;
pub mod session;
pub mod stage;

pub use error::ErrorCode;
pub use limits::{limits, Limits};
pub use session::{preflight, Session, SessionState};

/// Embedded Info.plist for the single-file tool.
/// `include_bytes!` alone does not bind the plist for codesign; `build.rs`
/// must `-sectcreate __TEXT __info_plist`, and the plist must carry
/// `CFBundleIdentifier` and `CFBundleName` or codesign still reports unbound.
/// Presence here is not evidence that TCC attributed the prompts.
pub fn usage_plist() -> &'static [u8] {
    include_bytes!("../assets/Info.plist")
}

pub fn usage_keys_present() -> bool {
    let plist = usage_plist();
    contains(plist, b"CFBundleIdentifier")
        && contains(plist, b"CFBundleName")
        && contains(plist, b"ai.syndai.taco")
        && contains(plist, b"NSMicrophoneUsageDescription")
        && contains(plist, b"NSAudioCaptureUsageDescription")
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}
