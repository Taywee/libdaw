use crate::{sample::Sample, Node, Result};

/// Copies each inputs channel into a single-channel output.
#[derive(Debug, Default)]
pub struct Explode {
    _private: (),
}

impl Node for Explode {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        outputs.extend(inputs.into_iter().flatten().copied().map(|channel| {
            let mut vec = Vec::new();
            vec.reserve_exact(1);
            vec.push(channel);
            vec.into()
        }));
        Ok(())
    }
}
