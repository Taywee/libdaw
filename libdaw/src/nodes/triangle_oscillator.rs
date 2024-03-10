use crate::stream::Stream;
use crate::{FrequencyNode, Node, Result};
use std::cell::Cell;

#[derive(Debug)]
pub struct TriangleOscillator {
    frequency: Cell<f64>,
    sample_rate: f64,
    /// Ramps from 0 to 1 per period
    ramp: Cell<f64>,
    delta: Cell<f64>,
    channels: usize,
}

impl TriangleOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        let node = TriangleOscillator {
            frequency: Cell::new(256.0),
            ramp: Default::default(),
            sample_rate: sample_rate as f64,
            delta: Cell::new(0.01),
            channels: channels.into(),
        };
        node.calculate_delta();
        node
    }

    fn calculate_delta(&self) {
        self.delta.set(self.frequency.get() / self.sample_rate);
    }
}

impl FrequencyNode for TriangleOscillator {
    fn set_frequency(&self, frequency: f64) -> Result<()> {
        self.frequency.set(frequency);
        self.calculate_delta();
        Ok(())
    }
    fn get_frequency(&self) -> Result<f64> {
        Ok(self.frequency.get())
    }
}

impl Node for TriangleOscillator {
    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) -> Result<()> {
        let ramp = self
            .ramp
            .replace((self.ramp.get() + self.delta.get()) % 1.0f64);
        // Builds this pattern:
        // /\
        //   \/
        let sample = (((ramp - 0.25).abs() - 0.5).abs() - 0.25) * 4.0;
        let mut output = Stream::new(self.channels);
        output.fill(sample);
        outputs.push(output);
        Ok(())
    }
}
