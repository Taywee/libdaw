use crate::stream::Stream;
use crate::{Node, Result};
use std::cell::Cell;

#[derive(Debug)]
pub struct Gain {
    gain: Cell<f64>,
}

impl Gain {
    pub fn new(gain: f64) -> Self {
        Self {
            gain: Cell::new(gain),
        }
    }

    pub fn set_gain(&self, gain: f64) {
        self.gain.set(gain);
    }

    pub fn get_gain(&self) -> f64 {
        self.gain.get()
    }
}

impl Node for Gain {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [Stream],
        outputs: &'c mut Vec<Stream>,
    ) -> Result<()> {
        outputs.extend_from_slice(inputs);
        let gain = self.gain.get();

        for output in outputs {
            *output *= gain;
        }
        Ok(())
    }
}
