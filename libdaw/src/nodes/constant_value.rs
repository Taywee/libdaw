use crate::sample::Sample;
use crate::{Node, Result};

#[derive(Debug, Default)]
pub struct ConstantValue {
    pub value: f64,
}

impl ConstantValue {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Node for ConstantValue {
    fn process<'a, 'b>(&'a mut self, _: &'b [Sample], outputs: &'a mut Vec<Sample>) -> Result<()> {
        outputs.push(self.value.into());
        Ok(())
    }
}
