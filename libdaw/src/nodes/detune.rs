use crate::{sync::AtomicF64, FrequencyNode, Node, Result};
use std::sync::{atomic::Ordering, Arc};

/// A wrapper for a FrequencyNode that can apply a detune.
#[derive(Debug)]
pub struct Detune {
    node: Arc<dyn FrequencyNode>,
    frequency: AtomicF64,
    detune: AtomicF64,
    detune_pow2: AtomicF64,
}

impl Detune {
    pub fn new(node: Arc<dyn FrequencyNode>) -> Self {
        Self {
            node,
            frequency: AtomicF64::new(256.0),
            detune: Default::default(),
            detune_pow2: AtomicF64::new(1.0),
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
    pub fn set_detune(&self, detune: f64) -> Result<()> {
        if self.detune.swap(detune, Ordering::Relaxed) != detune {
            let detune_pow2 = 2.0f64.powf(detune);
            self.detune_pow2.store(detune_pow2, Ordering::Relaxed);
            self.node
                .set_frequency(self.frequency.load(Ordering::Relaxed) * detune_pow2)?;
        }
        Ok(())
    }

    pub fn get_detune(&self) -> f64 {
        self.detune.load(Ordering::Relaxed)
    }
}

impl Node for Detune {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [crate::stream::Stream],
        outputs: &'c mut Vec<crate::stream::Stream>,
    ) -> Result<()> {
        self.node.process(inputs, outputs)
    }
}

impl FrequencyNode for Detune {
    fn get_frequency(&self) -> Result<f64> {
        Ok(self.frequency.load(Ordering::Relaxed))
    }

    fn set_frequency(&self, frequency: f64) -> Result<()> {
        if self.frequency.swap(frequency, Ordering::Relaxed) != frequency {
            self.node
                .set_frequency(frequency * self.detune_pow2.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
