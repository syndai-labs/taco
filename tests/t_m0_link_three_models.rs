//! T-M0-LINK-THREE-MODELS
//!
//! Native tier. A CPU run, a subprocess CLI, or an unpinned digest is not a pass.
//! This file is compiled only with `--features native` and fails closed.

use std::path::PathBuf;

use taco::limits;
use taco::session;

#[test]
fn t_m0_link_three_models() {
    if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        panic!(
            "T-M0-LINK-THREE-MODELS blocked: host is not macOS Apple Silicon. \
             Do not record a CPU or subprocess result. See risks-and-decisions.md R2."
        );
    }
    let dir = model_dir();
    let proof = session::prove_native(&dir, limits().cancel_reserve_ms()).unwrap_or_else(|err| {
        panic!("T-M0-LINK-THREE-MODELS not executed: engines refused ({err:?}) dir={dir:?}")
    });
    if proof.link_three != 1 || proof.gpu_offload != 1 || proof.vad_ort != 1 {
        panic!("T-M0-LINK-THREE-MODELS incomplete: {proof:?}");
    }
    assert!(
        proof.gpu_device.starts_with("MTL"),
        "gpu device {}",
        proof.gpu_device
    );
}

fn model_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("TACO_MODEL_DIR") {
        return PathBuf::from(dir);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../taco-m0-pins-cache")
}
