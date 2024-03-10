use crate::stream::Stream;
use crate::{Node, Result};
use std::cell::Cell;

#[derive(Debug, Default)]
pub struct ConstantValue {
    value: Cell<f64>,
    channels: usize,
}

impl ConstantValue {
    pub fn new(channels: u16, value: f64) -> Self {
        Self {
            value: value.into(),
            channels: channels.into(),
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
    fn process<'a, 'b>(&'a self, _: &'b [Stream], outputs: &'a mut Vec<Stream>) -> Result<()> {
        let mut stream = Stream::new(self.channels);
        stream.fill(self.value.get());
        outputs.push(stream);
        Ok(())
    }
}
