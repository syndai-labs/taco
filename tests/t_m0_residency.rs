//! In-process multi-cycle `phys_footprint` sample.
//!
//! Catalog metric: `task_vm_info.phys_footprint` after teardown, compared with
//! `cycle_tolerance_bytes` from the limit vector. RSS, the graphics ledger, and
//! malloc sizes are diagnostic. This file does not check M0-AC2 or M0-AC3.
//! Unset `TACO_RESIDENCY_CYCLES` records nothing and does not load a model.

use std::io::{stderr, Write};
use std::path::PathBuf;
use std::process;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use taco::engines::{self, VmSample};
use taco::limits;
use taco::session;

#[test]
fn multi_cycle_residency() {
    if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        panic!(
            "residency blocked: host is not macOS Apple Silicon. \
             Do not record a CPU or subprocess result."
        );
    }
    let Ok(raw) = std::env::var("TACO_RESIDENCY_CYCLES") else {
        eprintln!("residency_not_measured reason=TACO_RESIDENCY_CYCLES_unset");
        return;
    };
    let cycles: u32 = raw.parse().expect("TACO_RESIDENCY_CYCLES integer");
    assert!(cycles >= 1, "need at least one prove cycle");

    let dir = model_dir();
    let reserve = limits().cancel_reserve_ms();
    let tolerance = limits().get("cycle_tolerance_bytes");
    let catalog_cycles = limits().get("lifecycle_cycles");
    let pid = process::id();
    let exe = std::env::current_exe()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let baseline = engines::vm_sample();
    emit(0, "process_start", &baseline, &Peak::default(), 0, None);
    assert_eq!(baseline.ok, 1, "phys_footprint unread before load");

    let mut after = Vec::with_capacity(cycles as usize);
    let mut prove_ok = 1i32;
    for cycle in 1..=cycles {
        let peak = Arc::new(Peak::default());
        let stop = Arc::new(AtomicBool::new(false));
        let sampler = {
            let peak = Arc::clone(&peak);
            let stop = Arc::clone(&stop);
            thread::spawn(move || {
                while !stop.load(Ordering::Relaxed) {
                    peak.observe(&engines::vm_sample());
                    thread::sleep(Duration::from_millis(50));
                }
            })
        };
        let started = Instant::now();
        let proved = session::prove_native(&dir, reserve);
        stop.store(true, Ordering::Relaxed);
        let _ = sampler.join();
        let elapsed_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
        let sample = engines::vm_sample();
        let proof = match proved {
            Ok(proof) => proof,
            Err(err) => {
                emit(cycle, "prove_err", &sample, &peak, elapsed_ms, None);
                panic!("prove_native refused on cycle {cycle}: {err:?} dir={dir:?}");
            }
        };
        if proof.teardown != 1 || proof.link_three != 1 || proof.gpu_offload != 1 || proof.vad_ort != 1
        {
            prove_ok = 0;
        }
        emit(
            cycle,
            "after_teardown",
            &sample,
            &peak,
            elapsed_ms,
            Some((
                proof.link_three,
                proof.vad_ort,
                proof.gpu_offload,
                proof.teardown,
                proof.gpu_device.as_str(),
            )),
        );
        assert_eq!(sample.ok, 1, "phys_footprint unread on cycle {cycle}");
        assert!(sample.phys_footprint > 0, "zero phys_footprint on cycle {cycle}");
        after.push(sample.phys_footprint);
    }

    let first = after[0];
    let last = *after.last().unwrap();
    let max_after = after.iter().copied().max().unwrap();
    let min_after = after.iter().copied().min().unwrap();
    let within = last <= first.saturating_add(tolerance);
    let same_count = u64::from(cycles) == catalog_cycles;
    writeln!(
        stderr(),
        "residency_summary pid={pid} measured_cycles={cycles} catalog_cycles={catalog_cycles} \
         after_1={first} after_last={last} delta_last_minus_1={} max_after={max_after} \
         min_after={min_after} tolerance_bytes={tolerance} within_tolerance={} \
         catalog_cycle_count={} prove_predicates={prove_ok} exe={exe}",
        last.saturating_sub(first),
        if within { 1 } else { 0 },
        if same_count { 1 } else { 0 },
    )
    .ok();
    let _ = stderr().flush();
}

struct Peak {
    phys: AtomicU64,
    rss: AtomicU64,
    device: AtomicU64,
    graphics: AtomicU64,
    samples: AtomicU64,
}

impl Default for Peak {
    fn default() -> Self {
        Self {
            phys: AtomicU64::new(0),
            rss: AtomicU64::new(0),
            device: AtomicU64::new(0),
            graphics: AtomicU64::new(0),
            samples: AtomicU64::new(0),
        }
    }
}

impl Peak {
    fn observe(&self, sample: &VmSample) {
        if sample.ok != 1 {
            return;
        }
        fetch_max(&self.phys, sample.phys_footprint);
        fetch_max(&self.rss, sample.resident_size);
        fetch_max(&self.device, sample.device);
        if sample.graphics_ok == 1 && sample.graphics_footprint > 0 {
            fetch_max(&self.graphics, sample.graphics_footprint as u64);
        }
        self.samples.fetch_add(1, Ordering::Relaxed);
    }
}

fn fetch_max(slot: &AtomicU64, value: u64) {
    let mut current = slot.load(Ordering::Relaxed);
    while value > current {
        match slot.compare_exchange_weak(current, value, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(observed) => current = observed,
        }
    }
}

fn emit(
    cycle: u32,
    phase: &str,
    sample: &VmSample,
    peak: &Peak,
    elapsed_ms: u64,
    proof: Option<(i32, i32, i32, i32, &str)>,
) {
    let (link_three, vad_ort, gpu_offload, teardown, gpu_device) = proof.unwrap_or((0, 0, 0, 0, ""));
    writeln!(
        stderr(),
        "residency cycle={cycle} phase={phase} pid={} phys_footprint={} resident_size={} \
         device={} graphics_footprint={} graphics_nofootprint={} graphics_ok={} \
         malloc_in_use={} malloc_max={} ledger_phys_peak={} peak_phys={} peak_rss={} \
         peak_device={} peak_graphics={} samples={} elapsed_ms={elapsed_ms} \
         link_three={link_three} vad_ort={vad_ort} gpu_offload={gpu_offload} \
         teardown={teardown} gpu_device={gpu_device} sample_ok={}",
        process::id(),
        sample.phys_footprint,
        sample.resident_size,
        sample.device,
        sample.graphics_footprint,
        sample.graphics_nofootprint,
        sample.graphics_ok,
        sample.malloc_size_in_use,
        sample.malloc_max_size_in_use,
        sample.ledger_phys_footprint_peak,
        peak.phys.load(Ordering::Relaxed),
        peak.rss.load(Ordering::Relaxed),
        peak.device.load(Ordering::Relaxed),
        peak.graphics.load(Ordering::Relaxed),
        peak.samples.load(Ordering::Relaxed),
        sample.ok,
    )
    .ok();
    let _ = stderr().flush();
}

fn model_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("TACO_MODEL_DIR") {
        return PathBuf::from(dir);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../taco-m0-pins-cache")
}
