use crate::stream::Stream;
use crate::{FrequencyNode, Node};
use std::cell::Cell;

#[derive(Debug)]
pub struct SquareOscillator {
    frequency: Cell<f64>,
    samples_per_switch: Cell<f64>,
    samples_since_switch: Cell<f64>,
    sample_rate: f64,
    sample: Cell<f64>,
    channels: usize,
}

impl SquareOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        let node = Self {
            frequency: Cell::new(256.0),
            samples_since_switch: Default::default(),
            sample: Cell::new(1.0),
            samples_per_switch: Default::default(),
            sample_rate: sample_rate as f64,
            channels: channels.into(),
        };
        node.calculate_samples_per_switch();
        node
    }

    fn calculate_samples_per_switch(&self) {
        let switches_per_second = self.frequency.get() * 2.0;
        self.samples_per_switch
            .set(self.sample_rate / switches_per_second);
    }
}

impl FrequencyNode for SquareOscillator {
    fn set_frequency(&self, frequency: f64) {
        self.frequency.set(frequency);
        self.calculate_samples_per_switch();
    }
    fn get_frequency(&self) -> f64 {
        self.frequency.get()
    }
}

impl Node for SquareOscillator {
    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        let mut output = Stream::new(self.channels);
        output.fill(self.sample.get());
        outputs.push(output);

        let mut samples_since_switch = self.samples_since_switch.get();
        let samples_per_switch = self.samples_per_switch.get();
        if samples_since_switch >= samples_per_switch {
            samples_since_switch -= samples_per_switch;
            self.sample.set(self.sample.get() * -1.0);
        }
        self.samples_since_switch.set(samples_since_switch + 1.0);
    }
}
