use crate::sample::Sample;
use crate::{Node, Result};

#[derive(Debug, Default)]
pub struct ConstantValue {
    pub value: f64,
    channels: usize,
}

impl ConstantValue {
    pub fn new(channels: u16, value: f64) -> Self {
        Self {
            value,
            channels: channels.into(),
        }
    }
}

impl Node for ConstantValue {
    fn process<'a, 'b>(&'a mut self, _: &'b [Sample], outputs: &'a mut Vec<Sample>) -> Result<()> {
        let mut stream = Sample::zeroed(self.channels);
        stream.fill(self.value);
        outputs.push(stream);
        Ok(())
    }
}
