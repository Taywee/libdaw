use crate::stream::Stream;
use crate::{FrequencyNode, Node};
use std::cell::Cell;
use std::f64;

#[derive(Debug)]
pub struct SineOscillator {
    frequency: Cell<f64>,
    sample_rate: f64,
    /// Ramps from 0 to TAU per period
    ramp: Cell<f64>,
    delta: Cell<f64>,
    channels: usize,
}

impl SineOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        let node = SineOscillator {
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
        self.delta
            .set(self.frequency.get() * f64::consts::TAU / self.sample_rate);
    }
}

impl FrequencyNode for SineOscillator {
    fn set_frequency(&self, frequency: f64) {
        self.frequency.set(frequency);
        self.calculate_delta();
    }
    fn get_frequency(&self) -> f64 {
        self.frequency.get()
    }
}

impl Node for SineOscillator {
    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        let ramp = self
            .ramp
            .replace((self.ramp.get() + self.delta.get()) % f64::consts::TAU);
        let mut output = Stream::new(self.channels);
        output.fill(ramp.sin());
        outputs.push(output);
    }
}
