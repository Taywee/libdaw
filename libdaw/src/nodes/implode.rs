use crate::{sample::Sample, Node, Result};

/// Copies each channels of each inputs into channel of a single output.
#[derive(Debug, Default)]
pub struct Implode {
    _private: (),
}

impl Node for Implode {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        outputs.push(inputs.into_iter().flatten().copied().collect());
        Ok(())
    }
}
