//! `unsafe` FFI for `native/process_tap.m`. Not part of the `forbid(unsafe_code)` crate.

use std::ffi::{c_char, CString};

const KIND_SYSTEM: i32 = 1;
const KIND_PID: i32 = 2;
const KIND_BUNDLE: i32 = 3;
const REFUSED: i32 = 4;

#[repr(C)]
struct TapResult {
    status: i32,
    tap_id: u32,
    aggregate_id: u32,
    created_tap: i32,
    created_aggregate: i32,
    message: [u8; 160],
}

extern "C" {
    fn taco_spike_linked() -> i32;
    fn taco_process_tap_open(
        kind: i32,
        pid: i32,
        bundle_id: *const c_char,
        out: *mut TapResult,
    ) -> i32;
    fn taco_process_tap_close(
        tap_id: u32,
        created_tap: i32,
        aggregate_id: u32,
        created_aggregate: i32,
    );
}

pub struct ProcessTap {
    tap_id: u32,
    created_tap: i32,
    aggregate_id: u32,
    created_aggregate: i32,
}

impl Drop for ProcessTap {
    fn drop(&mut self) {
        unsafe {
            taco_process_tap_close(
                self.tap_id,
                self.created_tap,
                self.aggregate_id,
                self.created_aggregate,
            );
        }
        self.created_tap = 0;
        self.created_aggregate = 0;
    }
}

/// Link probe. Does not create a tap.
pub fn linked() -> bool {
    unsafe { taco_spike_linked() == 1 }
}

pub fn open_system() -> Result<ProcessTap, i32> {
    open(KIND_SYSTEM, 0, None)
}

pub fn open_pid(pid: u32) -> Result<ProcessTap, i32> {
    let pid = i32::try_from(pid).map_err(|_| REFUSED)?;
    open(KIND_PID, pid, None)
}

pub fn open_bundle(bundle_id: &str) -> Result<ProcessTap, i32> {
    let c = CString::new(bundle_id).map_err(|_| REFUSED)?;
    open(KIND_BUNDLE, 0, Some(c.as_ptr()))
}

fn open(kind: i32, pid: i32, bundle: Option<*const c_char>) -> Result<ProcessTap, i32> {
    let mut out = TapResult {
        status: 0,
        tap_id: 0,
        aggregate_id: 0,
        created_tap: 0,
        created_aggregate: 0,
        message: [0; 160],
    };
    let rc =
        unsafe { taco_process_tap_open(kind, pid, bundle.unwrap_or(std::ptr::null()), &mut out) };
    if rc != 0 || out.status != 0 {
        return Err(if out.status != 0 { out.status } else { rc });
    }
    Ok(ProcessTap {
        tap_id: out.tap_id,
        created_tap: out.created_tap,
        aggregate_id: out.aggregate_id,
        created_aggregate: out.created_aggregate,
    })
}
