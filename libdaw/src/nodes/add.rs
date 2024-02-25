use crate::stream::Stream;
use crate::Node;
use std::ops::Add as _;

#[derive(Debug, Default)]
pub struct Add {
    sample_rate: u32,
    channels: u16,
}

impl Node for Add {
    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    fn process<'a, 'b>(&'a mut self, inputs: &'b [Stream], outputs: &'a mut Vec<Stream>) {
        outputs.push(
            inputs
                .into_iter()
                .copied()
                .reduce(Stream::add)
                .unwrap_or_else(|| Stream::new(self.channels.into())),
        );
    }

    fn set_channels(&mut self, channels: u16) {
        self.channels = channels;
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn get_channels(&self) -> u16 {
        self.channels
    }
}