use crate::{sample::Sample, Node, Result};
use std::collections::VecDeque;

/// Simple averaging low pass filter.  Keeps a buffer of the length of the
/// passed-in frequency and averages that buffer for each new input sample.
#[derive(Debug)]
pub struct LowPassFilter {
    buffer_size: usize,
    buffers: Vec<VecDeque<Sample>>,

    /// Running averages
    averages: Vec<Sample>,
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
            averages: Vec::new(),
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
        self.averages.resize_with(inputs.len(), Default::default);

        // Calculates a rolling average, including eviction of previous values,
        // so the entire average doesn't have to be calculated every time.
        for ((buffer, average), sample) in
            self.buffers.iter_mut().zip(&mut self.averages).zip(inputs)
        {
            while buffer.len() >= buffer_size {
                let prev_len = buffer.len() as f64;
                let evicted = buffer.pop_front().unwrap();
                // Remove average influence of the evicted sample
                *average = (&*average * prev_len - evicted) / (prev_len - 1.0);
            }
            let prev_len = buffer.len() as f64;
            buffer.push_back(sample.clone());
            *average = (&*average * prev_len + sample) / (prev_len + 1.0);
            outputs.push(average.clone());
        }
        Ok(())
    }
}
