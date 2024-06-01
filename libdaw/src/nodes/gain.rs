use crate::{sample::Sample, Node, Result};

#[derive(Debug)]
pub struct Gain {
    pub gain: f64,
}

impl Gain {
    pub fn new(gain: f64) -> Self {
        Self { gain }
    }
}

impl Node for Gain {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        for input in inputs {
            outputs.push(input * self.gain);
        }
        Ok(())
    }
}
