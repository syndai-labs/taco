//! Frozen manifest, corners, corpora, and the unrun scheduling experiment.
//! Digests come from `docs/evidence/m0/pin-measurement.md` (2026-10-16 Mac host).
//! Measurement is not a Metal load and does not qualify M0-AC2.

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

/// True only together with measured digests and filenames below.
pub const PINS_MEASURED: bool = true;

pub const PINS: [ArtifactPin; 3] = [
    ArtifactPin {
        role: "vad",
        family: "silero",
        sha256: Some("1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3"),
        filename: Some("silero_vad.onnx"),
    },
    ArtifactPin {
        role: "asr",
        family: "whisper-small-multilingual",
        sha256: Some("1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b"),
        filename: Some("ggml-small.bin"),
    },
    ArtifactPin {
        role: "mt",
        family: "hy-mt2-1.8b-gguf",
        sha256: Some("dc5f44fcf1fa496ee7ad725982c0c8c553a4de00259b53af84c4b89fb0c06699"),
        filename: Some("Hy-MT2-1.8B-Q4_K_M.gguf"),
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

/// One subject. `spdx` and `notice` come only from a LICENSE file fetched for
/// that subject. See `docs/evidence/m0/license-inventory.md`. An engine row is
/// not a model row. `notice` is the copyright line, not the permission text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LicenseRecord {
    pub subject: &'static str,
    pub covers: &'static str,
    pub spdx: Option<&'static str>,
    pub notice: Option<&'static str>,
}

pub const LICENSES: [LicenseRecord; 5] = [
    LicenseRecord {
        subject: "silero-vad-weights",
        covers: "model",
        spdx: Some("MIT"),
        notice: Some("Copyright (c) 2020-present Silero Team"),
    },
    LicenseRecord {
        subject: "whisper-small-weights",
        covers: "model",
        spdx: Some("MIT"),
        notice: Some("Copyright (c) 2022 OpenAI"),
    },
    LicenseRecord {
        subject: "hy-mt2-1.8b-gguf",
        covers: "model",
        spdx: Some("Apache-2.0"),
        notice: Some("Copyright (C) 2026 Tencent. All rights reserved."),
    },
    LicenseRecord {
        subject: "whisper.cpp",
        covers: "engine",
        spdx: Some("MIT"),
        notice: Some("Copyright (c) 2023-2026 The ggml authors"),
    },
    LicenseRecord {
        subject: "llama.cpp",
        covers: "engine",
        spdx: Some("MIT"),
        notice: Some("Copyright (c) 2023-2026 The ggml authors"),
    },
];

/// Notarized distribution artifact. `None`: no signed install exists.
/// Filled `LICENSES` rows do not set this. Ad-hoc `codesign -s -` does not set this.
/// 2026-10-17 measurement: no Developer ID Application identity.
/// See `docs/evidence/m0/signed-install.md`.
pub const SIGNED_INSTALL: Option<&str> = None;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corner {
    pub id: &'static str,
    pub machine: &'static str,
    pub os_build: Option<&'static str>,
    pub qualified: bool,
}

pub const CORNERS: [Corner; 2] = [
    Corner {
        id: "C1",
        machine: "Apple Silicon M1 class, 16 GiB unified memory, AC power",
        os_build: Some("macOS 14.2"),
        qualified: false,
    },
    Corner {
        id: "C2",
        machine: "a different Apple Silicon machine",
        os_build: None,
        qualified: false,
    },
];

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
