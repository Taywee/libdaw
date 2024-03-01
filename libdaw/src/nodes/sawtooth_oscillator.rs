use crate::stream::Stream;
use crate::{FrequencyNode, Node};
use std::cell::Cell;

#[derive(Debug)]
pub struct SawtoothOscillator {
    frequency: Cell<f64>,
    sample_rate: Cell<f64>,
    sample: Cell<f64>,
    delta: Cell<f64>,
    channels: Cell<u16>,
}

impl SawtoothOscillator {
    fn calculate_delta(&self) {
        self.delta
            .set(self.frequency.get() * 2.0 / self.sample_rate.get());
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

    fn frequency_node(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn FrequencyNode> {
        self
    }
}

impl Default for SawtoothOscillator {
    fn default() -> Self {
        let node = SawtoothOscillator {
            frequency: Cell::new(256.0),
            sample: Default::default(),
            sample_rate: Cell::new(48000.0),
            delta: Cell::new(0.01),
            channels: Default::default(),
        };
        node.calculate_delta();
        node
    }
}

impl Node for SawtoothOscillator {
    fn set_sample_rate(&self, sample_rate: u32) {
        self.sample_rate.set(sample_rate.into());
        self.calculate_delta();
    }
    fn get_sample_rate(&self) -> u32 {
        self.sample_rate.get() as u32
    }

    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        let mut output = Stream::new(self.channels.get().into());
        output.fill(self.sample.get());
        outputs.push(output);

        let mut sample = self.sample.get() + self.delta.get();
        if sample > 1.0 {
            // Range is [-1.0, 1.0]
            sample -= 2.0;
        }
        self.sample.set(sample);
    }

    fn set_channels(&self, channels: u16) {
        self.channels.set(channels);
    }

    fn get_channels(&self) -> u16 {
        self.channels.get()
    }

    fn node(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn Node> {
        self
    }
}
