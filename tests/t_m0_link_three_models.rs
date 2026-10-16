//! T-M0-LINK-THREE-MODELS
//!
//! Native tier. A CPU run, a subprocess CLI, or an unpinned digest is not a pass.
//! This file is compiled only with `--features native` and fails closed.

#[test]
fn t_m0_link_three_models() {
    assert!(
        cfg!(all(target_os = "macos", target_arch = "aarch64")),
        "T-M0-LINK-THREE-MODELS blocked: host is not macOS Apple Silicon. \
         Do not record a CPU or subprocess result. See risks-and-decisions.md R2."
    );
    panic!(
        "T-M0-LINK-THREE-MODELS not executed: pinned VAD, Whisper small, and Hy-MT2 \
         digests are TBD and Metal GPU execution was not observed"
    );
}
