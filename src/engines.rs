//! In-process VAD, ASR, and MT adapters.
//!
//! `initialize`, `infer`, `cancel`, and `close` are the product path.
//! Pins, host, and the `native` feature are checked, then the call still
//! fails. `taco_engines.h` only exports the probe symbol; that is not a load.
//! ponytail: no worker flag until a Mac build can join one. Unsafe stays in
//! `native/ffi`, outside this `forbid(unsafe_code)` crate.

use crate::error::ErrorCode;
use crate::freeze::release_ready;
use crate::limits::limits;
use crate::session::platform_supported;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineRole {
    Vad,
    Asr,
    Mt,
}

pub const ROLES: [EngineRole; 3] = [EngineRole::Vad, EngineRole::Asr, EngineRole::Mt];

/// Why this revision will not open a context.
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
    // ponytail: do not call gpu_offload_linked here. A true probe flag is not
    // initialize, infer, or cancel, and the call can touch Metal.
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

pub fn initialize(role: EngineRole) -> Result<(), ErrorCode> {
    let _ = role;
    Err(block_reason())
}

/// Reject an oversized buffer before any native call. A fitting buffer still refuses.
pub fn infer(role: EngineRole, input: &[u8]) -> Result<(), ErrorCode> {
    let len = u64::try_from(input.len()).unwrap_or(u64::MAX);
    if len > input_cap(role) {
        return Err(ErrorCode::WindowExceeded);
    }
    Err(block_reason())
}

/// Cooperative cancel with no worker to join. This is not a completed cancel.
pub fn cancel(role: EngineRole) -> Result<(), ErrorCode> {
    let _ = role;
    Err(block_reason())
}

/// No native context was opened, so there is nothing to release.
pub fn close(role: EngineRole) {
    let _ = role;
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
