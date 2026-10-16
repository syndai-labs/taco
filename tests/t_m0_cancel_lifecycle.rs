//! T-M0-CANCEL-LIFECYCLE
//!
//! Native tier. A wrapper timeout with a live thread is not a pass.

#[test]
fn t_m0_cancel_lifecycle() {
    assert!(
        cfg!(all(target_os = "macos", target_arch = "aarch64")),
        "T-M0-CANCEL-LIFECYCLE blocked: host is not macOS Apple Silicon. \
         See risks-and-decisions.md R5."
    );
    panic!(
        "T-M0-CANCEL-LIFECYCLE not executed: no Metal worker was cancelled or joined \
         inside the shutdown reserve"
    );
}
