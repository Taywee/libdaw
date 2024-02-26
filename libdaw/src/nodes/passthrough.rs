use crate::{stream::Stream, Node};
use std::cell::Cell;

/// Copies all its inputs to outputs.  This is mostly a utility node to make
/// some patterns easier to implement.
#[derive(Debug, Default)]
pub struct Passthrough {
    sample_rate: Cell<u32>,
    channels: Cell<u16>,
}

impl Node for Passthrough {
    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        outputs.extend_from_slice(inputs);
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
