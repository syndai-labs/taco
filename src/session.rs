//! Session state machine. Capture is not started from this milestone.

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

/// Validate one invocation. On every path `capture_started` stays false.
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

/// There is no capture backend in this milestone.
pub fn start_capture(session: &mut Session) -> Result<(), ErrorCode> {
    session.capture_started = false;
    Err(ErrorCode::CaptureNotAvailable)
}

/// Open VAD, ASR, and MT in this process. Does not start capture.
pub fn open_inference() -> Result<(), ErrorCode> {
    for role in crate::engines::ROLES {
        crate::engines::initialize(role)?;
    }
    Ok(())
}

pub fn platform_supported() -> bool {
    cfg!(all(target_os = "macos", target_arch = "aarch64"))
}
