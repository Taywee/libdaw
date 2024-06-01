use crate::{sample::Sample, time::Duration, Node, Result};
use std::collections::VecDeque;

#[derive(Debug)]
struct DelaySample {
    play_sample: u64,
    stream: Sample,
}

type Buffer = VecDeque<DelaySample>;

#[derive(Debug)]
pub struct Delay {
    buffers: Vec<Buffer>,
    sample: u64,
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
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        if self.delay == 0 {
            outputs.extend_from_slice(inputs);
            return Ok(());
        }

        let sample = self.sample;
        self.sample += 1;

        let play_sample = sample + self.delay;

        if inputs.len() > self.buffers.len() {
            let delay = self.delay as usize;
            self.buffers
                .resize_with(inputs.len(), || VecDeque::with_capacity(delay));
        }

        for (i, buffer) in self.buffers.iter_mut().enumerate() {
            let play = buffer
                .front()
                .map(|buffer_sample| sample >= buffer_sample.play_sample)
                .unwrap_or(false);
            if play {
                outputs.push(buffer.pop_front().expect("buffer will not be empty").stream);
            }
            if let Some(stream) = inputs.get(i).cloned() {
                buffer.push_back(DelaySample {
                    play_sample,
                    stream,
                });
            }
        }
        Ok(())
    }
}
