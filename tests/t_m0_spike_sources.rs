//! T-M0-SPIKE-SOURCES
//!
//! Manual evidence is a repository script. This revision has no CoreAudio run.

use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn t_m0_spike_sources() {
    let verdict = run("T-M0-SPIKE-SOURCES");
    assert_eq!(verdict, "not_run");
    assert_ne!(verdict, "pass");
}

/// Tap open has to start the aggregate. The microphone `AudioDeviceStart` does
/// not. AC1 stays unchecked until a Mac operator records Allow and Deny.
#[test]
fn t_m0_spike_tap_starts_aggregate_without_checking_ac1() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src = fs::read_to_string(root.join("native/process_tap.m")).unwrap();
    let open = tap_open_body(&src);
    assert!(open.contains("AudioDeviceStart"), "{open}");
    assert!(open.contains("AudioDeviceStop"), "{open}");
    assert!(
        open.contains("@kAudioAggregateDeviceTapAutoStartKey : @0"),
        "{open}"
    );
    assert!(src.contains("TACO_CAPTURE_SPIKE_OPEN_MS"));
    let note = fs::read_to_string(root.join("docs/evidence/m0/audiocapture-prep.md")).unwrap();
    assert!(note.contains("- [ ] **M0-AC1:**"));
    assert!(!note.contains("- [x] **M0-AC1:**"));
    assert!(note.contains("No Mac run of this start is recorded"));
}

#[test]
fn t_m0_native_scripts_are_not_run() {
    assert_eq!(run("T-M0-LINK-THREE-MODELS"), "not_run");
    assert_eq!(run("T-M0-CANCEL-LIFECYCLE"), "not_run");
}

fn tap_open_body(src: &str) -> &str {
    let start = src.find("int taco_process_tap_open").expect("tap open");
    let rest = &src[start..];
    let end = rest
        .find("static OSStatus taco_input_ioproc")
        .expect("input proc follows tap open");
    &rest[..end]
}

fn run(id: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let script = root.join("scripts/qual").join(id);
    let dir = std::env::temp_dir().join(format!("taco-{id}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let out = dir.join("evidence.txt");
    let status = Command::new("sh")
        .arg(&script)
        .arg(&out)
        .status()
        .expect("qual script");
    assert_eq!(status.code(), Some(2));
    let text = fs::read_to_string(&out).expect("evidence");
    let _ = fs::remove_dir_all(&dir);
    assert!(text.contains("blocks="), "{text}");
    text.lines()
        .find_map(|line| line.strip_prefix("verdict="))
        .unwrap()
        .to_string()
}
