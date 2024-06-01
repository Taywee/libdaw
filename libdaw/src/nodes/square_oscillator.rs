use crate::sample::Sample;
use crate::{Node, Result};

#[derive(Debug)]
pub struct SquareOscillator {
    samples_since_switch: f64,
    sample_rate: f64,
    sample: f64,
    channels: usize,
}

impl SquareOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        Self {
            samples_since_switch: Default::default(),
            sample: 1.0,
            sample_rate: sample_rate as f64,
            channels: channels.into(),
        }
    }
}

impl Node for SquareOscillator {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        let mut output = Sample::zeroed(self.channels);
        output.fill(self.sample);
        outputs.push(output);

        let frequency = inputs
            .get(0)
            .and_then(|input| input.get(0).cloned())
            .unwrap_or(0.0);
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
