use crate::{sample::Sample, Node, Result};
use std::collections::VecDeque;

/// Simple averaging low pass filter.  Keeps a buffer of the length of the
/// passed-in frequency and averages that buffer for each new input sample.
#[derive(Debug)]
pub struct LowPassFilter {
    buffer_size: usize,
    buffers: Vec<VecDeque<Sample>>,
}
impl LowPassFilter {
    pub fn new(sample_rate: u32, frequency: f64) -> Result<Self> {
        let buffer_size = sample_rate as f64 / frequency;
        if !(buffer_size >= 0.0) {
            return Err("frequency must be non-negative".into());
        }
        let buffer_size = buffer_size as usize;
        Ok(Self {
            buffer_size,
            buffers: Vec::new(),
        })
    }
}

impl Node for LowPassFilter {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        let buffer_size = self.buffer_size;
        if buffer_size <= 1 {
            outputs.extend_from_slice(inputs);
            return Ok(());
        }
        self.buffers
            .resize_with(inputs.len(), move || VecDeque::with_capacity(buffer_size));

        for (buffer, sample) in self.buffers.iter_mut().zip(inputs) {
            while buffer.len() >= buffer_size {
                buffer.pop_front();
            }
            buffer.push_back(sample.clone());
            let sum: Sample = buffer.iter().sum();
            outputs.push(sum / buffer.len() as f64);
        }
        Ok(())
    }
}
