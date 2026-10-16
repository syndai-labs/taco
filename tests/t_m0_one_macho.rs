//! T-M0-ONE-MACHO
//!
//! The built binary on this host is not a qualified Mach-O. Fixture bytes prove
//! the inspector rejects sidecars, non-system dylibs, and missing usage strings.

use std::fs;
use std::path::Path;
use std::process::Command;

use taco::macho::{inspect_bytes, inspect_path, Verdict};

fn le(n: u32) -> [u8; 4] {
    n.to_le_bytes()
}

fn macho(cputype: u32, dylib: &str, trailer: &[u8]) -> Vec<u8> {
    macho_cmd(0xC, cputype, dylib, trailer)
}

fn macho_cmd(cmd: u32, cputype: u32, dylib: &str, trailer: &[u8]) -> Vec<u8> {
    let name = dylib.as_bytes();
    let name_off = 24u32;
    let raw = name_off as usize + name.len() + 1;
    let cmdsize = (raw + 7) & !7;
    let mut bytes = vec![0u8; cmdsize];
    bytes[0..4].copy_from_slice(&le(cmd));
    bytes[4..8].copy_from_slice(&le(cmdsize as u32));
    bytes[8..12].copy_from_slice(&le(name_off));
    bytes[name_off as usize..name_off as usize + name.len()].copy_from_slice(name);
    let mut file = Vec::new();
    file.extend_from_slice(&le(0xFEEDFACF));
    file.extend_from_slice(&le(cputype));
    file.extend_from_slice(&le(0));
    file.extend_from_slice(&le(2));
    file.extend_from_slice(&le(1));
    file.extend_from_slice(&le(cmdsize as u32));
    file.extend_from_slice(&le(0));
    file.extend_from_slice(&le(0));
    file.extend_from_slice(&bytes);
    file.extend_from_slice(trailer);
    file
}

const USAGE: &[u8] = b"NSMicrophoneUsageDescription NSAudioCaptureUsageDescription";

#[test]
fn t_m0_one_macho_host_binary_is_not_a_pass() {
    let path = Path::new(env!("CARGO_BIN_EXE_taco"));
    let bytes = fs::read(path).expect("read taco");
    assert!(bytes
        .windows(b"NSMicrophoneUsageDescription".len())
        .any(|w| w == b"NSMicrophoneUsageDescription"));
    assert!(bytes
        .windows(b"NSAudioCaptureUsageDescription".len())
        .any(|w| w == b"NSAudioCaptureUsageDescription"));
    let verdict = inspect_path(path);
    assert!(
        !verdict.is_pass(),
        "must not claim Mach-O pass: {verdict:?}"
    );
    assert!(
        matches!(verdict, Verdict::NotMachO { detail: "elf" }),
        "this host binary is ELF, got {verdict:?}"
    );
}

#[test]
fn t_m0_one_macho_inspector_rejects_bad_artifacts() {
    assert!(matches!(
        inspect_bytes(&[0x7f, b'E', b'L', b'F']),
        Verdict::NotMachO { detail: "elf" }
    ));
    let good = macho(0x0100_000C, "/usr/lib/libSystem.B.dylib", USAGE);
    assert!(inspect_bytes(&good).is_pass(), "{:?}", inspect_bytes(&good));
    let foreign = macho(0x0100_000C, "/usr/local/lib/libfoo.dylib", USAGE);
    assert!(!inspect_bytes(&foreign).is_pass());
    let lazy = macho_cmd(0x20, 0x0100_000C, "/usr/local/lib/libfoo.dylib", USAGE);
    assert!(!inspect_bytes(&lazy).is_pass(), "lazy non-system dylib");
    let lazy_system = macho_cmd(0x20, 0x0100_000C, "/usr/lib/libSystem.B.dylib", USAGE);
    assert!(inspect_bytes(&lazy_system).is_pass(), "system lazy dylib stays");
    let missing = macho(0x0100_000C, "/usr/lib/libSystem.B.dylib", b"");
    assert!(!inspect_bytes(&missing).is_pass());
    let intel = macho(0x0100_0007, "/usr/lib/libSystem.B.dylib", USAGE);
    assert!(!inspect_bytes(&intel).is_pass());

    let dir = std::env::temp_dir().join(format!("taco-macho-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let bin = dir.join("taco");
    fs::write(&bin, &good).unwrap();
    assert!(inspect_path(&bin).is_pass());
    fs::write(dir.join("shader.metal"), b"kernel").unwrap();
    assert!(!inspect_path(&bin).is_pass(), "sidecar must fail the gate");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn t_m0_one_macho_manual_script_is_not_run() {
    let verdict = script_verdict("T-M0-ONE-MACHO");
    assert_eq!(verdict, "not_run");
}

fn script_verdict(id: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let script = root.join("scripts/qual").join(id);
    let dir = std::env::temp_dir().join(format!("taco-qual-{}-{}", id, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let out = dir.join("evidence.txt");
    let status = Command::new("sh")
        .arg(&script)
        .arg(&out)
        .status()
        .expect("run qual script");
    assert_eq!(status.code(), Some(2), "{id} must not exit 0");
    let text = fs::read_to_string(&out).unwrap();
    let _ = fs::remove_dir_all(&dir);
    text.lines()
        .find_map(|line| line.strip_prefix("verdict="))
        .unwrap_or("")
        .to_string()
}
