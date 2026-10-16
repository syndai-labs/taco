//! The product crate forbids unsafe. This crate is the only call into
//! `taco_engines_linked`. No model is opened here.

/// Probe symbol from `native/taco_engines.h`.
///
/// True means this Mach-O was linked with GPU offload support. It does not
/// mean a model loaded, Metal ran a decode, or a worker joined.
#[must_use]
pub fn gpu_offload_linked() -> bool {
    linked() == 1
}

#[cfg(taco_engines_static)]
fn linked() -> i32 {
    // SAFETY: the symbol is a process-lifetime C function in the static
    // archive this build linked. It takes no pointer and does not borrow
    // Rust memory.
    unsafe { taco_engines_linked() }
}

#[cfg(taco_engines_static)]
extern "C" {
    fn taco_engines_linked() -> i32;
}

#[cfg(not(taco_engines_static))]
fn linked() -> i32 {
    0
}
