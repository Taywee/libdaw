use crate::{stream::Stream, time::Duration, Node, Result};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

#[derive(Debug)]
struct Sample {
    play_sample: u64,
    stream: Stream,
}

type Buffer = VecDeque<Sample>;

#[derive(Debug)]
pub struct Delay {
    buffers: Mutex<Vec<Buffer>>,
    sample: AtomicU64,
    delay: u64,
}

impl Delay {
    pub fn new(sample_rate: u32, delay: Duration) -> Self {
        let delay = (delay.seconds() * sample_rate as f64) as u64;
        Self {
            buffers: Default::default(),
            sample: Default::default(),
            delay,
        }
    }
}

impl Node for Delay {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [Stream],
        outputs: &'c mut Vec<Stream>,
    ) -> Result<()> {
        if self.delay == 0 {
            outputs.extend_from_slice(inputs);
            return Ok(());
        }
        let sample = self
            .sample
            .swap(self.sample.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
        let play_sample = sample + self.delay;

        let mut buffers = self.buffers.lock().expect("mutex poisoned");
        if inputs.len() > buffers.len() {
            let delay = self.delay as usize;
            buffers.resize_with(inputs.len(), || VecDeque::with_capacity(delay));
        }

        outputs.reserve(buffers.len());
        for (i, buffer) in buffers.iter_mut().enumerate() {
            let play = buffer
                .front()
                .map(|buffer_sample| sample >= buffer_sample.play_sample)
                .unwrap_or(false);
            if play {
                outputs.push(buffer.pop_front().expect("buffer will not be empty").stream);
            }
            if let Some(stream) = inputs.get(i).cloned() {
                buffer.push_back(Sample {
                    play_sample,
                    stream,
                });
            }
        }
        Ok(())
    }
}
