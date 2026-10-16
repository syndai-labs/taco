//! Immutable commits and the one terminal translation outcome.

use std::collections::{BTreeMap, VecDeque};

use crate::error::ErrorCode;
use crate::language::Language;
use crate::limits::Limits;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SegmentId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointReason {
    Silence,
    MaxDuration,
    StopFlush,
}

impl EndpointReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Silence => "silence",
            Self::MaxDuration => "max_duration",
            Self::StopFlush => "stop_flush",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "silence" => Some(Self::Silence),
            "max_duration" => Some(Self::MaxDuration),
            "stop_flush" => Some(Self::StopFlush),
            _ => None,
        }
    }
}

pub const ENDPOINT_REASONS: [EndpointReason; 3] = [
    EndpointReason::Silence,
    EndpointReason::MaxDuration,
    EndpointReason::StopFlush,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureDetail {
    NativeError,
    TooLong,
    TokenExhausted,
    EmptyOutput,
    BadOutput,
}

impl FailureDetail {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NativeError => "native_error",
            Self::TooLong => "too_long",
            Self::TokenExhausted => "token_exhausted",
            Self::EmptyOutput => "empty_output",
            Self::BadOutput => "bad_output",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "native_error" => Some(Self::NativeError),
            "too_long" => Some(Self::TooLong),
            "token_exhausted" => Some(Self::TokenExhausted),
            "empty_output" => Some(Self::EmptyOutput),
            "bad_output" => Some(Self::BadOutput),
            _ => None,
        }
    }
}

pub const FAILURE_DETAILS: [FailureDetail; 5] = [
    FailureDetail::NativeError,
    FailureDetail::TooLong,
    FailureDetail::TokenExhausted,
    FailureDetail::EmptyOutput,
    FailureDetail::BadOutput,
];

/// Top-level translation outcomes. Detail codes live inside `Failed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeKind {
    Complete,
    Failed(FailureDetail),
    SkippedOverload,
    TimedOut,
    Cancelled,
}

pub const TOP_LEVEL: [&str; 5] = [
    "complete",
    "failed",
    "skipped_overload",
    "timed_out",
    "cancelled",
];

impl OutcomeKind {
    pub fn top_level(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Failed(_) => "failed",
            Self::SkippedOverload => "skipped_overload",
            Self::TimedOut => "timed_out",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn token(self) -> String {
        match self {
            Self::Failed(detail) => format!("failed:{}", detail.as_str()),
            other => other.top_level().to_string(),
        }
    }

    pub fn parse_token(raw: &str) -> Option<Self> {
        if let Some(detail) = raw.strip_prefix("failed:") {
            return FailureDetail::parse(detail).map(Self::Failed);
        }
        match raw {
            "complete" => Some(Self::Complete),
            "skipped_overload" => Some(Self::SkippedOverload),
            "timed_out" => Some(Self::TimedOut),
            "cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommittedSegment {
    pub id: SegmentId,
    pub epoch: u64,
    pub start: u64,
    pub end: u64,
    pub source: Language,
    pub text: String,
    pub reason: EndpointReason,
    pub commit_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Draft {
    pub id: SegmentId,
    pub epoch: u64,
    pub revision: u32,
    pub start: u64,
    pub end: u64,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionEvent {
    Draft(Draft),
    Committed(CommittedSegment),
    Outcome {
        id: SegmentId,
        kind: OutcomeKind,
        target: String,
    },
    Discontinuity {
        epoch: u64,
        unknown_duration: bool,
    },
    Status {
        code: &'static str,
    },
}

/// Translation sees committed segments only.
pub fn translation_input(event: &SessionEvent) -> Result<&CommittedSegment, ErrorCode> {
    match event {
        SessionEvent::Committed(segment) => Ok(segment),
        _ => Err(ErrorCode::NotACommit),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsrFinal {
    EmptyDecode,
    Text,
}

pub fn classify_asr_final(text: &str) -> AsrFinal {
    if text.trim().is_empty() {
        AsrFinal::EmptyDecode
    } else {
        AsrFinal::Text
    }
}

#[derive(Debug, Default)]
pub struct Ledger {
    text: BTreeMap<u64, String>,
    outcomes: BTreeMap<u64, OutcomeKind>,
    owned: Vec<(u64, u64, u64)>,
}

impl Ledger {
    pub fn commit_text(&mut self, id: SegmentId, text: &str) -> Result<(), ErrorCode> {
        if self.text.contains_key(&id.0) {
            return Err(ErrorCode::InvariantBreach);
        }
        self.text.insert(id.0, text.to_string());
        Ok(())
    }

    pub fn text(&self, id: SegmentId) -> Option<&str> {
        self.text.get(&id.0).map(String::as_str)
    }

    /// First terminal outcome wins. A second outcome is refused and the first stays.
    pub fn record_outcome(&mut self, id: SegmentId, kind: OutcomeKind) -> Result<(), ErrorCode> {
        if self.outcomes.contains_key(&id.0) {
            return Err(ErrorCode::DuplicateOutcome);
        }
        self.outcomes.insert(id.0, kind);
        Ok(())
    }

    pub fn outcome(&self, id: SegmentId) -> Option<OutcomeKind> {
        self.outcomes.get(&id.0).copied()
    }

    pub fn commit_interval(&mut self, epoch: u64, start: u64, end: u64) -> Result<(), ErrorCode> {
        if end < start {
            return Err(ErrorCode::InvariantBreach);
        }
        for &(other_epoch, other_start, other_end) in &self.owned {
            if other_epoch == epoch && start < other_end && other_start < end {
                return Err(ErrorCode::InvariantBreach);
            }
        }
        self.owned.push((epoch, start, end));
        Ok(())
    }
}

pub fn check_text(limits: &Limits, text: &str) -> Result<(), ErrorCode> {
    if text.len() as u64 > limits.segment_text_bytes() {
        Err(ErrorCode::TextTooLong)
    } else {
        Ok(())
    }
}

pub fn budget_failure(
    limits: &Limits,
    source_bytes: u64,
    prompt_tokens: u64,
    completion_tokens: u64,
) -> Option<FailureDetail> {
    if source_bytes > limits.segment_text_bytes() {
        return Some(FailureDetail::TooLong);
    }
    let room = limits
        .token_budget()
        .saturating_sub(limits.completion_reserve());
    if prompt_tokens > room {
        return Some(FailureDetail::TooLong);
    }
    if completion_tokens > limits.completion_reserve() {
        return Some(FailureDetail::TokenExhausted);
    }
    None
}

pub fn check_window_ms(limits: &Limits, owned_ms: u64) -> Result<(), ErrorCode> {
    if owned_ms > limits.model_window_ms() {
        Err(ErrorCode::WindowExceeded)
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admit {
    Active,
    Queued,
    Refused,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmitQueue {
    waiting_cap: usize,
    waiting: usize,
    active: bool,
}

impl AdmitQueue {
    pub fn new(waiting_cap: usize) -> Self {
        Self {
            waiting_cap,
            waiting: 0,
            active: false,
        }
    }

    pub fn admit(&mut self) -> Admit {
        if !self.active {
            self.active = true;
            return Admit::Active;
        }
        if self.waiting < self.waiting_cap {
            self.waiting += 1;
            return Admit::Queued;
        }
        Admit::Refused
    }

    pub fn depth(&self) -> usize {
        self.waiting + usize::from(self.active)
    }
}

#[derive(Debug)]
pub struct UiRows {
    cap: usize,
    completed: VecDeque<u64>,
    pending: VecDeque<u64>,
}

impl UiRows {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            completed: VecDeque::new(),
            pending: VecDeque::new(),
        }
    }

    pub fn push_completed(&mut self, id: u64) {
        self.completed.push_back(id);
        self.evict();
    }

    pub fn push_pending(&mut self, id: u64) {
        self.pending.push_back(id);
        self.evict();
    }

    fn evict(&mut self) {
        while self.completed.len() + self.pending.len() > self.cap {
            if self.completed.pop_front().is_none() {
                break;
            }
        }
    }

    pub fn completed_len(&self) -> usize {
        self.completed.len()
    }

    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }
}

#[derive(Debug)]
pub struct StatusRing {
    cap: usize,
    events: VecDeque<u16>,
}

impl StatusRing {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            events: VecDeque::new(),
        }
    }

    pub fn push(&mut self, code: u16) {
        if self.cap == 0 {
            return;
        }
        if self.events.len() == self.cap {
            self.events.pop_front();
        }
        self.events.push_back(code);
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleAccount {
    pub live_ring: u64,
    pub gap: u64,
    pub owned: u64,
    pub untranscribed: u64,
    pub context: u64,
}

impl SampleAccount {
    pub fn balances(&self, captured: u64) -> bool {
        self.live_ring
            .saturating_add(self.gap)
            .saturating_add(self.owned)
            .saturating_add(self.untranscribed)
            .saturating_add(self.context)
            == captured
    }
}

/// Redirect policy from the acceptance cap. This does not open a connection.
pub fn allow_redirect(
    limits: &Limits,
    followed: u64,
    host: &str,
    allow: &[&str],
) -> Result<(), ErrorCode> {
    if followed >= limits.fetch_redirects() || !allow.contains(&host) {
        Err(ErrorCode::RedirectRejected)
    } else {
        Ok(())
    }
}

pub fn allow_retry(limits: &Limits, attempts_already: u64) -> Result<(), ErrorCode> {
    if attempts_already >= limits.fetch_attempts() {
        Err(ErrorCode::FetchAttemptsExhausted)
    } else {
        Ok(())
    }
}
