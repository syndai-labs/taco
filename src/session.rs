//! Session state machine and preflight. Capture opens only when the M0 spike
//! env gate is set so default `cargo test` never touches CoreAudio/CPAL devices.

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::capture::{self, CaptureRing};
use crate::error::ErrorCode;
use crate::freeze::release_ready;
use crate::language::{self, LanguagePair};
use crate::selector::{self, SourceSelector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Preflight,
    Starting,
    Running,
    Degraded,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Start,
    Ready,
    Fail,
    Stop,
    Degrade,
    Recover,
    Joined,
    Deadline,
}

#[derive(Debug)]
pub struct Session {
    pub state: SessionState,
    pub tainted: bool,
    pub capture_started: bool,
}

impl Session {
    pub fn new() -> Self {
        Self {
            state: SessionState::Preflight,
            tainted: false,
            capture_started: false,
        }
    }

    pub fn apply(&mut self, edge: Edge) -> Result<(), ErrorCode> {
        let next = match (self.state, edge) {
            (SessionState::Preflight, Edge::Start) => SessionState::Starting,
            (SessionState::Preflight, Edge::Fail) => SessionState::Failed,
            (SessionState::Preflight, Edge::Stop) => SessionState::Stopping,
            (SessionState::Starting, Edge::Ready) => SessionState::Running,
            (SessionState::Starting, Edge::Fail) => SessionState::Failed,
            (SessionState::Starting, Edge::Stop) => SessionState::Stopping,
            (SessionState::Running, Edge::Degrade) => {
                self.tainted = true;
                SessionState::Degraded
            }
            (SessionState::Running, Edge::Stop) => SessionState::Stopping,
            (SessionState::Degraded, Edge::Recover) => SessionState::Running,
            (SessionState::Degraded, Edge::Stop) => SessionState::Stopping,
            (SessionState::Stopping, Edge::Joined) => SessionState::Stopped,
            (SessionState::Stopping, Edge::Deadline) => SessionState::Failed,
            (SessionState::Stopping, Edge::Fail) => SessionState::Failed,
            _ => return Err(ErrorCode::IllegalTransition),
        };
        self.state = next;
        Ok(())
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RunRequest<'a> {
    pub sources: &'a [String],
    pub from: &'a str,
    pub to: &'a str,
    pub ui: &'a str,
}

#[derive(Debug)]
pub struct PreflightOk {
    pub selector: SourceSelector,
    pub pair: LanguagePair,
}

/// Validate one invocation. On the error path `capture_started` stays false.
pub fn preflight(session: &mut Session, req: &RunRequest<'_>) -> Result<PreflightOk, ErrorCode> {
    let result = (|| {
        if req.sources.is_empty() {
            return Err(ErrorCode::MissingArgument);
        }
        if req.sources.len() != 1 {
            return Err(ErrorCode::MultipleSources);
        }
        let selector = selector::parse_selector(&req.sources[0])?;
        let pair = language::parse_pair(req.from, req.to)?;
        language::parse_ui(req.ui)?;
        if !release_ready() {
            return Err(ErrorCode::ModelsUnpinned);
        }
        if !platform_supported() {
            return Err(ErrorCode::PlatformUnsupported);
        }
        Ok(PreflightOk { selector, pair })
    })();

    if result.is_err() {
        let _ = session.apply(Edge::Fail);
    }
    session.capture_started = false;
    result
}

/// Open the selected capture backend for the M0 distribution spike.
///
/// Real device and tap opens require `TACO_CAPTURE_SPIKE=1` so default
/// `cargo test` never prompts TCC. Without that env, this returns
/// `CaptureNotAvailable` and does not call the microphone or the process tap.
/// `system`, `app:`, and `pid:` use the wired CoreAudio tap on that same gate.
/// The microphone open stops waiting after `TACO_CAPTURE_SPIKE_OPEN_MS`
/// (u64 millis, default `180_000`) and returns `CaptureNotAvailable`.
/// After a microphone open returns `Ok`, the stream is held for
/// `TACO_CAPTURE_SPIKE_HOLD_MS` (u64 millis, default `1_800_000`).
pub fn start_capture(session: &mut Session, preflight: &PreflightOk) -> Result<(), ErrorCode> {
    if std::env::var_os("TACO_CAPTURE_SPIKE").is_none() {
        session.capture_started = false;
        return Err(ErrorCode::CaptureNotAvailable);
    }
    if !platform_supported() {
        session.capture_started = false;
        return Err(ErrorCode::PlatformUnsupported);
    }

    session.apply(Edge::Start)?;
    let outcome = match &preflight.selector {
        SourceSelector::Default | SourceSelector::Input { .. } => open_microphone_spike(),
        SourceSelector::System => map_tap(capture::open_system_tap()),
        SourceSelector::App { bundle_id } => map_tap(capture::open_bundle_tap(bundle_id)),
        SourceSelector::Pid { pid } => map_tap(capture::open_pid_tap(*pid)),
    };

    match outcome {
        Ok(()) => {
            session.apply(Edge::Ready)?;
            let started = true;
            session.capture_started = started;
            session.apply(Edge::Stop)?;
            session.apply(Edge::Joined)?;
            Ok(())
        }
        Err(code) => {
            let _ = session.apply(Edge::Fail);
            session.capture_started = false;
            Err(code)
        }
    }
}

fn open_microphone_spike() -> Result<(), ErrorCode> {
    let ring = Arc::new(CaptureRing::allocate(48_000, 1, 4)?);
    let mic = capture::open_default_microphone(Arc::clone(&ring))
        .map_err(|_| ErrorCode::CaptureNotAvailable)?;
    // After Ok only. Does not cover a CPAL wait inside open.
    thread::sleep(Duration::from_millis(spike_hold_ms()));
    drop(mic);
    let _ = ring.lost();
    Ok(())
}

/// Milliseconds to keep the microphone stream after open returns `Ok`.
fn spike_hold_ms() -> u64 {
    std::env::var("TACO_CAPTURE_SPIKE_HOLD_MS")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(1_800_000)
}

fn map_tap<T>(result: Result<T, i32>) -> Result<(), ErrorCode> {
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(ErrorCode::CaptureNotAvailable),
    }
}

pub fn platform_supported() -> bool {
    cfg!(all(target_os = "macos", target_arch = "aarch64"))
}

/// True when the operator explicitly enabled the capture spike.
pub fn capture_spike_enabled() -> bool {
    std::env::var_os("TACO_CAPTURE_SPIKE").is_some()
}
