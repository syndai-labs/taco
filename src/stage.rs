//! Replaceable stage traits. Fakes implement these without CoreAudio or models.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::ledger::{CommittedSegment, OutcomeKind, SessionEvent};

#[derive(Debug, Clone, Copy)]
pub struct FrameMeta {
    pub epoch: u64,
    pub sequence: u64,
    pub frames: u32,
    pub rate_hz: u32,
    pub channels: u16,
}

/// Capture publishes into a preallocated ring. It does not run inference.
pub trait Capture {
    fn on_frame(&mut self, frame: FrameMeta);
}

pub trait Vad {
    fn push_samples(&mut self, samples: &[f32]) -> bool;
}

pub trait Asr {
    fn transcribe(&mut self, samples: &[f32]) -> String;
}

pub trait Translator {
    fn translate(&mut self, committed: &CommittedSegment) -> OutcomeKind;
}

pub trait View {
    fn show(&mut self, event: &SessionEvent);
}

/// Occupancy ring. `publish` does not allocate. Sample storage arrives with M2.
#[derive(Debug)]
pub struct LossRing {
    cap: usize,
    len: AtomicUsize,
    loss: AtomicU64,
}

impl LossRing {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            len: AtomicUsize::new(0),
            loss: AtomicU64::new(0),
        }
    }

    pub fn publish(&self) -> bool {
        loop {
            let current = self.len.load(Ordering::Relaxed);
            if current >= self.cap {
                self.loss.fetch_add(1, Ordering::Relaxed);
                return false;
            }
            if self
                .len
                .compare_exchange(current, current + 1, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return true;
            }
        }
    }

    pub fn loss(&self) -> u64 {
        self.loss.load(Ordering::Relaxed)
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::Relaxed)
    }
}
