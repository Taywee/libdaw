use crate::{sample::Sample, Node, Result};

#[derive(Debug, Default)]
pub struct Multiply {
    _private: (),
}

impl Node for Multiply {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        outputs.push(inputs.iter().product());
        Ok(())
    }
}
