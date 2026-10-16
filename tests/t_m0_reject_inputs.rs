//! T-M0-REJECT-INPUTS
//!
//! Each case names a closed error code and is rejected before capture allocation.

use std::process::Command;

use taco::error::ErrorCode;
use taco::language::parse_pair;
use taco::ledger::{check_text, Ledger, OutcomeKind, SegmentId};
use taco::limits::parse;
use taco::selector::parse_selector;
use taco::session::{self, Edge, RunRequest, Session, SessionState};

fn limits() -> taco::Limits {
    parse(include_str!("../fixtures/m0/limit-vectors.txt")).expect("limit vector")
}

fn taco_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_taco"))
}

fn run(args: &[&str]) -> std::process::Output {
    taco_bin().args(args).output().expect("spawn taco")
}

fn assert_code(err: ErrorCode, expected: ErrorCode) {
    assert_eq!(err, expected, "closed code {}", err.as_str());
    assert!(!err.as_str().is_empty());
}

#[test]
fn t_m0_reject_inputs_two_sources() {
    let err = parse_many(&["default".into(), "system".into()]);
    assert_code(err, ErrorCode::MultipleSources);
    let output = run(&[
        "run", "--source", "default", "--source", "system", "--from", "en", "--to", "zh",
    ]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("multiple_sources"), "{stderr}");
}

#[test]
fn t_m0_reject_inputs_bare_device_name_and_bundle_id() {
    assert_code(
        parse_selector("MacBook Pro Microphone").unwrap_err(),
        ErrorCode::BareNameRejected,
    );
    assert_code(
        parse_selector("com.apple.Safari").unwrap_err(),
        ErrorCode::BareNameRejected,
    );
    let output = run(&[
        "run",
        "--source",
        "com.apple.Safari",
        "--from",
        "en",
        "--to",
        "zh",
    ]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("bare_name_rejected"));
}

#[test]
fn t_m0_reject_inputs_stdin_and_path() {
    assert_code(
        parse_selector("stdin").unwrap_err(),
        ErrorCode::StdinSourceRejected,
    );
    assert_code(
        parse_selector("-").unwrap_err(),
        ErrorCode::StdinSourceRejected,
    );
    assert_code(
        parse_selector("/tmp/clip.wav").unwrap_err(),
        ErrorCode::PathSourceRejected,
    );
    assert_code(
        parse_selector("clip.wav").unwrap_err(),
        ErrorCode::PathSourceRejected,
    );
    assert_code(
        parse_selector("input:https://example.com/a.wav").unwrap_err(),
        ErrorCode::InvalidSelectorSyntax,
    );
    assert_code(
        parse_selector("https://example.com/a.wav").unwrap_err(),
        ErrorCode::InvalidSelectorSyntax,
    );
    let output = run(&["run", "--source", "stdin", "--from", "en", "--to", "zh"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("stdin_source_rejected"));
}

#[test]
fn t_m0_reject_inputs_identical_and_unknown_languages() {
    assert_code(
        parse_pair("en", "en").unwrap_err(),
        ErrorCode::IdenticalLanguagePair,
    );
    assert_code(
        parse_pair("zh", "zh").unwrap_err(),
        ErrorCode::IdenticalLanguagePair,
    );
    assert_code(
        parse_pair("en", "fr").unwrap_err(),
        ErrorCode::LanguageTagUnknown,
    );
    assert_code(
        parse_pair("auto", "zh").unwrap_err(),
        ErrorCode::LanguageTagUnknown,
    );
    let output = run(&["run", "--source", "default", "--from", "en", "--to", "en"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("identical_language_pair"));
    let output = run(&["run", "--source", "default", "--from", "en", "--to", "fr"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("language_tag_unknown"));
}

#[test]
fn t_m0_reject_inputs_text_over_byte_cap() {
    let limits = limits();
    let cap = limits.segment_text_bytes() as usize;
    assert!(check_text(&limits, &"a".repeat(cap)).is_ok());
    assert_code(
        check_text(&limits, &"a".repeat(cap + 1)).unwrap_err(),
        ErrorCode::TextTooLong,
    );
    assert_code(
        check_text(&limits, &"测".repeat(cap)).unwrap_err(),
        ErrorCode::TextTooLong,
    );
}

#[test]
fn t_m0_reject_inputs_second_outcome() {
    let mut ledger = Ledger::default();
    let id = SegmentId(7);
    ledger
        .record_outcome(id, OutcomeKind::Complete)
        .expect("first outcome");
    assert_code(
        ledger
            .record_outcome(id, OutcomeKind::Cancelled)
            .unwrap_err(),
        ErrorCode::DuplicateOutcome,
    );
    assert_eq!(ledger.outcome(id), Some(OutcomeKind::Complete));
    ledger
        .record_outcome(SegmentId(8), OutcomeKind::TimedOut)
        .expect("other id");
}

#[test]
fn t_m0_reject_inputs_skips_state_machine() {
    let mut session = Session::new();
    assert_eq!(session.state, SessionState::Preflight);
    assert_code(
        session.apply(Edge::Ready).unwrap_err(),
        ErrorCode::IllegalTransition,
    );
    assert_eq!(session.state, SessionState::Preflight);
    assert!(!session.capture_started);
    session.apply(Edge::Start).unwrap();
    session.apply(Edge::Ready).unwrap();
    assert_eq!(session.state, SessionState::Running);
}

#[test]
fn t_m0_reject_inputs_valid_selector_does_not_capture() {
    let mut session = Session::new();
    let sources = vec!["default".to_string()];
    let req = RunRequest {
        sources: &sources,
        from: "en",
        to: "zh",
        ui: "cli",
    };
    // Digests are measured; preflight still does not start capture.
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        let ok = session::preflight(&mut session, &req).expect("pinned macOS preflight");
        assert_eq!(ok.selector, parse_selector("default").unwrap());
        assert!(!session.capture_started);
    } else {
        let err = session::preflight(&mut session, &req).unwrap_err();
        assert_code(err, ErrorCode::PlatformUnsupported);
        assert_eq!(session.state, SessionState::Failed);
        assert!(!session.capture_started);
    }
    assert_eq!(
        session::start_capture(&mut session).unwrap_err(),
        ErrorCode::CaptureNotAvailable
    );
    assert!(!session.capture_started);
}

#[test]
fn t_m0_reject_inputs_accepts_closed_selectors() {
    assert!(parse_selector("default").is_ok());
    assert!(parse_selector("system").is_ok());
    assert!(parse_selector("input:AppleHDAEngineInput:1").is_ok());
    assert!(parse_selector("app:com.apple.Safari").is_ok());
    assert!(parse_selector("pid:42").is_ok());
    assert!(parse_pair("en", "zh").is_ok());
    assert!(parse_pair("zh", "en").is_ok());
}

fn parse_many(sources: &[String]) -> ErrorCode {
    let mut session = Session::new();
    session::preflight(
        &mut session,
        &RunRequest {
            sources,
            from: "en",
            to: "zh",
            ui: "cli",
        },
    )
    .unwrap_err()
}
