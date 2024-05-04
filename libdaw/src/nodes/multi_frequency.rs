use crate::{stream::Stream, sync::AtomicF64, FrequencyNode, Node, Result};
use std::sync::{atomic::Ordering, Arc};

/// A FrequencyNode that wraps any number of other frequency nodes
#[derive(Debug)]
pub struct MultiFrequency {
    nodes: Box<[Arc<dyn FrequencyNode>]>,
    frequency: AtomicF64,
}

impl MultiFrequency {
    pub fn new(nodes: impl IntoIterator<Item = Arc<dyn FrequencyNode>>) -> Self {
        Self {
            frequency: AtomicF64::new(256.0),
            nodes: nodes.into_iter().collect(),
        }
    }
}

impl FrequencyNode for MultiFrequency {
    fn set_frequency(&self, frequency: f64) -> Result<()> {
        self.frequency.store(frequency, Ordering::Relaxed);
        for node in self.nodes.iter() {
            node.set_frequency(frequency)?;
        }
        Ok(())
    }
    fn get_frequency(&self) -> Result<f64> {
        Ok(self.frequency.load(Ordering::Relaxed))
    }
}

impl Node for MultiFrequency {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [Stream],
        outputs: &'c mut Vec<Stream>,
    ) -> Result<()> {
        for node in self.nodes.iter() {
            node.process(inputs, outputs)?;
        }
        Ok(())
    }
}
