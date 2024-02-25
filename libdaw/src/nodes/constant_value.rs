use crate::stream::Stream;
use crate::Node;

#[derive(Debug, Default)]
pub struct ConstantValue {
    value: f64,
    channels: u16,
    sample_rate: u32,
}

impl ConstantValue {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            channels: Default::default(),
            sample_rate: Default::default(),
        }
    }
}

impl ConstantValue {
    pub fn get_value(&mut self) -> f64 {
        self.value
    }
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}

impl Node for ConstantValue {
    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    fn process<'a, 'b>(&'a mut self, _: &'b [Stream], outputs: &'a mut Vec<Stream>) {
        let mut stream = Stream::new(self.channels.into());
        stream.fill(self.value);
        outputs.push(stream);
    }

    fn set_channels(&mut self, channels: u16) {
        self.channels = channels;
    }
    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn get_channels(&self) -> u16 {
        self.channels
    }
}
