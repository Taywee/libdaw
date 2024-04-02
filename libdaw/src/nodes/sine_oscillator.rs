use crate::{stream::Stream, sync::AtomicF64, FrequencyNode, Node, Result};
use std::{f64, sync::atomic::Ordering};

#[derive(Debug)]
pub struct SineOscillator {
    frequency: AtomicF64,
    sample_rate: f64,
    /// Ramps from 0 to TAU per period
    ramp: AtomicF64,
    delta: AtomicF64,
    channels: usize,
}

impl SineOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        let node = SineOscillator {
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
            self.frequency.load(Ordering::Relaxed) * f64::consts::TAU / self.sample_rate,
            Ordering::Relaxed,
        );
    }
}

impl FrequencyNode for SineOscillator {
    fn set_frequency(&self, frequency: f64) -> Result<()> {
        self.frequency.store(frequency, Ordering::Relaxed);
        self.calculate_delta();
        Ok(())
    }
    fn get_frequency(&self) -> Result<f64> {
        Ok(self.frequency.load(Ordering::Relaxed))
    }
}

impl Node for SineOscillator {
    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) -> Result<()> {
        let ramp = self.ramp.swap(
            (self.ramp.load(Ordering::Relaxed) + self.delta.load(Ordering::Relaxed))
                % f64::consts::TAU,
            Ordering::Relaxed,
        );
        let mut output = Stream::new(self.channels);
        output.fill(ramp.sin());
        outputs.push(output);
        Ok(())
    }
}
