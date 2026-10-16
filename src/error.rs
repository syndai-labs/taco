/// Closed error codes. Exit class follows ADR 0006: syntax is 2, preflight is 1.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    UnknownFlag,
    MissingArgument,
    InvalidSelectorSyntax,
    MultipleSources,
    BareNameRejected,
    StdinSourceRejected,
    PathSourceRejected,
    IdenticalLanguagePair,
    LanguageTagUnknown,
    UiUnsupported,
    TextTooLong,
    DuplicateOutcome,
    IllegalTransition,
    InvalidFormat,
    WindowExceeded,
    ModelsUnpinned,
    FetchRefusedUnpinned,
    PlatformUnsupported,
    CaptureNotAvailable,
    EngineNotLinked,
    NotACommit,
    InvariantBreach,
    RedirectRejected,
    FetchAttemptsExhausted,
    HeldOutClosed,
}

impl ErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnknownFlag => "unknown_flag",
            Self::MissingArgument => "missing_argument",
            Self::InvalidSelectorSyntax => "invalid_selector_syntax",
            Self::MultipleSources => "multiple_sources",
            Self::BareNameRejected => "bare_name_rejected",
            Self::StdinSourceRejected => "stdin_source_rejected",
            Self::PathSourceRejected => "path_source_rejected",
            Self::IdenticalLanguagePair => "identical_language_pair",
            Self::LanguageTagUnknown => "language_tag_unknown",
            Self::UiUnsupported => "ui_unsupported",
            Self::TextTooLong => "text_too_long",
            Self::DuplicateOutcome => "duplicate_outcome",
            Self::IllegalTransition => "illegal_transition",
            Self::InvalidFormat => "invalid_format",
            Self::WindowExceeded => "window_exceeded",
            Self::ModelsUnpinned => "models_unpinned",
            Self::FetchRefusedUnpinned => "fetch_refused_unpinned",
            Self::PlatformUnsupported => "platform_unsupported",
            Self::CaptureNotAvailable => "capture_not_available",
            Self::EngineNotLinked => "engine_not_linked",
            Self::NotACommit => "not_a_commit",
            Self::InvariantBreach => "invariant_breach",
            Self::RedirectRejected => "redirect_rejected",
            Self::FetchAttemptsExhausted => "fetch_attempts_exhausted",
            Self::HeldOutClosed => "held_out_closed",
        }
    }

    pub fn exit_class(self) -> i32 {
        match self {
            Self::UnknownFlag
            | Self::MissingArgument
            | Self::InvalidSelectorSyntax
            | Self::MultipleSources
            | Self::BareNameRejected
            | Self::StdinSourceRejected
            | Self::PathSourceRejected => 2,
            _ => 1,
        }
    }
}

/// ADR 0006 exit table. First matching situation wins; this enum is that order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopKind {
    Help,
    Usage,
    DeadlineMissed,
    Fatal,
    SigintInBudget,
    GracefulClean,
    GracefulTainted,
}

pub fn exit_code(kind: StopKind) -> i32 {
    match kind {
        StopKind::Help | StopKind::GracefulClean => 0,
        StopKind::Usage => 2,
        StopKind::DeadlineMissed | StopKind::Fatal | StopKind::GracefulTainted => 1,
        StopKind::SigintInBudget => 130,
    }
}
