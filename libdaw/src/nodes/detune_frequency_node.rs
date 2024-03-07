use crate::{FrequencyNode, Node};
use std::{cell::Cell, rc::Rc};

/// A node that can play a sequence of notes from a frequency node creator.
#[derive(Debug)]
pub struct DetuneFrequencyNode {
    node: Rc<dyn FrequencyNode>,
    frequency: Cell<f64>,
    detune: Cell<f64>,
    detune_pow2: Cell<f64>,
}

impl DetuneFrequencyNode {
    pub fn new(node: Rc<dyn FrequencyNode>) -> Self {
        Self {
            node,
            frequency: Cell::new(256.0),
            detune: Default::default(),
            detune_pow2: Cell::new(1.0),
        }
    }

    /// Set the detune as a number of octaves to shift the note.  In essence,
    /// this is a log2 of the number that will be multiplied by the dry
    /// frequency.  ie. 0 will disable detune, 1 will double the frequency
    /// (raise one octave), 2 will quadruple (raise two octaves), etc.  Each
    /// whole number shifts the note an octave in that direction. Negatives will
    /// similarly reduce the frequency by that much. -1 will drop an octave, -2
    /// will drop another octave, and so on.
    /// This also detunes all actively playing notes.
    pub fn set_detune(&self, detune: f64) {
        if self.detune.replace(detune) != detune {
            let detune_pow2 = 2.0f64.powf(detune);
            self.detune_pow2.set(detune_pow2);
            self.node.set_frequency(self.frequency.get() * detune_pow2);
        }
    }

    pub fn get_detune(&self) -> f64 {
        self.detune.get()
    }
}

impl Node for DetuneFrequencyNode {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [crate::stream::Stream],
        outputs: &'c mut Vec<crate::stream::Stream>,
    ) {
        self.node.process(inputs, outputs);
    }
}

impl FrequencyNode for DetuneFrequencyNode {
    fn get_frequency(&self) -> f64 {
        self.frequency.get()
    }

    fn set_frequency(&self, frequency: f64) {
        if self.frequency.replace(frequency) != frequency {
            self.node.set_frequency(frequency * self.detune_pow2.get());
        }
    }
}
