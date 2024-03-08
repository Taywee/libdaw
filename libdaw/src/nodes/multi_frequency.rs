use crate::stream::Stream;
use crate::{FrequencyNode, Node};
use std::cell::Cell;
use std::rc::Rc;

/// A FrequencyNode that wraps any number of other frequency nodes
#[derive(Debug)]
pub struct MultiFrequency {
    nodes: Box<[Rc<dyn FrequencyNode>]>,
    frequency: Cell<f64>,
}

impl MultiFrequency {
    pub fn new(nodes: impl IntoIterator<Item = Rc<dyn FrequencyNode>>) -> Self {
        Self {
            frequency: Cell::new(256.0),
            nodes: nodes.into_iter().collect(),
        }
    }
}

impl FrequencyNode for MultiFrequency {
    fn set_frequency(&self, frequency: f64) {
        self.frequency.set(frequency);
        for node in self.nodes.iter() {
            node.set_frequency(frequency);
        }
    }
    fn get_frequency(&self) -> f64 {
        self.frequency.get()
    }
}

impl Node for MultiFrequency {
    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        for node in self.nodes.iter() {
            node.process(inputs, outputs);
        }
    }
}
