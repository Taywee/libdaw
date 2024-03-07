use crate::stream::Stream;
use crate::{FrequencyNode, Node};
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
        self.delta
            .set(self.frequency.get() * 2.0 / self.sample_rate);
    }
}

impl FrequencyNode for SawtoothOscillator {
    fn set_frequency(&self, frequency: f64) {
        self.frequency.set(frequency);
        self.calculate_delta();
    }
    fn get_frequency(&self) -> f64 {
        self.frequency.get()
    }
}

impl Node for SawtoothOscillator {
    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        let mut output = Stream::new(self.channels);
        output.fill(self.sample.get());
        outputs.push(output);

        let mut sample = self.sample.get() + self.delta.get();
        if sample > 1.0 {
            // Range is [-1.0, 1.0]
            sample -= 2.0;
        }
        self.sample.set(sample);
    }
}
