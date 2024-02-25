use crate::stream::Stream;
use crate::Node;

/// Copies all its inputs to outputs.  This is mostly a utility node to make
/// some patterns easier to implement.
#[derive(Debug, Default)]
pub struct Passthrough {
    sample_rate: u32,
    channels: u16,
}

impl Node for Passthrough {
    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    fn process<'a, 'b>(&'a mut self, inputs: &'b [Stream], outputs: &'a mut Vec<Stream>) {
        outputs.extend_from_slice(inputs);
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
