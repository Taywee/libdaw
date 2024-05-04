use crate::stream::Stream;
use crate::sync::AtomicF64;
use crate::{FrequencyNode, Node, Result};

use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct TriangleOscillator {
    frequency: AtomicF64,
    sample_rate: f64,
    /// Ramps from 0 to 1 per period
    ramp: AtomicF64,
    delta: AtomicF64,
    channels: usize,
}

impl TriangleOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        let node = TriangleOscillator {
            frequency: AtomicF64::new(256.0),
            ramp: Default::default(),
            sample_rate: sample_rate as f64,
            delta: AtomicF64::new(0.01),
            channels: channels.into(),
        };
        node.calculate_delta();
        node
    }

    fn calculate_delta(&self) {
        self.delta.store(
            self.frequency.load(Ordering::Relaxed) / self.sample_rate,
            Ordering::Relaxed,
        );
    }
}

impl FrequencyNode for TriangleOscillator {
    fn set_frequency(&self, frequency: f64) -> Result<()> {
        self.frequency.store(frequency, Ordering::Relaxed);
        self.calculate_delta();
        Ok(())
    }
    fn get_frequency(&self) -> Result<f64> {
        Ok(self.frequency.load(Ordering::Relaxed))
    }
}

impl Node for TriangleOscillator {
    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) -> Result<()> {
        let ramp = self.ramp.swap(
            (self.ramp.load(Ordering::Relaxed) + self.delta.load(Ordering::Relaxed)) % 1.0f64,
            Ordering::Relaxed,
        );
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
