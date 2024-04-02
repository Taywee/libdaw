use crate::{stream::Stream, sync::AtomicF64, Node, Result};
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct Gain {
    gain: AtomicF64,
}

impl Gain {
    pub fn new(gain: f64) -> Self {
        Self {
            gain: AtomicF64::new(gain),
        }
    }

    pub fn set_gain(&self, gain: f64) {
        self.gain.store(gain, Ordering::Relaxed);
    }

    pub fn get_gain(&self) -> f64 {
        self.gain.load(Ordering::Relaxed)
    }
}

impl Node for Gain {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [Stream],
        outputs: &'c mut Vec<Stream>,
    ) -> Result<()> {
        outputs.extend_from_slice(inputs);
        let gain = self.gain.load(Ordering::Relaxed);

        for output in outputs {
            *output *= gain;
        }
        Ok(())
    }
}
