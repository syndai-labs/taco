//! T-M0-CANCEL-LIFECYCLE
//!
//! Native tier. A wrapper timeout with a live thread is not a pass.

use std::path::PathBuf;

use taco::limits;
use taco::session;

#[test]
fn t_m0_cancel_lifecycle() {
    if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        panic!(
            "T-M0-CANCEL-LIFECYCLE blocked: host is not macOS Apple Silicon. \
             See risks-and-decisions.md R5."
        );
    }
    let dir = model_dir();
    let reserve = limits().cancel_reserve_ms();
    let proof = session::prove_native(&dir, reserve).unwrap_or_else(|err| {
        panic!("T-M0-CANCEL-LIFECYCLE not executed: engines refused ({err:?}) dir={dir:?}")
    });
    if proof.cancel_lifecycle != 1 || proof.teardown != 1 {
        panic!("T-M0-CANCEL-LIFECYCLE incomplete: {proof:?}");
    }
    for (name, join) in [
        ("whisper_init", proof.whisper_init_join_ms),
        ("whisper_decode", proof.whisper_decode_join_ms),
        ("mt_init", proof.mt_init_join_ms),
        ("mt_decode", proof.mt_decode_join_ms),
    ] {
        let join = u64::try_from(join).unwrap_or(u64::MAX);
        assert!(join <= reserve, "{name} join {join} ms over reserve {reserve}");
    }
}

fn model_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("TACO_MODEL_DIR") {
        return PathBuf::from(dir);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../taco-m0-pins-cache")
}
