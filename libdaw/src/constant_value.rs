use super::Node;
use crate::streams::{Channels, Streams};
use smallvec::smallvec;

#[derive(Debug, Default)]
pub struct ConstantValue(f64);

impl ConstantValue {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl ConstantValue {
    pub fn get_value(&mut self) -> f64 {
        self.0
    }
    pub fn set_value(&mut self, value: f64) {
        self.0 = value;
    }
}

impl Node for ConstantValue {
    fn set_sample_rate(&mut self, _: u32) {}

    fn process(&mut self, _inputs: Streams) -> Streams {
        Streams(smallvec![Channels(smallvec![self.0])])
    }
}
