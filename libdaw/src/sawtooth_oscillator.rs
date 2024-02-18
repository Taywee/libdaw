use super::Node;
use crate::streams::{Channels, Streams};
use smallvec::smallvec;

#[derive(Debug)]
pub struct SawtoothOscillator {
    frequency: f64,
    sample_rate: f64,
    sample: f64,
    delta: f64,
}

impl SawtoothOscillator {
    fn calculate_delta(&mut self) {
        self.delta = self.frequency * 2.0 / self.sample_rate;
    }
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
        self.calculate_delta();
    }
    pub fn get_frequency(&self) -> f64 {
        self.frequency
    }
}

impl Default for SawtoothOscillator {
    fn default() -> Self {
        let mut node = SawtoothOscillator {
            frequency: 256.0,
            sample: -1.0,
            sample_rate: 48000.0,
            delta: 0.01,
        };
        node.calculate_delta();
        node
    }
}

impl Node for SawtoothOscillator {
    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate.into();
        self.calculate_delta();
    }

    fn process(&mut self, _: Streams) -> Streams {
        let output = Streams(smallvec![Channels(smallvec![self.sample])]);
        self.sample += self.delta;
        while self.sample > 1.0 {
            self.sample -= 1.0;
        }

        output
    }
}
