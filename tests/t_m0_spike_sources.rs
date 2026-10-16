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

#[test]
fn t_m0_native_scripts_are_not_run() {
    assert_eq!(run("T-M0-LINK-THREE-MODELS"), "not_run");
    assert_eq!(run("T-M0-CANCEL-LIFECYCLE"), "not_run");
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
