use crate::stream::Stream;
use crate::Node;
use std::cell::Cell;
use std::ops::Add as _;

#[derive(Debug, Default)]
pub struct Add {
    sample_rate: Cell<u32>,
    channels: Cell<u16>,
}

impl Node for Add {
    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        outputs.push(
            inputs
                .into_iter()
                .copied()
                .reduce(Stream::add)
                .unwrap_or_else(|| Stream::new(self.channels.get().into())),
        );
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
