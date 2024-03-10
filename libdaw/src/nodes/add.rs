use crate::stream::Stream;
use crate::{Node, Result};
use std::ops::Add as _;

#[derive(Debug)]
pub struct Add {
    channels: usize,
}

impl Add {
    pub fn new(channels: u16) -> Self {
        Add {
            channels: channels.into(),
        }
    }
}

impl Node for Add {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [Stream],
        outputs: &'c mut Vec<Stream>,
    ) -> Result<()> {
        outputs.push(
            inputs
                .into_iter()
                .copied()
                .reduce(Stream::add)
                .unwrap_or_else(|| Stream::new(self.channels)),
        );
        Ok(())
    }
}
