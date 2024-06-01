use crate::{sample::Sample, Node, Result};

#[derive(Debug)]
pub struct SawtoothOscillator {
    sample_rate: f64,
    sample: f64,
    channels: usize,
}

impl SawtoothOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        SawtoothOscillator {
            sample: Default::default(),
            sample_rate: sample_rate as f64,
            channels: channels.into(),
        }
    }
}

impl Node for SawtoothOscillator {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        let frequency = inputs
            .get(0)
            .and_then(|input| input.get(0).cloned())
            .unwrap_or(0.0);
        // Multiply by 2.0 because the samples vary from -1.0 to 1.0, which is a
        // 2.0 range.
        let delta = frequency * 2.0 / self.sample_rate;
        let sample = self.sample;
        self.sample = (self.sample + delta + 1.0f64) % 2.0f64 - 1.0f64;

        let mut output = Sample::zeroed(self.channels);
        output.fill(sample);
        outputs.push(output);
        Ok(())
    }
}
