use crate::stream::Stream;
use crate::{FrequencyNode, Node};
use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct SquareOscillator {
    frequency: Cell<f64>,
    samples_per_switch: Cell<f64>,
    samples_since_switch: Cell<f64>,
    sample_rate: Cell<f64>,
    sample: Cell<f64>,
    channels: Cell<u16>,
}

impl SquareOscillator {
    fn calculate_samples_per_switch(&self) {
        let switches_per_second = self.frequency.get() * 2.0;
        self.samples_per_switch
            .set(self.sample_rate.get() / switches_per_second);
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

    fn frequency_node(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn FrequencyNode> {
        self
    }
}

impl Default for SquareOscillator {
    fn default() -> Self {
        let node = Self {
            frequency: Cell::new(256.0),
            samples_since_switch: Default::default(),
            sample: Cell::new(1.0),
            samples_per_switch: Cell::new(100000.0),
            sample_rate: Cell::new(48000.0),
            channels: Default::default(),
        };
        node.calculate_samples_per_switch();
        node
    }
}

impl Node for SquareOscillator {
    fn set_sample_rate(&self, sample_rate: u32) {
        self.sample_rate.set(sample_rate.into());
        self.calculate_samples_per_switch();
    }
    fn get_sample_rate(&self) -> u32 {
        self.sample_rate.get() as u32
    }

    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        let mut output = Stream::new(self.channels.get().into());
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
