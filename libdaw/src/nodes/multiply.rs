use crate::{stream::Stream, Node, Result};

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
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [Stream],
        outputs: &'c mut Vec<Stream>,
    ) -> Result<()> {
        let mut output: Stream = inputs.iter().product();
        output.resize(self.channels, 0.0);
        outputs.push(output);
        Ok(())
    }
}
