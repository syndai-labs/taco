//! Default-input spike via CPAL on macOS.
//!
//! The closure copies bytes into a preallocated ring. It does not allocate, log,
//! resample, run inference, wait, or enter an async runtime. CPAL's own host glue
//! sits outside that closure and is not claimed to be allocation-free.
//!
//! The session path does not call this. Other hosts fail closed.

use std::sync::Arc;

#[cfg(target_os = "macos")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use super::{note_backend_call, CaptureRing};

pub struct MicrophoneSession {
    _ring: Arc<CaptureRing>,
    #[cfg(target_os = "macos")]
    _stream: cpal::Stream,
}

pub fn open_default_microphone(ring: Arc<CaptureRing>) -> Result<MicrophoneSession, ()> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (ring, note_backend_call as fn());
        return Err(());
    }
    #[cfg(target_os = "macos")]
    {
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
        Ok(MicrophoneSession {
            _stream: stream,
            _ring: ring,
        })
    }
}
