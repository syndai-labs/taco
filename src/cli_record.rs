//! CLI ledger lines. Drafts are not records. Stdout is this format and nothing else.

use crate::ledger::{CommittedSegment, EndpointReason, OutcomeKind, SegmentId};

pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if must_escape(c) => out.push_str(&format!("\\u{{{:x}}}", u32::from(c))),
            c => out.push(c),
        }
    }
    out
}

pub fn escape_bytes(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(text) => escape(text),
        Err(_) => bytes.iter().map(|byte| format!("\\x{byte:02x}")).collect(),
    }
}

fn must_escape(c: char) -> bool {
    c.is_control()
        || matches!(
            c,
            '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}'
        )
}

pub fn field(text: &str) -> String {
    if text.is_empty() {
        return "-".to_string();
    }
    let escaped = escape(text);
    if escaped == "-" {
        return "\\x2d".to_string();
    }
    escaped
}

pub fn unfield(raw: &str) -> String {
    if raw == "-" {
        return String::new();
    }
    unescape(raw)
}

fn unescape(raw: &str) -> String {
    let mut out = String::new();
    let mut chars = raw.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('x') => {
                let hi = chars.next().and_then(|c| c.to_digit(16));
                let lo = chars.next().and_then(|c| c.to_digit(16));
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    if let Some(decoded) = char::from_u32((hi << 4) | lo) {
                        out.push(decoded);
                    }
                }
            }
            Some('u') => {
                if chars.next() != Some('{') {
                    continue;
                }
                let mut hex = String::new();
                for c in chars.by_ref() {
                    if c == '}' {
                        break;
                    }
                    hex.push(c);
                }
                if let Ok(value) = u32::from_str_radix(&hex, 16) {
                    if let Some(decoded) = char::from_u32(value) {
                        out.push(decoded);
                    }
                }
            }
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

pub fn commit_line(segment: &CommittedSegment) -> String {
    format!(
        "commit {} {} {} {} {} {}",
        segment.id.0,
        segment.epoch,
        segment.start,
        segment.end,
        segment.reason.as_str(),
        field(&segment.text)
    )
}

pub fn outcome_line(id: SegmentId, kind: OutcomeKind, target: &str) -> String {
    format!("outcome {} {} {}", id.0, kind.token(), field(target))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitRecord {
    pub id: SegmentId,
    pub epoch: u64,
    pub start: u64,
    pub end: u64,
    pub reason: EndpointReason,
    pub text: String,
}

pub fn parse_commit_line(line: &str) -> Option<CommitRecord> {
    let mut parts = line.splitn(7, ' ');
    if parts.next()? != "commit" {
        return None;
    }
    Some(CommitRecord {
        id: SegmentId(parts.next()?.parse().ok()?),
        epoch: parts.next()?.parse().ok()?,
        start: parts.next()?.parse().ok()?,
        end: parts.next()?.parse().ok()?,
        reason: EndpointReason::parse(parts.next()?)?,
        text: unfield(parts.next()?),
    })
}

pub fn parse_outcome_line(line: &str) -> Option<(SegmentId, OutcomeKind, String)> {
    let mut parts = line.splitn(4, ' ');
    if parts.next()? != "outcome" {
        return None;
    }
    let id: u64 = parts.next()?.parse().ok()?;
    let kind = OutcomeKind::parse_token(parts.next()?)?;
    let target = unfield(parts.next().unwrap_or("-"));
    Some((SegmentId(id), kind, target))
}
