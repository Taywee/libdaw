use crate::{Node, Result, Sample};

/// Detunes the input frequency by the amount given.  If no input comes in, just
/// passes out the detune multiplier.
#[derive(Debug, Default)]
pub struct Detune {
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
        let input = inputs
            .get(0)
            .and_then(|input| input.get(0).cloned())
            .unwrap_or(1.0);
        let detune_pow2 = 2.0f64.powf(self.detune);
        outputs.push(Sample {
            channels: vec![input * detune_pow2],
        });
        Ok(())
    }
}
