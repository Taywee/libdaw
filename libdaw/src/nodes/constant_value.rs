use crate::stream::Stream;
use crate::Node;
use std::cell::Cell;

#[derive(Debug, Default)]
pub struct ConstantValue {
    value: Cell<f64>,
    channels: usize,
}

impl ConstantValue {
    pub fn new(value: f64, channels: u16) -> Self {
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
    fn process<'a, 'b>(&'a self, _: &'b [Stream], outputs: &'a mut Vec<Stream>) {
        let mut stream = Stream::new(self.channels);
        stream.fill(self.value.get());
        outputs.push(stream);
    }

    fn node(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn Node> {
        self
    }
}
