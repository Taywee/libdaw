use crate::stream::Stream;
use crate::Node;

use std::ops::Mul as _;

#[derive(Debug, Default)]
pub struct Multiply {
    channels: usize,
}

impl Multiply {
    pub fn new(channels: u16) -> Self {
        Multiply {
            channels: channels.into(),
        }
    }
}

impl Node for Multiply {
    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        outputs.push(
            inputs
                .into_iter()
                .copied()
                .reduce(Stream::mul)
                .unwrap_or_else(|| Stream::new(self.channels)),
        );
    }
}
