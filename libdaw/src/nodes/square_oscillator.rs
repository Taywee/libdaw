use crate::streams::{Channels, Streams};
use crate::Node;
use smallvec::smallvec;

#[derive(Debug)]
pub struct SquareOscillator {
    frequency: f64,
    samples_per_switch: f64,
    samples_since_switch: f64,
    sample_rate: f64,
    sample: f64,
}

impl SquareOscillator {
    fn calculate_samples_per_switch(&mut self) {
        let switches_per_second = self.frequency * 2.0;
        self.samples_per_switch = self.sample_rate / switches_per_second;
    }
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
        self.calculate_samples_per_switch();
    }
    pub fn get_frequency(&self) -> f64 {
        self.frequency
    }
}

impl Default for SquareOscillator {
    fn default() -> Self {
        let mut node = SquareOscillator {
            frequency: 256.0,
            samples_since_switch: 0.0,
            sample: 1.0,
            samples_per_switch: 100000.0,
            sample_rate: 48000.0,
        };
        node.calculate_samples_per_switch();
        node
    }
}

impl Node for SquareOscillator {
    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate.into();
        self.calculate_samples_per_switch();
    }

    fn process(&mut self, _: Streams) -> Streams {
        let output = Streams(smallvec![Channels(smallvec![self.sample])]);
        while self.samples_since_switch >= self.samples_per_switch {
            self.samples_since_switch -= self.samples_per_switch;
            self.sample *= -1.0;
        }
        self.samples_since_switch += 1.0;
        output
    }
}
