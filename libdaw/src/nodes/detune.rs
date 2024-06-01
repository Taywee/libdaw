use crate::{Node, Result, Sample};

/// Turns the detune that's either passed in as an input or set on this node to
/// be a multiplier to adjust a frequency by that many number of octaves.
#[derive(Debug, Default)]
pub struct Detune {
    /// The amount to detune if no input comes in.
    pub detune: f64,
}

impl Detune {
    pub fn new() -> Self {
        Self {
            detune: Default::default(),
        }
    }
}

impl Node for Detune {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [crate::sample::Sample],
        outputs: &'c mut Vec<crate::sample::Sample>,
    ) -> Result<()> {
        let detune = inputs
            .get(0)
            .and_then(|input| input.get(0).cloned())
            .unwrap_or(self.detune);
        let detune_pow2 = 2.0f64.powf(detune);
        outputs.push(Sample {
            channels: vec![detune_pow2],
        });
        Ok(())
    }
}
