use crate::error::ErrorCode;

/// Charter source categories. Microphone and external input are different
/// categories that share the input selector spelling until a device UID is known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCategory {
    BuiltInMicrophone,
    ExternalInput,
    SystemOutput,
    SelectedApplication,
}

pub const SOURCE_CATEGORIES: [SourceCategory; 4] = [
    SourceCategory::BuiltInMicrophone,
    SourceCategory::ExternalInput,
    SourceCategory::SystemOutput,
    SourceCategory::SelectedApplication,
];

pub fn category_name(category: SourceCategory) -> &'static str {
    match category {
        SourceCategory::BuiltInMicrophone => "built-in microphone",
        SourceCategory::ExternalInput => "external input",
        SourceCategory::SystemOutput => "system output",
        SourceCategory::SelectedApplication => "selected application",
    }
}

/// How a parsed selector sits on the four categories.
/// Input selectors are not system and not application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassifiedSource {
    /// Built-in versus external is a device fact resolved at session start.
    InputPendingDevice,
    SystemOutput,
    SelectedApplication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceSelector {
    Default,
    Input { device_uid: String },
    System,
    App { bundle_id: String },
    Pid { pid: u32 },
}

impl SourceSelector {
    pub fn classify(&self) -> ClassifiedSource {
        match self {
            Self::Default | Self::Input { .. } => ClassifiedSource::InputPendingDevice,
            Self::System => ClassifiedSource::SystemOutput,
            Self::App { .. } | Self::Pid { .. } => ClassifiedSource::SelectedApplication,
        }
    }
}

/// Failed application selection never becomes system capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionFailurePolicy {
    StopDoNotBroaden,
}

pub fn on_unresolved_app() -> SelectionFailurePolicy {
    SelectionFailurePolicy::StopDoNotBroaden
}

/// M2 external-input reservation. Mac evidence names no device, so both fields
/// stay unset. That open slot blocks M0-AC1. M0 does not qualify a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalInputReservation {
    pub display_name: Option<&'static str>,
    pub device_uid: Option<&'static str>,
    pub qualified_in_m0: bool,
}

pub const EXTERNAL_INPUT: ExternalInputReservation = ExternalInputReservation {
    display_name: None,
    device_uid: None,
    qualified_in_m0: false,
};

const AUDIO_EXT: &[&str] = &[
    "wav", "mp3", "flac", "aiff", "aif", "caf", "m4a", "ogg", "pcm",
];

pub fn parse_selector(raw: &str) -> Result<SourceSelector, ErrorCode> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(ErrorCode::InvalidSelectorSyntax);
    }
    if raw.eq_ignore_ascii_case("stdin") || raw == "-" {
        return Err(ErrorCode::StdinSourceRejected);
    }
    if raw.contains("://") {
        return Err(ErrorCode::InvalidSelectorSyntax);
    }
    if is_path(raw) {
        return Err(ErrorCode::PathSourceRejected);
    }
    if raw == "default" {
        return Ok(SourceSelector::Default);
    }
    if raw == "system" {
        return Ok(SourceSelector::System);
    }
    if let Some(uid) = raw.strip_prefix("input:") {
        return parse_uid(uid).map(|device_uid| SourceSelector::Input { device_uid });
    }
    if let Some(bundle) = raw.strip_prefix("app:") {
        return parse_bundle(bundle).map(|bundle_id| SourceSelector::App { bundle_id });
    }
    if let Some(pid) = raw.strip_prefix("pid:") {
        let pid: u32 = pid.parse().map_err(|_| ErrorCode::InvalidSelectorSyntax)?;
        return Ok(SourceSelector::Pid { pid });
    }
    if raw.contains(':') {
        return Err(ErrorCode::InvalidSelectorSyntax);
    }
    Err(ErrorCode::BareNameRejected)
}

fn parse_uid(uid: &str) -> Result<String, ErrorCode> {
    if uid.is_empty()
        || uid.chars().any(char::is_whitespace)
        || uid.starts_with('/')
        || uid.contains("..")
    {
        return Err(ErrorCode::InvalidSelectorSyntax);
    }
    Ok(uid.to_string())
}

fn parse_bundle(bundle: &str) -> Result<String, ErrorCode> {
    if bundle.is_empty()
        || bundle.contains('/')
        || bundle.contains(':')
        || bundle.chars().any(char::is_whitespace)
        || !bundle.contains('.')
    {
        return Err(ErrorCode::InvalidSelectorSyntax);
    }
    Ok(bundle.to_string())
}

fn is_path(raw: &str) -> bool {
    raw.contains('/') || raw.starts_with('.') || has_audio_ext(raw)
}

fn has_audio_ext(raw: &str) -> bool {
    raw.rsplit_once('.')
        .map(|(_, ext)| AUDIO_EXT.iter().any(|e| ext.eq_ignore_ascii_case(e)))
        .unwrap_or(false)
}
