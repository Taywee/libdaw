use crate::sample::Sample;
use crate::{Node, Result};

#[derive(Debug)]
pub struct Square {
    /// The frequency if no input comes in.
    pub frequency: f64,

    samples_since_switch: f64,
    sample_rate: f64,
    sample: f64,
}

impl Square {
    pub fn new(sample_rate: u32, frequency: f64) -> Self {
        Self {
            frequency,
            samples_since_switch: Default::default(),
            sample: 1.0,
            sample_rate: sample_rate as f64,
        }
    }
}

impl Node for Square {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        outputs.push(self.sample.into());

        let frequency = inputs
            .get(0)
            .and_then(|input| input.get(0).cloned())
            .unwrap_or(self.frequency);
        let switches_per_second = frequency * 2.0;
        let samples_per_switch = self.sample_rate / switches_per_second;

        if self.samples_since_switch >= samples_per_switch {
            self.samples_since_switch -= samples_per_switch;
            self.sample = -self.sample;
        }
        self.samples_since_switch += 1.0;
        Ok(())
    }
}
