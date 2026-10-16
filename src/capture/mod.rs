//! Microphone and process-tap spike.
//!
//! Preflight does not call these functions. Process-tap FFI stays in
//! `native/process_tap.m` and is not invoked from Rust.

mod microphone;
mod process_tap;
mod ring;

pub use microphone::open_default_microphone;
pub use process_tap::{open_bundle_tap, open_pid_tap, open_system_tap};
pub use ring::{rings_allocated, CaptureRing};

use std::sync::atomic::{AtomicU64, Ordering};

static BACKEND_CALLS: AtomicU64 = AtomicU64::new(0);

pub fn backend_calls() -> u64 {
    BACKEND_CALLS.load(Ordering::SeqCst)
}

pub(crate) fn note_backend_call() {
    BACKEND_CALLS.fetch_add(1, Ordering::SeqCst);
}

/// Safe Rust does not call `taco_spike_linked`. This stays false and does not
/// open a tap or claim a TCC grant.
pub fn spike_linked() -> bool {
    false
}

/// Keeps the spike functions in the binary without calling them.
pub fn linkage_anchor() -> usize {
    let microphone = open_default_microphone as *const ();
    let system = open_system_tap as *const ();
    let bundle = open_bundle_tap as *const ();
    let pid = open_pid_tap as *const ();
    microphone as usize ^ system as usize ^ bundle as usize ^ pid as usize
}
