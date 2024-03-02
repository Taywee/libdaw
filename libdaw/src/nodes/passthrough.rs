use crate::{stream::Stream, Node};
use std::cell::Cell;

/// Copies all its inputs to outputs.  This is mostly a utility node to make
/// some patterns easier to implement.
#[derive(Debug, Default)]
pub struct Passthrough {
    _private: (),
}

impl Node for Passthrough {
    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        outputs.extend_from_slice(inputs);
    }

    fn node(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn Node> {
        self
    }
}
