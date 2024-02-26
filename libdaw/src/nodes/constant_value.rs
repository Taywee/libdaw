use crate::stream::Stream;
use crate::Node;
use std::cell::Cell;

#[derive(Debug, Default)]
pub struct ConstantValue {
    value: Cell<f64>,
    sample_rate: Cell<u32>,
    channels: Cell<u16>,
}

impl ConstantValue {
    pub fn new(value: f64) -> Self {
        Self {
            value: value.into(),
            channels: Default::default(),
            sample_rate: Default::default(),
        }
    }
    pub fn get_value(&self) -> f64 {
        self.value.get()
    }
    pub fn set_value(&self, value: f64) {
        self.value.set(value);
    }
}

impl Node for ConstantValue {
    fn process<'a, 'b>(&'a self, _: &'b [Stream], outputs: &'a mut Vec<Stream>) {
        let mut stream = Stream::new(self.channels.get().into());
        stream.fill(self.value.get());
        outputs.push(stream);
    }

    fn set_sample_rate(&self, sample_rate: u32) {
        self.sample_rate.set(sample_rate);
    }

    fn set_channels(&self, channels: u16) {
        self.channels.set(channels);
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate.get()
    }

    fn get_channels(&self) -> u16 {
        self.channels.get()
    }

    fn node(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn Node> {
        self
    }
}
