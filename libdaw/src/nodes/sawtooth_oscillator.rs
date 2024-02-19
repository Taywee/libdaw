use crate::stream::Stream;
use crate::Node;

#[derive(Debug)]
pub struct SawtoothOscillator {
    frequency: f64,
    sample_rate: f64,
    sample: f64,
    delta: f64,
    channels: u16,
}

impl SawtoothOscillator {
    fn calculate_delta(&mut self) {
        self.delta = self.frequency * 2.0 / self.sample_rate;
    }
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
        self.calculate_delta();
    }
    pub fn get_frequency(&self) -> f64 {
        self.frequency
    }
}

impl Default for SawtoothOscillator {
    fn default() -> Self {
        let mut node = SawtoothOscillator {
            frequency: 256.0,
            sample: -1.0,
            sample_rate: 48000.0,
            delta: 0.01,
            channels: Default::default(),
        };
        node.calculate_delta();
        node
    }
}

impl Node for SawtoothOscillator {
    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate.into();
        self.calculate_delta();
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate as u32
    }

    fn process<'a, 'b>(&'a mut self, _: &'b [Stream], outputs: &'a mut Vec<Stream>) {
        let mut output = Stream::new(self.channels.into());
        output.fill(self.sample);
        outputs.push(output);

        self.sample += self.delta;
        while self.sample > 1.0 {
            self.sample -= 1.0;
        }
    }

    fn set_channels(&mut self, channels: u16) {
        self.channels = channels;
    }

    fn get_channels(&self) -> u16 {
        self.channels
    }
}
