extern crate portaudio as pa;
extern crate time;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: usize    = 1024;
const CHANNELS: i32    = 1;

type BufferRef  = Arc<Mutex<VecDeque<[f32; FRAMES]>>>;
type StreamType = pa::Stream<pa::NonBlocking, pa::Input<f32>>;

pub struct AudioSession {
    pa_handle:     pa::PortAudio,
    stream_handle: StreamType,
    pub buffer:    BufferRef,
}

impl AudioSession {
    pub fn new(
        pa_handle: pa::PortAudio,
        stream_handle: StreamType,
        buffer: BufferRef
    ) -> AudioSession {
        AudioSession {
            pa_handle,
            stream_handle,
            buffer,
        }
    }
}

pub fn run() -> Result<AudioSession, pa::Error> {
    let pa = pa::PortAudio::new()?;
    let bufferp: BufferRef = Arc::new(Mutex::new(VecDeque::new()));
    let settings = pa.default_input_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES as u32)?;
    let buffer_ref = bufferp.clone();

    let callback = move |args: pa::InputStreamCallbackArgs<f32>| {
        let pa::InputStreamCallbackArgs { buffer, .. } = args;

        let mut dequeue = buffer_ref.lock().unwrap();
        let mut xdata   = [0f32; FRAMES];

        xdata.clone_from_slice(buffer);
        dequeue.push_back(xdata);

        while dequeue.len() > 1 {
            dequeue.pop_front();
        }

        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    Ok(AudioSession::new(pa, stream, bufferp))
}
