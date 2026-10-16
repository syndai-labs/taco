//! Acceptance caps. The numbers live in `fixtures/m0/limit-vectors.txt`.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::error::ErrorCode;

pub const EMBEDDED: &str = include_str!("../fixtures/m0/limit-vectors.txt");

pub const KEYS: &[&str] = &[
    "alignment_ms",
    "adequacy_min_score",
    "adequacy_percent",
    "asr_active",
    "asr_clean_percent",
    "asr_noise_percent",
    "asr_waiting",
    "cancel_reserve_ms",
    "cli_backlog_bytes",
    "cli_backlog_records",
    "completion_reserve",
    "corpus_long_passages",
    "corpus_minutes",
    "corpus_speakers",
    "corpus_utterances",
    "coverage_percent",
    "critical_cases",
    "cycle_tolerance_bytes",
    "draft_hz",
    "drain_ms",
    "endurance_cells",
    "endurance_minutes",
    "endpoint_silence_ms",
    "fetch_attempts",
    "fetch_redirects",
    "forced_max_ms",
    "forced_p95_ms",
    "freshness_ms",
    "host_swap_minutes",
    "ineligible_percent",
    "job_deadline_ms",
    "latency_max_ms",
    "latency_p50_ms",
    "latency_p95_ms",
    "latency_p99_ms",
    "lifecycle_cycles",
    "median_growth_bytes",
    "memory_ceiling_bytes",
    "memory_ceiling_raise_bytes",
    "model_window_ms",
    "mt_active",
    "mt_waiting",
    "native_channels_max",
    "native_channels_min",
    "native_rate_max_hz",
    "native_rate_min_hz",
    "nonspeech_minutes",
    "normalized_channels",
    "normalized_rate_hz",
    "occupancy_max_percent",
    "occupancy_min_percent",
    "output_deadline_ms",
    "pcm_bytes_max",
    "pcm_bytes_min",
    "pending_drafts",
    "preroll_ms",
    "qualify_rate_a_hz",
    "qualify_rate_b_hz",
    "raw_ring_bytes",
    "raw_ring_seconds",
    "segment_text_bytes",
    "shutdown_ms",
    "slope_milli_mib_per_min",
    "slope_revision_max_milli_mib_per_min",
    "smoke_bilingual_minutes",
    "smoke_max_ms",
    "smoke_source_minutes",
    "smoke_utterances",
    "status_events",
    "sustained_forced_segments",
    "sustained_minutes",
    "thermal_seconds",
    "timeline_error_ms",
    "token_budget",
    "trailing_silence_ms",
    "ui_completed_rows",
    "ui_text_bytes",
    "warmup_ms",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limits {
    map: BTreeMap<String, u64>,
}

impl Limits {
    pub fn get(&self, key: &str) -> u64 {
        *self
            .map
            .get(key)
            .unwrap_or_else(|| panic!("missing limit {key}"))
    }

    pub fn segment_text_bytes(&self) -> u64 {
        self.get("segment_text_bytes")
    }
    pub fn token_budget(&self) -> u64 {
        self.get("token_budget")
    }
    pub fn completion_reserve(&self) -> u64 {
        self.get("completion_reserve")
    }
    pub fn mt_waiting(&self) -> u64 {
        self.get("mt_waiting")
    }
    pub fn asr_waiting(&self) -> u64 {
        self.get("asr_waiting")
    }
    pub fn model_window_ms(&self) -> u64 {
        self.get("model_window_ms")
    }
    pub fn raw_ring_seconds(&self) -> u64 {
        self.get("raw_ring_seconds")
    }
    pub fn raw_ring_bytes(&self) -> u64 {
        self.get("raw_ring_bytes")
    }
    pub fn memory_ceiling_bytes(&self) -> u64 {
        self.get("memory_ceiling_bytes")
    }
    pub fn ui_text_bytes(&self) -> u64 {
        self.get("ui_text_bytes")
    }
    pub fn cli_backlog_bytes(&self) -> u64 {
        self.get("cli_backlog_bytes")
    }
    pub fn ui_completed_rows(&self) -> u64 {
        self.get("ui_completed_rows")
    }
    pub fn status_events(&self) -> u64 {
        self.get("status_events")
    }
    pub fn fetch_redirects(&self) -> u64 {
        self.get("fetch_redirects")
    }
    pub fn fetch_attempts(&self) -> u64 {
        self.get("fetch_attempts")
    }
    pub fn drain_ms(&self) -> u64 {
        self.get("drain_ms")
    }
    pub fn cancel_reserve_ms(&self) -> u64 {
        self.get("cancel_reserve_ms")
    }
    pub fn shutdown_ms(&self) -> u64 {
        self.get("shutdown_ms")
    }
    pub fn native_channels_min(&self) -> u64 {
        self.get("native_channels_min")
    }
    pub fn native_channels_max(&self) -> u64 {
        self.get("native_channels_max")
    }
    pub fn native_rate_min_hz(&self) -> u64 {
        self.get("native_rate_min_hz")
    }
    pub fn native_rate_max_hz(&self) -> u64 {
        self.get("native_rate_max_hz")
    }
    pub fn pcm_bytes_min(&self) -> u64 {
        self.get("pcm_bytes_min")
    }
    pub fn pcm_bytes_max(&self) -> u64 {
        self.get("pcm_bytes_max")
    }
    pub fn normalized_rate_hz(&self) -> u64 {
        self.get("normalized_rate_hz")
    }
    pub fn corpus_utterances(&self) -> u64 {
        self.get("corpus_utterances")
    }
}

pub fn parse(text: &str) -> Result<Limits, String> {
    let mut map = BTreeMap::new();
    for (lineno, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("line {}: expected key = value", lineno + 1))?;
        let key = key.trim();
        let value = value.trim();
        if !KEYS.contains(&key) {
            return Err(format!("unknown limit {key}"));
        }
        let parsed: u64 = value
            .parse()
            .map_err(|_| format!("bad integer for {key}"))?;
        if map.insert(key.to_string(), parsed).is_some() {
            return Err(format!("duplicate limit {key}"));
        }
    }
    for key in KEYS {
        if !map.contains_key(*key) {
            return Err(format!("missing limit {key}"));
        }
    }
    if map.len() != KEYS.len() {
        return Err("limit vector key count drifted".into());
    }
    Ok(Limits { map })
}

pub fn limits() -> &'static Limits {
    static CELL: OnceLock<Limits> = OnceLock::new();
    CELL.get_or_init(|| parse(EMBEDDED).expect("embedded limit vector"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RingPlan {
    pub capacity_frames: u64,
    pub two_second_binds: bool,
}

/// Reject unsupported formats before any buffer allocation.
/// `capacity_frames = min(seconds × rate × channels, byte_cap / bytes_per_frame)`.
pub fn plan_ring(
    limits: &Limits,
    rate_hz: u64,
    channels: u64,
    bytes_per_sample: u64,
) -> Result<RingPlan, ErrorCode> {
    if channels < limits.native_channels_min() || channels > limits.native_channels_max() {
        return Err(ErrorCode::InvalidFormat);
    }
    if rate_hz < limits.native_rate_min_hz() || rate_hz > limits.native_rate_max_hz() {
        return Err(ErrorCode::InvalidFormat);
    }
    if bytes_per_sample != limits.pcm_bytes_min() && bytes_per_sample != limits.pcm_bytes_max() {
        return Err(ErrorCode::InvalidFormat);
    }
    // Acceptance: capacity_frames = min(2s × rate × channels, 16 MiB / bytes_per_frame).
    // The left term already counts every channel, so bytes_per_frame is the sample width.
    if bytes_per_sample == 0 {
        return Err(ErrorCode::InvalidFormat);
    }
    let frames_time = limits
        .raw_ring_seconds()
        .saturating_mul(rate_hz)
        .saturating_mul(channels);
    let frames_bytes = limits.raw_ring_bytes() / bytes_per_sample;
    Ok(RingPlan {
        capacity_frames: frames_time.min(frames_bytes),
        two_second_binds: frames_time <= frames_bytes,
    })
}

pub fn named_byte_caps(limits: &Limits) -> u64 {
    limits
        .raw_ring_bytes()
        .saturating_add(limits.ui_text_bytes())
        .saturating_add(limits.cli_backlog_bytes())
}

/// Resident model bytes are measured on device. Unknown is not under the ceiling.
pub fn ceiling_holds(limits: &Limits, resident_model_bytes: Option<u64>) -> bool {
    let Some(models) = resident_model_bytes else {
        return false;
    };
    named_byte_caps(limits).saturating_add(models) <= limits.memory_ceiling_bytes()
}

pub fn shutdown_consistent(limits: &Limits) -> bool {
    limits.drain_ms().saturating_add(limits.cancel_reserve_ms()) == limits.shutdown_ms()
}
