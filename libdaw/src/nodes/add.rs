use crate::{sample::Sample, Node, Result};

#[derive(Debug, Default)]
pub struct Add {
    _private: (),
}

impl Node for Add {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        outputs.push(inputs.into_iter().sum());
        Ok(())
    }
}
