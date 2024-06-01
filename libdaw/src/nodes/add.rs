use crate::{sample::Sample, Node, Result};

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
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        let mut output: Sample = inputs.iter().sum();
        output.channels.resize(self.channels, 0.0);
        outputs.push(output);
        Ok(())
    }
}
