//! CoreAudio process tap and a private aggregate live in `native/process_tap.m`.
//! Safe Rust does not cross that boundary, does not create a tap, and does not
//! claim a TCC grant.

const KIND_SYSTEM: i32 = 1;
const KIND_PID: i32 = 2;
const KIND_BUNDLE: i32 = 3;

/// Matches `TACO_REFUSED` in `native/process_tap.m`. Not a successful open.
const NOT_WIRED: i32 = 4;

pub struct ProcessTap {
    _private: (),
}

fn refuse(kind: i32) -> Result<ProcessTap, i32> {
    let _ = kind;
    Err(NOT_WIRED)
}

pub fn open_system_tap() -> Result<ProcessTap, i32> {
    refuse(KIND_SYSTEM)
}

pub fn open_pid_tap(pid: u32) -> Result<ProcessTap, i32> {
    let _ = pid;
    refuse(KIND_PID)
}

pub fn open_bundle_tap(bundle_id: &str) -> Result<ProcessTap, i32> {
    let _ = bundle_id;
    refuse(KIND_BUNDLE)
}
