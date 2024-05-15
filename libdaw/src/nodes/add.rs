use crate::{stream::Stream, Node, Result};

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
        let mut output: Stream = inputs.iter().sum();
        output.resize(self.channels, 0.0);
        outputs.push(output);
        Ok(())
    }
}
