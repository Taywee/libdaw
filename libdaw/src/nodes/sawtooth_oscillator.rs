use crate::stream::Stream;
use crate::{FrequencyNode, Node, Result};
use std::cell::Cell;

#[derive(Debug)]
pub struct SawtoothOscillator {
    frequency: Cell<f64>,
    sample_rate: f64,
    sample: Cell<f64>,
    delta: Cell<f64>,
    channels: usize,
}

impl SawtoothOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        let node = SawtoothOscillator {
            frequency: Cell::new(256.0),
            sample: Default::default(),
            sample_rate: sample_rate as f64,
            delta: Cell::new(0.01),
            channels: channels.into(),
        };
        node.calculate_delta();
        node
    }

    fn calculate_delta(&self) {
        // Multiply by 2.0 because the samples vary from -1.0 to 1.0, which is a
        // 2.0 range.
        self.delta
            .set(self.frequency.get() * 2.0 / self.sample_rate);
    }
}

impl FrequencyNode for SawtoothOscillator {
    fn set_frequency(&self, frequency: f64) -> Result<()> {
        self.frequency.set(frequency);
        self.calculate_delta();
        Ok(())
    }
    fn get_frequency(&self) -> Result<f64> {
        Ok(self.frequency.get())
    }
}

impl Node for SawtoothOscillator {
    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) -> Result<()> {
        let sample = self
            .sample
            .replace((self.sample.get() + self.delta.get() + 1.0f64) % 2.0f64 - 1.0f64);

        let mut output = Stream::new(self.channels);
        output.fill(sample);
        outputs.push(output);
        Ok(())
    }
}
