use crate::{sample::Sample, Node, Result};
use std::f64;

#[derive(Debug)]
pub struct SineOscillator {
    sample_rate: f64,
    /// Ramps from 0 to TAU per period
    ramp: f64,
    channels: usize,
}

impl SineOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        SineOscillator {
            ramp: Default::default(),
            sample_rate: sample_rate as f64,
            channels: channels.into(),
        }
    }
}

impl Node for SineOscillator {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        let frequency = inputs
            .get(0)
            .and_then(|input| input.get(0).cloned())
            .unwrap_or(0.0);
        let delta = frequency * f64::consts::TAU / self.sample_rate;
        let mut output = Sample::zeroed(self.channels);
        output.fill(self.ramp.sin());
        outputs.push(output);

        self.ramp = (self.ramp + delta) % f64::consts::TAU;
        Ok(())
    }
}
