//! CoreAudio process tap. `unsafe` lives in `taco-tap` (`native/tap`) because
//! this crate has `forbid(unsafe_code)`. `native/process_tap.m` is the real
//! CoreAudio tap. It is compiled only for macOS, and only called when
//! `TACO_CAPTURE_SPIKE` is set. On that path the aggregate is started with
//! `AudioDeviceStart`, held, then stopped. Drop destroys it.

/// Matches `TACO_REFUSED` in `native/process_tap.m`. Not a successful open.
const NOT_WIRED: i32 = 4;

pub struct ProcessTap {
    #[cfg(target_os = "macos")]
    inner: taco_tap::ProcessTap,
    #[cfg(not(target_os = "macos"))]
    _closed: (),
}

pub fn open_system_tap() -> Result<ProcessTap, i32> {
    open(Which::System)
}

pub fn open_pid_tap(pid: u32) -> Result<ProcessTap, i32> {
    open(Which::Pid(pid))
}

pub fn open_bundle_tap(bundle_id: &str) -> Result<ProcessTap, i32> {
    open(Which::Bundle(bundle_id))
}

#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
enum Which<'a> {
    System,
    Pid(u32),
    Bundle(&'a str),
}

fn open(which: Which<'_>) -> Result<ProcessTap, i32> {
    if std::env::var_os("TACO_CAPTURE_SPIKE").is_none() {
        return Err(NOT_WIRED);
    }
    #[cfg(target_os = "macos")]
    {
        super::note_backend_call();
        let inner = match which {
            Which::System => taco_tap::open_system(),
            Which::Pid(pid) => taco_tap::open_pid(pid),
            Which::Bundle(bundle_id) => taco_tap::open_bundle(bundle_id),
        }?;
        return Ok(ProcessTap { inner });
    }
    #[cfg(not(target_os = "macos"))]
    {
        match which {
            Which::System | Which::Pid(_) | Which::Bundle(_) => Err(NOT_WIRED),
        }
    }
}
