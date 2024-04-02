use crate::stream::Stream;
use crate::sync::AtomicF64;
use crate::{FrequencyNode, Node, Result};

use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct SquareOscillator {
    frequency: AtomicF64,
    samples_per_switch: AtomicF64,
    samples_since_switch: AtomicF64,
    sample_rate: f64,
    sample: AtomicF64,
    channels: usize,
}

impl SquareOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        let node = Self {
            frequency: AtomicF64::new(256.0),
            samples_since_switch: Default::default(),
            sample: AtomicF64::new(1.0),
            samples_per_switch: Default::default(),
            sample_rate: sample_rate as f64,
            channels: channels.into(),
        };
        node.calculate_samples_per_switch();
        node
    }

    fn calculate_samples_per_switch(&self) {
        let switches_per_second = self.frequency.load(Ordering::Relaxed) * 2.0;
        self.samples_per_switch
            .store(self.sample_rate / switches_per_second, Ordering::Relaxed);
    }
}

impl FrequencyNode for SquareOscillator {
    fn set_frequency(&self, frequency: f64) -> Result<()> {
        self.frequency.store(frequency, Ordering::Relaxed);
        self.calculate_samples_per_switch();
        Ok(())
    }
    fn get_frequency(&self) -> Result<f64> {
        Ok(self.frequency.load(Ordering::Relaxed))
    }
}

impl Node for SquareOscillator {
    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) -> Result<()> {
        let mut output = Stream::new(self.channels);
        let sample = self.sample.load(Ordering::Relaxed);
        output.fill(sample);
        outputs.push(output);

        let mut samples_since_switch = self.samples_since_switch.load(Ordering::Relaxed);
        let samples_per_switch = self.samples_per_switch.load(Ordering::Relaxed);
        if samples_since_switch >= samples_per_switch {
            samples_since_switch -= samples_per_switch;
            self.sample.store(sample * -1.0, Ordering::Relaxed);
        }
        self.samples_since_switch
            .store(samples_since_switch + 1.0, Ordering::Relaxed);
        Ok(())
    }
}
