use crate::{stream::Stream, sync::AtomicF64, FrequencyNode, Node, Result};
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct SawtoothOscillator {
    frequency: AtomicF64,
    sample_rate: f64,
    sample: AtomicF64,
    delta: AtomicF64,
    channels: usize,
}

impl SawtoothOscillator {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        let node = SawtoothOscillator {
            frequency: AtomicF64::new(256.0),
            sample: Default::default(),
            sample_rate: sample_rate as f64,
            delta: AtomicF64::new(0.01),
            channels: channels.into(),
        };
        node.calculate_delta();
        node
    }

    fn calculate_delta(&self) {
        // Multiply by 2.0 because the samples vary from -1.0 to 1.0, which is a
        // 2.0 range.
        self.delta.store(
            self.frequency.load(Ordering::Relaxed) * 2.0 / self.sample_rate,
            Ordering::Relaxed,
        );
    }
}

impl FrequencyNode for SawtoothOscillator {
    fn set_frequency(&self, frequency: f64) -> Result<()> {
        self.frequency.store(frequency, Ordering::Relaxed);
        self.calculate_delta();
        Ok(())
    }
    fn get_frequency(&self) -> Result<f64> {
        Ok(self.frequency.load(Ordering::Relaxed))
    }
}

impl Node for SawtoothOscillator {
    fn process<'a, 'b, 'c>(&'a self, _: &'b [Stream], outputs: &'c mut Vec<Stream>) -> Result<()> {
        let sample = self.sample.swap(
            (self.sample.load(Ordering::Relaxed) + self.delta.load(Ordering::Relaxed) + 1.0f64)
                % 2.0f64
                - 1.0f64,
            Ordering::Relaxed,
        );

        let mut output = Stream::new(self.channels);
        output.fill(sample);
        outputs.push(output);
        Ok(())
    }
}
