//! Mach-O gate for the release artifact. ELF is not a pass.

use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    NotMachO { detail: &'static str },
    Rejected { reasons: Vec<String> },
    Pass,
}

impl Verdict {
    pub fn is_pass(&self) -> bool {
        matches!(self, Self::Pass)
    }
}

const SIDECAR_EXT: &[&str] = &[
    "metal", "so", "dylib", "gguf", "onnx", "pt", "metallib", "bin",
];

pub fn inspect_path(path: &Path) -> Verdict {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return Verdict::Rejected {
                reasons: vec![format!("unreadable: {error}")],
            };
        }
    };
    let mut verdict = inspect_bytes(&bytes);
    if verdict.is_pass() {
        if let Some(reasons) = sidecar_reasons(path) {
            verdict = Verdict::Rejected { reasons };
        }
    }
    verdict
}

pub fn inspect_bytes(bytes: &[u8]) -> Verdict {
    if bytes.len() < 4 {
        return Verdict::NotMachO { detail: "short" };
    }
    if bytes.starts_with(&[0x7f, b'E', b'L', b'F']) {
        return Verdict::NotMachO { detail: "elf" };
    }
    let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    match magic {
        0xFEEDFACF => parse_macho_64(bytes),
        0xCFFAEDFE => Verdict::Rejected {
            reasons: vec!["byte-swapped mach-o".into()],
        },
        0xFEEDFACE => Verdict::Rejected {
            reasons: vec!["mach-o 32-bit".into()],
        },
        0xCAFEBABE | 0xBEBAFECA => Verdict::Rejected {
            reasons: vec!["fat binary is not one arm64 mach-o".into()],
        },
        _ => Verdict::NotMachO { detail: "unknown" },
    }
}

fn parse_macho_64(bytes: &[u8]) -> Verdict {
    if bytes.len() < 32 {
        return Verdict::Rejected {
            reasons: vec!["truncated header".into()],
        };
    }
    let mut reasons = Vec::new();
    let cputype = u32le(&bytes[4..8]);
    let filetype = u32le(&bytes[12..16]);
    let ncmds = u32le(&bytes[16..20]);
    let sizeofcmds = u32le(&bytes[20..24]) as usize;
    if cputype != 0x0100_000C {
        reasons.push(format!("cputype {cputype:#x} is not arm64"));
    }
    if filetype != 2 {
        reasons.push("filetype is not MH_EXECUTE".into());
    }
    let end = 32usize.saturating_add(sizeofcmds);
    if end > bytes.len() {
        reasons.push("load commands extend past the file".into());
        return Verdict::Rejected { reasons };
    }
    let mut offset = 32usize;
    for _ in 0..ncmds {
        if offset + 8 > end {
            reasons.push("truncated load command".into());
            break;
        }
        let cmd = u32le(&bytes[offset..offset + 4]);
        let cmdsize = u32le(&bytes[offset + 4..offset + 8]) as usize;
        if cmdsize < 8 || offset + cmdsize > end {
            reasons.push("bad cmdsize".into());
            break;
        }
        if is_dylib(cmd) {
            match dylib_name(&bytes[offset..offset + cmdsize]) {
                Ok(name) if is_system(&name) => {}
                Ok(name) => reasons.push(format!("non-system dylib {name}")),
                Err(reason) => reasons.push(reason),
            }
        }
        offset += cmdsize;
    }
    if !contains(bytes, b"NSMicrophoneUsageDescription") {
        reasons.push("missing NSMicrophoneUsageDescription".into());
    }
    if !contains(bytes, b"NSAudioCaptureUsageDescription") {
        reasons.push("missing NSAudioCaptureUsageDescription".into());
    }
    if reasons.is_empty() {
        Verdict::Pass
    } else {
        Verdict::Rejected { reasons }
    }
}

fn is_dylib(cmd: u32) -> bool {
    matches!(cmd, 0xC | 0x8000_0018 | 0x8000_001F | 0x8000_0023)
}

fn is_system(name: &str) -> bool {
    name.starts_with("/usr/lib/") || name.starts_with("/System/Library/")
}

fn dylib_name(cmd: &[u8]) -> Result<String, String> {
    if cmd.len() < 12 {
        return Err("short dylib command".into());
    }
    let name_off = u32le(&cmd[8..12]) as usize;
    if name_off >= cmd.len() {
        return Err("dylib name offset".into());
    }
    let bytes = &cmd[name_off..];
    let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
    String::from_utf8(bytes[..end].to_vec()).map_err(|_| "dylib name utf-8".into())
}

fn sidecar_reasons(path: &Path) -> Option<Vec<String>> {
    let dir = path.parent()?;
    let mut reasons = Vec::new();
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(ext) = name.rsplit_once('.').map(|(_, ext)| ext) {
            if SIDECAR_EXT.contains(&ext) {
                reasons.push(format!("adjacent {name}"));
            }
        }
    }
    if reasons.is_empty() {
        None
    } else {
        Some(reasons)
    }
}

fn u32le(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|window| window == needle)
}
