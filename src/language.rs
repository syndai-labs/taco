use crate::error::ErrorCode;

/// One row of the closed language table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Language {
    pub tag: &'static str,
    pub asr_id: &'static str,
    /// Full name used when the instruction arm is English.
    pub prompt_name_en: &'static str,
    /// Full name used when the instruction arm is Chinese.
    pub prompt_name_zh: &'static str,
}

pub const EN: Language = Language {
    tag: "en",
    asr_id: "en",
    prompt_name_en: "English",
    prompt_name_zh: "英语",
};

pub const ZH: Language = Language {
    tag: "zh",
    asr_id: "zh",
    prompt_name_en: "Chinese",
    prompt_name_zh: "中文",
};

pub const LANGUAGES: [Language; 2] = [EN, ZH];

/// Qualified directions. Experimental pairs stay empty.
pub const PAIRS: [(&str, &str); 2] = [("en", "zh"), ("zh", "en")];

pub const EXPERIMENTAL_PAIRS: [(&str, &str); 0] = [];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguagePair {
    pub from: Language,
    pub to: Language,
}

pub fn tags() -> [&'static str; 2] {
    [EN.tag, ZH.tag]
}

pub fn experimental_pairs() -> &'static [(&'static str, &'static str)] {
    &EXPERIMENTAL_PAIRS
}

pub fn parse_pair(from: &str, to: &str) -> Result<LanguagePair, ErrorCode> {
    let from_lang = lookup(from)?;
    let to_lang = lookup(to)?;
    if from_lang.tag == to_lang.tag {
        return Err(ErrorCode::IdenticalLanguagePair);
    }
    if !PAIRS.contains(&(from_lang.tag, to_lang.tag)) {
        return Err(ErrorCode::LanguageTagUnknown);
    }
    Ok(LanguagePair {
        from: from_lang,
        to: to_lang,
    })
}

fn lookup(tag: &str) -> Result<Language, ErrorCode> {
    LANGUAGES
        .into_iter()
        .find(|lang| lang.tag == tag)
        .ok_or(ErrorCode::LanguageTagUnknown)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiMode {
    Cli,
    Tui,
}

pub fn parse_ui(raw: &str) -> Result<UiMode, ErrorCode> {
    match raw {
        "cli" => Ok(UiMode::Cli),
        "tui" => Ok(UiMode::Tui),
        _ => Err(ErrorCode::UiUnsupported),
    }
}
