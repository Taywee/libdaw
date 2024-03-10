use crate::{stream::Stream, Node, Result};

/// Copies all its inputs to outputs.  This is mostly a utility node to make
/// some patterns easier to implement.
#[derive(Debug, Default)]
pub struct Passthrough {
    _private: (),
}

impl Node for Passthrough {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [Stream],
        outputs: &'c mut Vec<Stream>,
    ) -> Result<()> {
        outputs.extend_from_slice(inputs);
        Ok(())
    }
}
