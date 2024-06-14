use crate::{sample::Sample, Node, Result};

#[derive(Debug)]
pub struct Sawtooth {
    /// The frequency if no input comes in.
    pub frequency: f64,

    sample_rate: f64,
    sample: f64,
}

impl Sawtooth {
    pub fn new(sample_rate: u32, frequency: f64) -> Self {
        Sawtooth {
            frequency,
            sample: Default::default(),
            sample_rate: sample_rate as f64,
        }
    }
}

impl Node for Sawtooth {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        let frequency = inputs
            .get(0)
            .and_then(|input| input.get(0).cloned())
            .unwrap_or(self.frequency);
        outputs.push(self.sample.into());
        // Multiply by 2.0 because the samples vary from -1.0 to 1.0, which is a
        // 2.0 range.
        let delta = frequency * 2.0 / self.sample_rate;
        self.sample = (self.sample + delta + 1.0f64) % 2.0f64 - 1.0f64;
        Ok(())
    }
}
