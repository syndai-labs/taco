//! Default-input spike via CPAL on macOS.
//!
//! The closure copies bytes into a preallocated ring. It does not allocate, log,
//! resample, run inference, wait, or enter an async runtime. CPAL's own host glue
//! sits outside that closure and is not claimed to be allocation-free.
//!
//! The session path does not call this. Other hosts fail closed.
//! On macOS the CPAL open runs on a worker. The caller waits at most
//! `TACO_CAPTURE_SPIKE_OPEN_MS`. `cpal::Stream` is not `Send`, so the stream
//! stays on that worker. This does not press Allow or Deny.

use std::sync::Arc;

#[cfg(target_os = "macos")]
use std::sync::mpsc;
#[cfg(target_os = "macos")]
use std::thread;
#[cfg(target_os = "macos")]
use std::time::Duration;

#[cfg(target_os = "macos")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use super::{note_backend_call, CaptureRing};

pub struct MicrophoneSession {
    _ring: Arc<CaptureRing>,
    #[cfg(target_os = "macos")]
    stop: Option<mpsc::Sender<()>>,
    #[cfg(target_os = "macos")]
    worker: Option<thread::JoinHandle<()>>,
}

#[cfg(target_os = "macos")]
impl Drop for MicrophoneSession {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

/// Milliseconds the caller waits for the CPAL open.
/// Unset or not a `u64` uses `180_000` (3 minutes).
#[cfg(target_os = "macos")]
fn open_timeout_ms() -> u64 {
    std::env::var("TACO_CAPTURE_SPIKE_OPEN_MS")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(180_000)
}

pub fn open_default_microphone(ring: Arc<CaptureRing>) -> Result<MicrophoneSession, ()> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (ring, note_backend_call as fn());
        return Err(());
    }
    #[cfg(target_os = "macos")]
    {
        let kept = Arc::clone(&ring);
        let (opened_tx, opened_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel::<()>();
        let worker = thread::spawn(move || {
            let stream = match open_stream(ring) {
                Ok(stream) => stream,
                Err(()) => {
                    let _ = opened_tx.send(false);
                    return;
                }
            };
            if opened_tx.send(true).is_err() {
                drop(stream);
                return;
            }
            let _ = stop_rx.recv();
            drop(stream);
        });
        match opened_rx.recv_timeout(Duration::from_millis(open_timeout_ms())) {
            Ok(true) => Ok(MicrophoneSession {
                _ring: kept,
                stop: Some(stop_tx),
                worker: Some(worker),
            }),
            Ok(false) | Err(mpsc::RecvTimeoutError::Disconnected) => {
                let _ = worker.join();
                Err(())
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Detach. Joining would wait out the CoreAudio block. Dropping
                // the reply channel makes a late open drop the stream.
                drop(opened_rx);
                drop(stop_tx);
                drop(worker);
                Err(())
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn open_stream(ring: Arc<CaptureRing>) -> Result<cpal::Stream, ()> {
    note_backend_call();
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or(())?;
    let supported = device.default_input_config().map_err(|_| ())?;
    let format = supported.sample_format();
    let config: cpal::StreamConfig = supported.into();
    let callback_ring = Arc::clone(&ring);
    let stream = device
        .build_input_stream_raw(
            &config,
            format,
            move |data, _info| {
                callback_ring.push_bytes(data.bytes());
            },
            |_err| {},
            None,
        )
        .map_err(|_| ())?;
    stream.play().map_err(|_| ())?;
    Ok(stream)
}
