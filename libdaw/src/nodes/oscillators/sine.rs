use crate::{sample::Sample, Node, Result};
use std::f64;

#[derive(Debug)]
pub struct Sine {
    /// The frequency if no input comes in.
    pub frequency: f64,

    sample_rate: f64,
    /// Ramps from 0 to TAU per period
    ramp: f64,
}

impl Sine {
    pub fn new(sample_rate: u32, frequency: f64) -> Self {
        Sine {
            frequency,
            ramp: Default::default(),
            sample_rate: sample_rate as f64,
        }
    }
}

impl Node for Sine {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        let frequency = inputs
            .get(0)
            .and_then(|input| input.get(0).cloned())
            .unwrap_or(self.frequency);
        let delta = frequency / self.sample_rate;
        outputs.push((self.ramp * f64::consts::TAU).sin().into());
        self.ramp = (self.ramp + delta) % 1.0;
        Ok(())
    }
}
