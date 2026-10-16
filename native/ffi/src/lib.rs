//! The product crate forbids unsafe. This crate is the only call into the
//! static engine archive. Model loads happen only when the product asks.

use std::ffi::CString;
use std::os::raw::c_char;
use std::path::Path;

/// Probe symbol from `native/taco_engines.h`.
///
/// True means this Mach-O was linked with GPU offload support. It does not
/// mean a model loaded, Metal ran a decode, or a worker joined.
#[must_use]
pub fn gpu_offload_linked() -> bool {
    linked() == 1
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proof {
    pub linked: i32,
    pub vad_whisper: i32,
    pub vad_ort: i32,
    pub asr_ok: i32,
    pub mt_ok: i32,
    pub gpu_offload: i32,
    pub off_n: i32,
    pub off_d: i32,
    pub asr_text_bytes: i32,
    pub mt_text_bytes: i32,
    pub whisper_init_join_ms: i32,
    pub whisper_decode_join_ms: i32,
    pub mt_init_join_ms: i32,
    pub mt_decode_join_ms: i32,
    pub whisper_init_within: i32,
    pub whisper_decode_within: i32,
    pub mt_init_within: i32,
    pub mt_decode_within: i32,
    pub link_three: i32,
    pub cancel_lifecycle: i32,
    pub teardown: i32,
    pub gpu_device: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VmSample {
    pub phys_footprint: u64,
    pub resident_size: u64,
    pub device: u64,
    pub ledger_phys_footprint_peak: i64,
    pub graphics_footprint: i64,
    pub graphics_nofootprint: i64,
    pub malloc_size_in_use: u64,
    pub malloc_max_size_in_use: u64,
    pub ok: i32,
    pub graphics_ok: i32,
}

#[must_use]
pub fn init_role(role: i32, path: &Path) -> i32 {
    let Some(path) = c_path(path) else {
        return 2;
    };
    init_raw(role, path.as_ptr())
}

#[must_use]
pub fn infer_role(role: i32, input: &[u8]) -> i32 {
    let mut out_n = 0;
    infer_raw(role, input.as_ptr(), input.len(), &mut out_n)
}

#[must_use]
pub fn cancel_role(role: i32, reserve_ms: i32) -> (i32, i32) {
    let mut join_ms = -1;
    let rc = cancel_raw(role, reserve_ms, &mut join_ms);
    (rc, join_ms)
}

pub fn close_role(role: i32) {
    close_raw(role);
}

/// Addresses of the product entry points.
///
/// The release link runs `-dead_strip`. Taking these addresses from the taco
/// binary keeps infer, cancel, close, and prove in the Mach-O even when a
/// given invocation does not call them.
#[must_use]
pub fn entrypoints() -> usize {
    let slots = [
        init_role as *const (),
        infer_role as *const (),
        cancel_role as *const (),
        close_role as *const (),
        prove as *const (),
    ];
    slots.iter().fold(0usize, |acc, slot| acc ^ (*slot as usize))
}

#[must_use]
pub fn vm_sample() -> VmSample {
    sample_raw()
}

pub fn prove(vad: &Path, asr: &Path, mt: &Path, reserve_ms: i32) -> Result<Proof, i32> {
    let (Some(vad), Some(asr), Some(mt)) = (c_path(vad), c_path(asr), c_path(mt)) else {
        return Err(2);
    };
    prove_raw(vad.as_ptr(), asr.as_ptr(), mt.as_ptr(), reserve_ms)
}

fn c_path(path: &Path) -> Option<CString> {
    CString::new(path.to_string_lossy().as_bytes()).ok()
}

#[cfg(taco_engines_static)]
fn linked() -> i32 {
    // SAFETY: the symbol is a process-lifetime C function in the static
    // archive this build linked. It takes no pointer and does not borrow
    // Rust memory.
    unsafe { taco_engines_linked() }
}

#[cfg(taco_engines_static)]
fn init_raw(role: i32, path: *const c_char) -> i32 {
    // SAFETY: `path` is a NUL-terminated CString that outlives the call.
    // The engine copies the path bytes it needs before returning.
    unsafe { taco_engines_init(role, path) }
}

#[cfg(taco_engines_static)]
fn infer_raw(role: i32, data: *const u8, len: usize, out_n: *mut i32) -> i32 {
    // SAFETY: `data` points at `len` initialized bytes owned by the caller
    // for the duration of the call. `out_n` is a live i32.
    unsafe { taco_engines_infer(role, data, len, out_n) }
}

#[cfg(taco_engines_static)]
fn cancel_raw(role: i32, reserve_ms: i32, join_ms: *mut i32) -> i32 {
    // SAFETY: `join_ms` is a live i32. The call joins its own worker before
    // it returns.
    unsafe { taco_engines_cancel(role, reserve_ms, join_ms) }
}

#[cfg(taco_engines_static)]
fn close_raw(role: i32) {
    // SAFETY: close releases only the context this process opened for `role`.
    unsafe { taco_engines_close(role) }
}

#[cfg(taco_engines_static)]
fn prove_raw(
    vad: *const c_char,
    asr: *const c_char,
    mt: *const c_char,
    reserve_ms: i32,
) -> Result<Proof, i32> {
    let mut raw = RawProof::zeroed();
    // SAFETY: the three paths are NUL-terminated CStrings that outlive the
    // call. `raw` is a live POD the C function writes before returning.
    let rc = unsafe { taco_engines_prove(vad, asr, mt, reserve_ms, &mut raw) };
    if rc == 2 && raw.teardown == 0 && raw.link_three == 0 && raw.linked == 0 {
        return Err(rc);
    }
    Ok(raw.into_proof())
}

#[cfg(taco_engines_static)]
extern "C" {
    fn taco_engines_linked() -> i32;
    fn taco_engines_init(role: i32, path: *const c_char) -> i32;
    fn taco_engines_infer(role: i32, data: *const u8, len: usize, out_n: *mut i32) -> i32;
    fn taco_engines_cancel(role: i32, reserve_ms: i32, join_ms: *mut i32) -> i32;
    fn taco_engines_close(role: i32);
    fn taco_engines_prove(
        vad: *const c_char,
        asr: *const c_char,
        mt: *const c_char,
        reserve_ms: i32,
        out: *mut RawProof,
    ) -> i32;
    fn taco_engines_vm_sample(out: *mut RawVm);
}

#[cfg(taco_engines_static)]
#[repr(C)]
struct RawVm {
    phys_footprint: u64,
    resident_size: u64,
    device: u64,
    ledger_phys_footprint_peak: i64,
    graphics_footprint: i64,
    graphics_nofootprint: i64,
    malloc_size_in_use: u64,
    malloc_max_size_in_use: u64,
    ok: i32,
    graphics_ok: i32,
}

#[cfg(taco_engines_static)]
fn sample_raw() -> VmSample {
    let mut raw = RawVm {
        phys_footprint: 0,
        resident_size: 0,
        device: 0,
        ledger_phys_footprint_peak: 0,
        graphics_footprint: 0,
        graphics_nofootprint: 0,
        malloc_size_in_use: 0,
        malloc_max_size_in_use: 0,
        ok: 0,
        graphics_ok: 0,
    };
    // SAFETY: `raw` is a live POD. The C function writes it before returning
    // and does not retain the pointer.
    unsafe { taco_engines_vm_sample(&mut raw) };
    VmSample {
        phys_footprint: raw.phys_footprint,
        resident_size: raw.resident_size,
        device: raw.device,
        ledger_phys_footprint_peak: raw.ledger_phys_footprint_peak,
        graphics_footprint: raw.graphics_footprint,
        graphics_nofootprint: raw.graphics_nofootprint,
        malloc_size_in_use: raw.malloc_size_in_use,
        malloc_max_size_in_use: raw.malloc_max_size_in_use,
        ok: raw.ok,
        graphics_ok: raw.graphics_ok,
    }
}

#[cfg(taco_engines_static)]
#[repr(C)]
struct RawProof {
    linked: i32,
    vad_whisper: i32,
    vad_ort: i32,
    asr_ok: i32,
    mt_ok: i32,
    gpu_offload: i32,
    off_n: i32,
    off_d: i32,
    asr_text_bytes: i32,
    mt_text_bytes: i32,
    whisper_init_join_ms: i32,
    whisper_decode_join_ms: i32,
    mt_init_join_ms: i32,
    mt_decode_join_ms: i32,
    whisper_init_within: i32,
    whisper_decode_within: i32,
    mt_init_within: i32,
    mt_decode_within: i32,
    link_three: i32,
    cancel_lifecycle: i32,
    teardown: i32,
    gpu_device: [u8; 64],
}

#[cfg(taco_engines_static)]
impl RawProof {
    fn zeroed() -> Self {
        Self {
            linked: 0,
            vad_whisper: 0,
            vad_ort: 0,
            asr_ok: 0,
            mt_ok: 0,
            gpu_offload: 0,
            off_n: 0,
            off_d: 0,
            asr_text_bytes: 0,
            mt_text_bytes: 0,
            whisper_init_join_ms: 0,
            whisper_decode_join_ms: 0,
            mt_init_join_ms: 0,
            mt_decode_join_ms: 0,
            whisper_init_within: 0,
            whisper_decode_within: 0,
            mt_init_within: 0,
            mt_decode_within: 0,
            link_three: 0,
            cancel_lifecycle: 0,
            teardown: 0,
            gpu_device: [0; 64],
        }
    }

    fn into_proof(self) -> Proof {
        let end = self
            .gpu_device
            .iter()
            .position(|b| *b == 0)
            .unwrap_or(self.gpu_device.len());
        Proof {
            linked: self.linked,
            vad_whisper: self.vad_whisper,
            vad_ort: self.vad_ort,
            asr_ok: self.asr_ok,
            mt_ok: self.mt_ok,
            gpu_offload: self.gpu_offload,
            off_n: self.off_n,
            off_d: self.off_d,
            asr_text_bytes: self.asr_text_bytes,
            mt_text_bytes: self.mt_text_bytes,
            whisper_init_join_ms: self.whisper_init_join_ms,
            whisper_decode_join_ms: self.whisper_decode_join_ms,
            mt_init_join_ms: self.mt_init_join_ms,
            mt_decode_join_ms: self.mt_decode_join_ms,
            whisper_init_within: self.whisper_init_within,
            whisper_decode_within: self.whisper_decode_within,
            mt_init_within: self.mt_init_within,
            mt_decode_within: self.mt_decode_within,
            link_three: self.link_three,
            cancel_lifecycle: self.cancel_lifecycle,
            teardown: self.teardown,
            gpu_device: String::from_utf8_lossy(&self.gpu_device[..end]).into_owned(),
        }
    }
}

#[cfg(not(taco_engines_static))]
fn linked() -> i32 {
    0
}

#[cfg(not(taco_engines_static))]
fn init_raw(_role: i32, _path: *const c_char) -> i32 {
    -1
}

#[cfg(not(taco_engines_static))]
fn infer_raw(_role: i32, _data: *const u8, _len: usize, _out_n: *mut i32) -> i32 {
    -1
}

#[cfg(not(taco_engines_static))]
fn cancel_raw(_role: i32, _reserve_ms: i32, join_ms: *mut i32) -> i32 {
    if !join_ms.is_null() {
        // SAFETY: the caller passed a live i32 or null. Null was checked.
        unsafe { *join_ms = -1 };
    }
    -1
}

#[cfg(not(taco_engines_static))]
fn close_raw(_role: i32) {}

#[cfg(not(taco_engines_static))]
fn prove_raw(
    _vad: *const c_char,
    _asr: *const c_char,
    _mt: *const c_char,
    _reserve_ms: i32,
) -> Result<Proof, i32> {
    Err(-1)
}

#[cfg(not(taco_engines_static))]
fn sample_raw() -> VmSample {
    VmSample {
        phys_footprint: 0,
        resident_size: 0,
        device: 0,
        ledger_phys_footprint_peak: 0,
        graphics_footprint: 0,
        graphics_nofootprint: 0,
        malloc_size_in_use: 0,
        malloc_max_size_in_use: 0,
        ok: 0,
        graphics_ok: 0,
    }
}
