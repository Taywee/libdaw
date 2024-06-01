use crate::{sample::Sample, Node, Result};

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
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        let mut output: Sample = inputs.iter().product();
        output.channels.resize(self.channels, 0.0);
        outputs.push(output);
        Ok(())
    }
}
