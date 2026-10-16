//! In-process VAD, ASR, and MT adapters.
//!
//! `initialize`, `infer`, `cancel`, and `close` call the static engines when a
//! model directory is bound and this Mach-O linked them. Otherwise they refuse.
//! Unsafe stays in `native/ffi`, outside this `forbid(unsafe_code)` crate.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::error::ErrorCode;
use crate::freeze::{self, release_ready};
use crate::limits::limits;
use crate::session::platform_supported;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineRole {
    Vad,
    Asr,
    Mt,
}

pub const ROLES: [EngineRole; 3] = [EngineRole::Vad, EngineRole::Asr, EngineRole::Mt];

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

/// One `task_vm_info` read. `phys_footprint` is the acceptance metric.
/// `resident_size` and malloc sizes are diagnostic. `graphics_footprint` is
/// the kernel graphics ledger, not `MTLDevice.currentAllocatedSize`.
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

static MODEL_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);

/// Bind the directory that holds the three pinned filenames.
pub fn bind_model_dir(dir: &Path) {
    *MODEL_DIR.lock().expect("model dir") = Some(dir.to_path_buf());
}

fn bound_dir() -> Option<PathBuf> {
    MODEL_DIR.lock().ok().and_then(|guard| guard.clone())
}

fn pin_name(role: EngineRole) -> Option<&'static str> {
    let want = match role {
        EngineRole::Vad => "vad",
        EngineRole::Asr => "asr",
        EngineRole::Mt => "mt",
    };
    freeze::PINS
        .iter()
        .find(|pin| pin.role == want)
        .and_then(|pin| pin.filename)
}

fn bound_pin(role: EngineRole) -> Option<PathBuf> {
    Some(bound_dir()?.join(pin_name(role)?))
}

#[cfg_attr(not(feature = "native"), allow(dead_code))]
fn role_code(role: EngineRole) -> i32 {
    match role {
        EngineRole::Vad => 0,
        EngineRole::Asr => 1,
        EngineRole::Mt => 2,
    }
}

/// Why this call will not open a context.
pub fn block_reason() -> ErrorCode {
    if !release_ready() {
        return ErrorCode::ModelsUnpinned;
    }
    if !platform_supported() {
        return ErrorCode::PlatformUnsupported;
    }
    if !cfg!(feature = "native") {
        return ErrorCode::EngineNotLinked;
    }
    ErrorCode::EngineNotLinked
}

/// Probe hook. False unless this build linked `taco_engines_linked`.
/// Not a model load and not an acceptance result.
#[must_use]
pub fn gpu_offload_linked() -> bool {
    #[cfg(feature = "native")]
    {
        taco_engines_ffi::gpu_offload_linked()
    }
    #[cfg(not(feature = "native"))]
    {
        false
    }
}

fn linked_here() -> bool {
    cfg!(feature = "native") && platform_supported() && gpu_offload_linked()
}

/// Keep infer, cancel, and prove in the Mach-O. Not a model load.
#[must_use]
pub fn native_entrypoints() -> usize {
    #[cfg(feature = "native")]
    {
        std::hint::black_box(taco_engines_ffi::entrypoints())
    }
    #[cfg(not(feature = "native"))]
    {
        0
    }
}

pub fn initialize(role: EngineRole) -> Result<(), ErrorCode> {
    let Some(path) = bound_pin(role) else {
        return Err(block_reason());
    };
    if !linked_here() {
        return Err(block_reason());
    }
    #[cfg(feature = "native")]
    {
        return if taco_engines_ffi::init_role(role_code(role), &path) == 0 {
            Ok(())
        } else {
            Err(ErrorCode::EngineNotLinked)
        };
    }
    #[cfg(not(feature = "native"))]
    {
        let _ = path;
        Err(block_reason())
    }
}

/// Reject an oversized buffer before any native call.
pub fn infer(role: EngineRole, input: &[u8]) -> Result<(), ErrorCode> {
    let len = u64::try_from(input.len()).unwrap_or(u64::MAX);
    if len > input_cap(role) {
        return Err(ErrorCode::WindowExceeded);
    }
    if bound_pin(role).is_none() || !linked_here() {
        return Err(block_reason());
    }
    #[cfg(feature = "native")]
    {
        return if taco_engines_ffi::infer_role(role_code(role), input) == 0 {
            Ok(())
        } else {
            Err(ErrorCode::EngineNotLinked)
        };
    }
    #[cfg(not(feature = "native"))]
    {
        Err(block_reason())
    }
}

/// Join an in-flight native worker. No worker is not a completed cancel.
pub fn cancel(role: EngineRole) -> Result<(), ErrorCode> {
    if bound_pin(role).is_none() || !linked_here() {
        return Err(block_reason());
    }
    let reserve = i32::try_from(limits().cancel_reserve_ms()).unwrap_or(i32::MAX);
    #[cfg(feature = "native")]
    {
        let (rc, _join_ms) = taco_engines_ffi::cancel_role(role_code(role), reserve);
        return if rc == 0 {
            Ok(())
        } else {
            Err(ErrorCode::EngineNotLinked)
        };
    }
    #[cfg(not(feature = "native"))]
    {
        let _ = reserve;
        Err(block_reason())
    }
}

/// Release a context this process opened. A refused engine has nothing to free.
pub fn close(role: EngineRole) {
    if bound_pin(role).is_none() || !linked_here() {
        return;
    }
    #[cfg(feature = "native")]
    {
        taco_engines_ffi::close_role(role_code(role));
    }
}

/// Read `task_vm_info.phys_footprint` for this process. `ok == 0` means unread.
#[must_use]
pub fn vm_sample() -> VmSample {
    #[cfg(feature = "native")]
    {
        let sample = taco_engines_ffi::vm_sample();
        return VmSample {
            phys_footprint: sample.phys_footprint,
            resident_size: sample.resident_size,
            device: sample.device,
            ledger_phys_footprint_peak: sample.ledger_phys_footprint_peak,
            graphics_footprint: sample.graphics_footprint,
            graphics_nofootprint: sample.graphics_nofootprint,
            malloc_size_in_use: sample.malloc_size_in_use,
            malloc_max_size_in_use: sample.malloc_max_size_in_use,
            ok: sample.ok,
            graphics_ok: sample.graphics_ok,
        };
    }
    #[cfg(not(feature = "native"))]
    {
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
}

/// Run load, infer, and cancel for the three pins. Refusal is `EngineNotLinked`.
pub fn prove_pins(dir: &Path, reserve_ms: u64) -> Result<Proof, ErrorCode> {
    if !linked_here() {
        return Err(ErrorCode::EngineNotLinked);
    }
    let vad = dir.join(pin_name(EngineRole::Vad).ok_or(ErrorCode::EngineNotLinked)?);
    let asr = dir.join(pin_name(EngineRole::Asr).ok_or(ErrorCode::EngineNotLinked)?);
    let mt = dir.join(pin_name(EngineRole::Mt).ok_or(ErrorCode::EngineNotLinked)?);
    #[cfg(feature = "native")]
    {
        let reserve = i32::try_from(reserve_ms).unwrap_or(i32::MAX);
        return taco_engines_ffi::prove(&vad, &asr, &mt, reserve)
            .map(proof_from_ffi)
            .map_err(|_| ErrorCode::EngineNotLinked);
    }
    #[cfg(not(feature = "native"))]
    {
        let _ = (vad, asr, mt, reserve_ms);
        Err(ErrorCode::EngineNotLinked)
    }
}

#[cfg(feature = "native")]
fn proof_from_ffi(proof: taco_engines_ffi::Proof) -> Proof {
    Proof {
        linked: proof.linked,
        vad_whisper: proof.vad_whisper,
        vad_ort: proof.vad_ort,
        asr_ok: proof.asr_ok,
        mt_ok: proof.mt_ok,
        gpu_offload: proof.gpu_offload,
        off_n: proof.off_n,
        off_d: proof.off_d,
        asr_text_bytes: proof.asr_text_bytes,
        mt_text_bytes: proof.mt_text_bytes,
        whisper_init_join_ms: proof.whisper_init_join_ms,
        whisper_decode_join_ms: proof.whisper_decode_join_ms,
        mt_init_join_ms: proof.mt_init_join_ms,
        mt_decode_join_ms: proof.mt_decode_join_ms,
        whisper_init_within: proof.whisper_init_within,
        whisper_decode_within: proof.whisper_decode_within,
        mt_init_within: proof.mt_init_within,
        mt_decode_within: proof.mt_decode_within,
        link_three: proof.link_three,
        cancel_lifecycle: proof.cancel_lifecycle,
        teardown: proof.teardown,
        gpu_device: proof.gpu_device,
    }
}

pub fn input_cap(role: EngineRole) -> u64 {
    let limits = limits();
    match role {
        EngineRole::Mt => limits.segment_text_bytes(),
        EngineRole::Vad | EngineRole::Asr => {
            let samples = limits
                .model_window_ms()
                .saturating_mul(limits.normalized_rate_hz())
                / 1000;
            samples.saturating_mul(limits.pcm_bytes_max())
        }
    }
}
