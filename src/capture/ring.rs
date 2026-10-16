//! Preallocated capture ring. `push_bytes` does not allocate, log, or wait.

use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering};

use crate::error::ErrorCode;
use crate::limits::{self, plan_ring};

static ALLOCATED: AtomicU64 = AtomicU64::new(0);

pub fn rings_allocated() -> u64 {
    ALLOCATED.load(Ordering::SeqCst)
}

pub struct CaptureRing {
    buf: Box<[AtomicU8]>,
    capacity: usize,
    write: AtomicUsize,
    read: AtomicUsize,
    lost: AtomicU64,
}

impl CaptureRing {
    pub fn allocate(rate_hz: u64, channels: u64, bytes_per_sample: u64) -> Result<Self, ErrorCode> {
        let plan = plan_ring(limits::limits(), rate_hz, channels, bytes_per_sample)?;
        let bytes = plan
            .capacity_frames
            .checked_mul(bytes_per_sample)
            .ok_or(ErrorCode::InvalidFormat)?;
        let len = usize::try_from(bytes).map_err(|_| ErrorCode::InvalidFormat)?;
        ALLOCATED.fetch_add(1, Ordering::SeqCst);
        let buf = (0..len).map(|_| AtomicU8::new(0)).collect();
        Ok(Self {
            buf,
            capacity: len,
            write: AtomicUsize::new(0),
            read: AtomicUsize::new(0),
            lost: AtomicU64::new(0),
        })
    }

    pub fn push_bytes(&self, input: &[u8]) {
        let cap = self.capacity;
        if cap == 0 || input.is_empty() {
            return;
        }
        let read = self.read.load(Ordering::Acquire);
        let write = self.write.load(Ordering::Relaxed);
        let used = write.wrapping_sub(read);
        if used > cap || input.len() >= cap.saturating_sub(used) {
            self.lost.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let start = write % cap;
        let first = (cap - start).min(input.len());
        for (offset, byte) in input.iter().enumerate() {
            let index = if offset < first {
                start + offset
            } else {
                offset - first
            };
            self.buf[index].store(*byte, Ordering::Relaxed);
        }
        self.write
            .store(write.wrapping_add(input.len()), Ordering::Release);
    }

    pub fn lost(&self) -> u64 {
        self.lost.load(Ordering::Relaxed)
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
