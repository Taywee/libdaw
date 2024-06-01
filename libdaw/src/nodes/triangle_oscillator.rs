use crate::sample::Sample;
use crate::{Node, Result};

#[derive(Debug)]
pub struct TriangleOscillator {
    sample_rate: f64,
    /// Ramps from 0 to 1 per period
    ramp: f64,
    channels: usize,
}

impl TriangleOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        TriangleOscillator {
            ramp: Default::default(),
            sample_rate: sample_rate as f64,
            channels: channels.into(),
        }
    }
}

impl Node for TriangleOscillator {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        let frequency = inputs
            .get(0)
            .and_then(|input| input.get(0).cloned())
            .unwrap_or(0.0);
        let delta = frequency / self.sample_rate;
        let ramp = self.ramp;
        self.ramp = (ramp + delta) % 1.0f64;
        // Builds this pattern:
        // /\
        //   \/
        let sample = (((ramp - 0.25).abs() - 0.5).abs() - 0.25) * 4.0;
        let mut output = Sample::zeroed(self.channels);
        output.fill(sample);
        outputs.push(output);
        Ok(())
    }
}
