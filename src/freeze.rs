//! Frozen manifest, corners, corpora, and the unrun scheduling experiment.
//! Digests stay unset until a macOS measurement records them.

use crate::error::ErrorCode;
use crate::language::Language;
use crate::limits::Limits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactPin {
    pub role: &'static str,
    pub family: &'static str,
    pub sha256: Option<&'static str>,
    pub filename: Option<&'static str>,
}

/// Flip only together with measured digests from the native spike.
pub const PINS_MEASURED: bool = false;

pub const PINS: [ArtifactPin; 3] = [
    ArtifactPin {
        role: "vad",
        family: "silero",
        sha256: None,
        filename: None,
    },
    ArtifactPin {
        role: "asr",
        family: "whisper-small-multilingual",
        sha256: None,
        filename: None,
    },
    ArtifactPin {
        role: "mt",
        family: "hy-mt2-1.8b-gguf",
        sha256: None,
        filename: None,
    },
];

pub fn release_ready() -> bool {
    PINS_MEASURED
        && PINS
            .iter()
            .all(|pin| pin.sha256.is_some() && pin.filename.is_some())
}

pub fn fetch_models() -> Result<(), ErrorCode> {
    Err(ErrorCode::FetchRefusedUnpinned)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LicenseRecord {
    pub subject: &'static str,
    pub covers: &'static str,
    pub spdx: Option<&'static str>,
}

pub const LICENSES: [LicenseRecord; 5] = [
    LicenseRecord {
        subject: "silero-vad-weights",
        covers: "model",
        spdx: None,
    },
    LicenseRecord {
        subject: "whisper-small-weights",
        covers: "model",
        spdx: None,
    },
    LicenseRecord {
        subject: "hy-mt2-1.8b-gguf",
        covers: "model",
        spdx: None,
    },
    LicenseRecord {
        subject: "whisper.cpp",
        covers: "engine",
        spdx: None,
    },
    LicenseRecord {
        subject: "llama.cpp",
        covers: "engine",
        spdx: None,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corner {
    pub id: &'static str,
    pub machine: &'static str,
    pub os_build: Option<&'static str>,
    pub qualified: bool,
}

/// Set when C1's chip, memory bytes, and OS build are copied from a measurement.
/// Does not set `Corner::qualified`. C2 may still have `os_build: None`.
pub const C1_NAMED: bool = true;

/// This Project has no second acceptance machine.
/// Stays set until a different Apple Silicon Mac is measured: chip name,
/// memory in bytes, and macOS product version and build. Not a measurement.
pub const C2_BLOCKED: bool = true;

pub const CORNERS: [Corner; 2] = [
    Corner {
        id: "C1",
        machine: "Apple M1 Pro, 34359738368 bytes (32 GiB)",
        os_build: Some("macOS 27.0.1 (26A434)"),
        qualified: false,
    },
    Corner {
        id: "C2",
        machine: "blocked: no second acceptance machine",
        os_build: None,
        qualified: false,
    },
];

/// Only OS build with taco evidence. Not a qualified support promise.
/// Stays partial until an allow and a deny exist on this build, and until any
/// older build is either measured or left outside the floor on purpose.
pub const OS_FLOOR: &str = "macOS 27.0.1 (26A434)";
pub const OS_FLOOR_PARTIAL: bool = true;

/// TCC launch parent when a GUI parent would take responsibility.
/// Named, not exercised. Not a second machine and not a TUI host.
pub const ALTERNATE_TERMINAL_HOST: &str = "launchd user agent";
pub const ALTERNATE_HOST_EXERCISED: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CorpusSlot {
    pub name: &'static str,
    pub natural_utterances: u64,
    pub sealed: bool,
}

pub const CORPORA: [CorpusSlot; 3] = [
    CorpusSlot {
        name: "development",
        natural_utterances: 0,
        sealed: false,
    },
    CorpusSlot {
        name: "held-out",
        natural_utterances: 0,
        sealed: false,
    },
    CorpusSlot {
        name: "critical",
        natural_utterances: 0,
        sealed: false,
    },
];

pub fn corpus_gate_met(limits: &Limits) -> bool {
    let need = limits.corpus_utterances();
    CORPORA
        .iter()
        .all(|slot| slot.sealed && slot.natural_utterances >= need)
}

/// Held-out labels stay closed to every caller, including development scoring.
pub fn read_held_out_labels(_caller: &str) -> Result<(), ErrorCode> {
    Err(ErrorCode::HeldOutClosed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScheduleExperiment {
    pub ran: bool,
    pub quantizations_compared: u64,
    pub max_quantizations: u64,
    pub selected_schedule: Option<&'static str>,
    pub selected_quantization: Option<&'static str>,
    pub selected_instruction: Option<&'static str>,
    pub selected_sampling: Option<&'static str>,
}

pub const EXPERIMENT: ScheduleExperiment = ScheduleExperiment {
    ran: false,
    quantizations_compared: 0,
    max_quantizations: 2,
    selected_schedule: None,
    selected_quantization: None,
    selected_instruction: None,
    selected_sampling: None,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionArm {
    English,
    Chinese,
}

pub const BASELINE_INSTRUCTION: InstructionArm = InstructionArm::English;
pub const COMPARISON_WINNER: Option<InstructionArm> = None;

const ENGLISH_TEMPLATE: &str = "Translate the following text into {target_lang}. Note that you should only output the translated result without any additional explanation:\n\n{source_text}";
const CHINESE_TEMPLATE: &str =
    "将以下文本翻译为{target_lang}，注意只需要输出翻译后的结果，不要额外解释：\n\n{source_text}";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptRole {
    User,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptTurn {
    pub role: PromptRole,
    pub text: String,
}

pub fn render_prompt(arm: InstructionArm, target: Language, source: &str) -> PromptTurn {
    let template = match arm {
        InstructionArm::English => ENGLISH_TEMPLATE,
        InstructionArm::Chinese => CHINESE_TEMPLATE,
    };
    let name = match arm {
        InstructionArm::English => target.prompt_name_en,
        InstructionArm::Chinese => target.prompt_name_zh,
    };
    let (head, tail) = template
        .split_once("{source_text}")
        .expect("instruction template");
    let head = head.replace("{target_lang}", name);
    PromptTurn {
        role: PromptRole::User,
        text: format!("{head}{source}{tail}"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SamplingArm {
    pub name: &'static str,
    pub temperature_milli: u32,
}

pub const SAMPLING_ARMS: [SamplingArm; 2] = [
    SamplingArm {
        name: "publisher",
        temperature_milli: 700,
    },
    SamplingArm {
        name: "seeded",
        temperature_milli: 0,
    },
];

pub const SELECTED_SAMPLING: Option<&str> = None;

pub const COMMANDS: &[&str] = &[
    "help",
    "version",
    "notices",
    "sources list",
    "models list",
    "models fetch",
    "models verify",
    "run",
];
