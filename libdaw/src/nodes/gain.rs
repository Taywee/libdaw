use crate::stream::Stream;
use crate::Node;
use std::cell::Cell;


#[derive(Debug)]
pub struct Gain {
    sample_rate: Cell<u32>,
    channels: Cell<u16>,
    gain: Cell<f64>,
}

impl Gain {
    pub fn new(gain: f64) -> Self {
        Self {
            sample_rate: Default::default(),
            channels: Default::default(),
            gain: Cell::new(gain),
        }
    }

    pub fn set_gain(&self, gain: f64) {
        self.gain.set(gain);
    }

    pub fn get_gain(&self) -> f64 {
        self.gain.get()
    }
}

impl Node for Gain {
    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        outputs.extend_from_slice(inputs);
        let gain = self.gain.get();

        for output in outputs {
            *output *= gain;
        }
    }

    fn set_sample_rate(&self, sample_rate: u32) {
        self.sample_rate.set(sample_rate);
    }

    fn set_channels(&self, channels: u16) {
        self.channels.set(channels);
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate.get()
    }

    fn get_channels(&self) -> u16 {
        self.channels.get()
    }

    fn node(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn Node> {
        self
    }
}
