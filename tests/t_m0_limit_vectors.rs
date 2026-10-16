//! T-M0-LIMIT-VECTORS
//!
//! The fixture is the acceptance-plan copy. Behavioral checks pass the parsed
//! numbers in, including a mutated copy, so a hardcoded constant cannot pass.

use std::fs;
use std::path::PathBuf;

use taco::error::ErrorCode;
use taco::ledger::{
    allow_redirect, allow_retry, budget_failure, check_text, check_window_ms, Admit, AdmitQueue,
    FailureDetail, Ledger, OutcomeKind, SegmentId, StatusRing, UiRows,
};
use taco::limits::{self, plan_ring, shutdown_consistent, Limits};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/m0/limit-vectors.txt")
}

fn load() -> Limits {
    let text = fs::read_to_string(fixture_path()).expect("read limit vector");
    assert_eq!(text, limits::EMBEDDED);
    assert!(text.len() < 64 * 1024, "fixture must not carry weights");
    assert!(!text.contains("GGUF"));
    assert!(!text.bytes().any(|b| b == 0));
    limits::parse(&text).expect("parse limit vector")
}

fn acceptance() -> String {
    fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/roadmap/acceptance.md"))
        .expect("acceptance plan")
}

fn row(md: &str, label: &str) -> String {
    md.lines()
        .find(|line| line.starts_with(&format!("| {label} |")))
        .unwrap_or_else(|| panic!("missing acceptance row {label}"))
        .to_string()
}

#[test]
fn t_m0_limit_vectors_match_acceptance_plan() {
    let limits = load();
    let md = acceptance();

    let native = row(&md, "Native input");
    assert!(native.contains("1–8") || native.contains("1-8"), "{native}");
    assert!(
        native.contains("8–192") || native.contains("8-192"),
        "{native}"
    );
    assert_eq!(limits.native_channels_min(), 1);
    assert_eq!(limits.native_channels_max(), 8);
    assert_eq!(limits.native_rate_min_hz(), 8_000);
    assert_eq!(limits.native_rate_max_hz(), 192_000);

    let ring = row(&md, "Raw ring");
    assert!(ring.contains("2 seconds"), "{ring}");
    assert!(ring.contains("16 MiB"), "{ring}");
    assert_eq!(limits.raw_ring_seconds(), 2);
    assert_eq!(limits.raw_ring_bytes(), 16 * 1024 * 1024);

    let freshness = row(&md, "Freshness");
    assert!(freshness.contains("2 seconds"), "{freshness}");
    assert_eq!(limits.get("freshness_ms"), 2_000);

    let normalized = row(&md, "Normalized audio");
    assert!(normalized.contains("16 kHz"), "{normalized}");
    assert!(normalized.contains("20 ms"), "{normalized}");
    assert_eq!(limits.get("normalized_rate_hz"), 16_000);
    assert_eq!(limits.get("normalized_channels"), 1);
    assert_eq!(limits.get("timeline_error_ms"), 20);

    let window = row(&md, "Model window");
    assert!(window.contains("15.0 s"), "{window}");
    assert_eq!(limits.model_window_ms(), 15_000);

    let preroll = row(&md, "Pre-roll");
    assert!(preroll.contains("300 ms"), "{preroll}");
    assert_eq!(limits.get("preroll_ms"), 300);

    let silence = row(&md, "Endpoint silence");
    assert!(silence.contains("500 ms"), "{silence}");
    assert_eq!(limits.get("endpoint_silence_ms"), 500);
    assert_eq!(limits.get("trailing_silence_ms"), 500);

    let asr = row(&md, "ASR queue");
    assert!(asr.contains("2 waiting"), "{asr}");
    assert!(asr.contains("1 active"), "{asr}");
    assert_eq!(limits.asr_waiting(), 2);
    assert_eq!(limits.get("asr_active"), 1);
    assert_eq!(limits.get("pending_drafts"), 1);

    let draft = row(&md, "Draft cadence");
    assert!(draft.contains("1 Hz"), "{draft}");
    assert_eq!(limits.get("draft_hz"), 1);

    let mt = row(&md, "Translation queue");
    assert!(mt.contains("4 waiting"), "{mt}");
    assert!(mt.contains("1 active"), "{mt}");
    assert_eq!(limits.mt_waiting(), 4);
    assert_eq!(limits.get("mt_active"), 1);

    let text = row(&md, "Segment text");
    assert!(text.contains("8 KiB"), "{text}");
    assert_eq!(limits.segment_text_bytes(), 8 * 1024);

    let tokens = row(&md, "Translation budget");
    assert!(
        tokens.contains("2,048") || tokens.contains("2048"),
        "{tokens}"
    );
    assert!(tokens.contains("512"), "{tokens}");
    assert_eq!(limits.token_budget(), 2048);
    assert_eq!(limits.completion_reserve(), 512);
    assert_ne!(limits.token_budget(), 4096);

    let ui = row(&md, "UI retention");
    assert!(ui.contains("200"), "{ui}");
    assert!(ui.contains("4 MiB"), "{ui}");
    assert_eq!(limits.ui_completed_rows(), 200);
    assert_eq!(limits.get("ui_text_bytes"), 4 * 1024 * 1024);

    let cli = row(&md, "CLI backlog");
    assert!(cli.contains("64"), "{cli}");
    assert!(cli.contains("1 MiB"), "{cli}");
    assert_eq!(limits.get("cli_backlog_records"), 64);
    assert_eq!(limits.get("cli_backlog_bytes"), 1024 * 1024);

    let output = row(&md, "TUI present");
    assert!(output.contains("2 s"), "{output}");
    assert_eq!(limits.get("output_deadline_ms"), 2_000);

    let status = row(&md, "Status memory");
    assert!(status.contains("128"), "{status}");
    assert_eq!(limits.status_events(), 128);

    let job = row(&md, "Job deadline");
    assert!(job.contains("10 s"), "{job}");
    assert_eq!(limits.get("job_deadline_ms"), 10_000);

    let shutdown = row(&md, "Shutdown");
    assert!(shutdown.contains("10 s"), "{shutdown}");
    assert!(md.contains("Drain budget is the first 8 s"), "{md}");
    assert!(md.contains("The last 2 s are the cancellation"), "{md}");
    assert_eq!(limits.shutdown_ms(), 10_000);
    assert_eq!(limits.drain_ms(), 8_000);
    assert_eq!(limits.cancel_reserve_ms(), 2_000);
    assert!(shutdown_consistent(&limits));
    assert_ne!(limits.drain_ms() + limits.cancel_reserve_ms(), 12_000);

    let fetch = row(&md, "Fetch");
    assert!(
        fetch.contains("3 transport") || fetch.contains("≤3 transport"),
        "{fetch}"
    );
    assert!(
        fetch.contains("3 redirects") || fetch.contains("≤3 redirects"),
        "{fetch}"
    );
    assert_eq!(limits.fetch_attempts(), 3);
    assert_eq!(limits.fetch_redirects(), 3);

    let memory = row(&md, "Memory ceiling");
    assert!(memory.contains("6 GiB"), "{memory}");
    assert_eq!(limits.memory_ceiling_bytes(), 6 * 1024 * 1024 * 1024);
    assert!(md.contains("only up to 8 GiB"));
    assert_eq!(
        limits.get("memory_ceiling_raise_bytes"),
        8 * 1024 * 1024 * 1024
    );

    assert!(md.contains("| p50 | ≤2 s |"));
    assert!(md.contains("| p95 | ≤4 s |"));
    assert!(md.contains("| p99 | ≤8 s |"));
    assert!(md.contains("| max | ≤10 s |"));
    assert_eq!(limits.get("latency_p50_ms"), 2_000);
    assert_eq!(limits.get("latency_p95_ms"), 4_000);
    assert_eq!(limits.get("latency_p99_ms"), 8_000);
    assert_eq!(limits.get("latency_max_ms"), 10_000);
    assert!(md.contains("p95 ≤5 s"));
    assert_eq!(limits.get("forced_p95_ms"), 5_000);
    assert_eq!(limits.get("forced_max_ms"), 10_000);

    assert!(md.contains("≤0.25 MiB/min"));
    assert!(md.contains("only up to 1 MiB/min"));
    assert_eq!(limits.get("slope_milli_mib_per_min"), 250);
    assert_eq!(limits.get("slope_revision_max_milli_mib_per_min"), 1000);
    assert!(md.contains("64 MiB"));
    assert_eq!(limits.get("median_growth_bytes"), 64 * 1024 * 1024);
    assert!(md.contains("32 MiB"));
    assert_eq!(limits.get("cycle_tolerance_bytes"), 32 * 1024 * 1024);

    assert!(md.contains("5 minutes"));
    assert_eq!(limits.get("warmup_ms"), 5 * 60 * 1000);
    assert!(md.contains("60"));
    assert_eq!(limits.get("thermal_seconds"), 60);
    assert!(md.contains("above 20 ms"));
    assert_eq!(limits.get("alignment_ms"), 20);
    assert!(md.contains("more than 5%"));
    assert_eq!(limits.get("ineligible_percent"), 5);

    assert!(md.contains("WER ≤15%"));
    assert!(md.contains("≤25%"));
    assert_eq!(limits.get("asr_clean_percent"), 15);
    assert_eq!(limits.get("asr_noise_percent"), 25);
    assert!(md.contains("≥98%"));
    assert_eq!(limits.get("coverage_percent"), 98);
    assert!(md.contains("10 minutes digital silence"));
    assert_eq!(limits.get("nonspeech_minutes"), 10);
    assert!(md.contains("≥95%"));
    assert_eq!(limits.get("adequacy_percent"), 95);
    assert_eq!(limits.get("adequacy_min_score"), 4);
    assert!(md.contains("≥50-case"));
    assert_eq!(limits.get("critical_cases"), 50);

    assert!(md.contains("at least 200 natural"));
    assert_eq!(limits.corpus_utterances(), 200);
    assert!(md.contains("30 minutes of speech"));
    assert_eq!(limits.get("corpus_minutes"), 30);
    assert!(md.contains("at least 5 speakers"));
    assert_eq!(limits.get("corpus_speakers"), 5);
    assert!(md.contains("at least 20 long"));
    assert_eq!(limits.get("corpus_long_passages"), 20);
    assert!(md.contains("50–70%") || md.contains("50-70%"));
    assert_eq!(limits.get("occupancy_min_percent"), 50);
    assert_eq!(limits.get("occupancy_max_percent"), 70);
    assert!(md.contains("10-minute sustained-speech"));
    assert_eq!(limits.get("sustained_minutes"), 10);
    assert!(md.contains("at least 30 forced"));
    assert_eq!(limits.get("sustained_forced_segments"), 30);
    assert!(md.contains("One hundred in-process"));
    assert_eq!(limits.get("lifecycle_cycles"), 100);

    assert!(md.contains("≥10 measured minutes"));
    assert_eq!(limits.get("smoke_source_minutes"), 10);
    assert!(md.contains("≥15 measured minutes"));
    assert_eq!(limits.get("smoke_bilingual_minutes"), 15);
    assert!(md.contains("≥50 timed utterances"));
    assert_eq!(limits.get("smoke_utterances"), 50);
    assert_eq!(limits.get("smoke_max_ms"), 10_000);

    assert!(md.contains("Sixteen endurance cells"));
    assert_eq!(limits.get("endurance_cells"), 16);
    assert!(md.contains("60 measured minutes"));
    assert_eq!(limits.get("endurance_minutes"), 60);
    assert!(md.contains("at least 10 minutes"));
    assert_eq!(limits.get("host_swap_minutes"), 10);
    assert!(md.contains("44.1"));
    assert!(md.contains("48 kHz"));
    assert_eq!(limits.get("qualify_rate_a_hz"), 44_100);
    assert_eq!(limits.get("qualify_rate_b_hz"), 48_000);

    assert_eq!(limits::KEYS.len(), 78, "fixture dropped or added a cap");
}

#[test]
fn t_m0_limit_vectors_refusals_follow_the_fixture() {
    let mut limits = load();
    let cap = limits.segment_text_bytes() as usize;
    assert!(check_text(&limits, &"a".repeat(cap)).is_ok());
    assert_eq!(
        check_text(&limits, &"a".repeat(cap + 1)).unwrap_err(),
        ErrorCode::TextTooLong
    );
    limits = load();
    // The function must read the struct, not a private constant.
    let saved = limits.segment_text_bytes();
    assert!(saved > 4);
    // Rebuild via parse of a modified copy so the value is not the embedded constant.
    let text = fs::read_to_string(fixture_path()).unwrap().replace(
        &format!("segment_text_bytes = {saved}"),
        "segment_text_bytes = 4",
    );
    let tight = limits::parse(&text).unwrap();
    assert_eq!(
        check_text(&tight, "hello").unwrap_err(),
        ErrorCode::TextTooLong
    );

    let limits = load();
    assert_eq!(
        budget_failure(&limits, limits.segment_text_bytes() + 1, 1, 1),
        Some(FailureDetail::TooLong)
    );
    let room = limits.token_budget() - limits.completion_reserve();
    assert_eq!(
        budget_failure(&limits, 1, room + 1, 1),
        Some(FailureDetail::TooLong)
    );
    assert_eq!(
        budget_failure(&limits, 1, 1, limits.completion_reserve() + 1),
        Some(FailureDetail::TokenExhausted)
    );
    assert_eq!(
        budget_failure(&limits, 1, room, limits.completion_reserve()),
        None
    );

    assert_eq!(
        check_window_ms(&limits, limits.model_window_ms()).unwrap(),
        ()
    );
    assert_eq!(
        check_window_ms(&limits, limits.model_window_ms() + 1).unwrap_err(),
        ErrorCode::WindowExceeded
    );

    let mut mt = AdmitQueue::new(limits.mt_waiting() as usize);
    assert_eq!(mt.admit(), Admit::Active);
    for _ in 0..limits.mt_waiting() {
        assert_eq!(mt.admit(), Admit::Queued);
    }
    let depth = mt.depth();
    assert_eq!(mt.admit(), Admit::Refused);
    assert_eq!(mt.depth(), depth);

    let mut asr = AdmitQueue::new(limits.asr_waiting() as usize);
    assert_eq!(asr.admit(), Admit::Active);
    for _ in 0..limits.asr_waiting() {
        assert_eq!(asr.admit(), Admit::Queued);
    }
    assert_eq!(asr.admit(), Admit::Refused);

    let mut tiny = AdmitQueue::new(1);
    assert_eq!(tiny.admit(), Admit::Active);
    assert_eq!(tiny.admit(), Admit::Queued);
    assert_eq!(tiny.admit(), Admit::Refused);

    assert!(allow_redirect(&limits, 0, "cdn.example", &["cdn.example"]).is_ok());
    assert!(allow_redirect(
        &limits,
        limits.fetch_redirects() - 1,
        "cdn.example",
        &["cdn.example"]
    )
    .is_ok());
    assert_eq!(
        allow_redirect(
            &limits,
            limits.fetch_redirects(),
            "cdn.example",
            &["cdn.example"]
        )
        .unwrap_err(),
        ErrorCode::RedirectRejected
    );
    assert_eq!(
        allow_redirect(&limits, 0, "evil.example", &["cdn.example"]).unwrap_err(),
        ErrorCode::RedirectRejected
    );
    assert!(allow_retry(&limits, limits.fetch_attempts() - 1).is_ok());
    assert_eq!(
        allow_retry(&limits, limits.fetch_attempts()).unwrap_err(),
        ErrorCode::FetchAttemptsExhausted
    );
}

#[test]
fn t_m0_limit_vectors_ring_formula_and_queues_do_not_hardcode() {
    let limits = load();
    for &rate in &[
        limits.native_rate_min_hz(),
        limits.get("qualify_rate_a_hz"),
        limits.get("qualify_rate_b_hz"),
        limits.native_rate_max_hz(),
    ] {
        for &channels in &[limits.native_channels_min(), limits.native_channels_max()] {
            for &width in &[limits.pcm_bytes_min(), limits.pcm_bytes_max()] {
                let plan = plan_ring(&limits, rate, channels, width).expect("supported format");
                assert!(
                    plan.two_second_binds,
                    "2s term must bind for {rate} Hz {channels} ch {width} B"
                );
                assert_eq!(
                    plan.capacity_frames,
                    limits.raw_ring_seconds() * rate * channels
                );
            }
        }
    }
    assert_eq!(
        plan_ring(&limits, limits.native_rate_min_hz() - 1, 1, 2).unwrap_err(),
        ErrorCode::InvalidFormat
    );
    assert_eq!(
        plan_ring(&limits, 48_000, limits.native_channels_max() + 1, 2).unwrap_err(),
        ErrorCode::InvalidFormat
    );
    assert_eq!(
        plan_ring(&limits, 48_000, 1, 8).unwrap_err(),
        ErrorCode::InvalidFormat
    );

    let text = fs::read_to_string(fixture_path()).unwrap().replace(
        &format!("raw_ring_bytes = {}", limits.raw_ring_bytes()),
        "raw_ring_bytes = 10",
    );
    let tight = limits::parse(&text).unwrap();
    let plan = plan_ring(&tight, 8_000, 1, 2).unwrap();
    assert!(!plan.two_second_binds);
    assert!(plan.capacity_frames < tight.raw_ring_seconds() * 8_000);
}

#[test]
fn t_m0_limit_vectors_retention_and_ceiling() {
    let limits = load();
    let mut rows = UiRows::new(limits.ui_completed_rows() as usize);
    for id in 0..limits.ui_completed_rows() + 5 {
        rows.push_completed(id);
    }
    assert_eq!(rows.completed_len(), limits.ui_completed_rows() as usize);
    rows.push_pending(999);
    assert_eq!(rows.pending_len(), 1);
    assert_eq!(
        rows.completed_len(),
        limits.ui_completed_rows() as usize - 1
    );

    let mut pending_only = UiRows::new(2);
    pending_only.push_pending(1);
    pending_only.push_pending(2);
    pending_only.push_pending(3);
    assert_eq!(
        pending_only.pending_len(),
        3,
        "pending rows are not evicted"
    );

    let mut ledger = Ledger::default();
    ledger
        .record_outcome(SegmentId(1), OutcomeKind::Complete)
        .unwrap();
    let mut status = StatusRing::new(limits.status_events() as usize);
    for code in 0..limits.status_events() + 10 {
        status.push(code as u16);
    }
    assert_eq!(status.len(), limits.status_events() as usize);
    assert_eq!(ledger.outcome(SegmentId(1)), Some(OutcomeKind::Complete));

    assert!(!limits::ceiling_holds(&limits, None));
    let static_caps = limits::named_byte_caps(&limits);
    assert!(static_caps < limits.memory_ceiling_bytes());
    assert!(limits::ceiling_holds(
        &limits,
        Some(limits.memory_ceiling_bytes() - static_caps)
    ));
    assert!(!limits::ceiling_holds(
        &limits,
        Some(limits.memory_ceiling_bytes())
    ));
}
