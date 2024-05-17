use std::sync::{Arc, Mutex};

use crate::{
    metronome::Beat,
    pitch::{Pitch, PitchClass},
};

/// A running state that is used to manage a resolve operation.
#[derive(Debug, Clone)]
pub struct ResolveState {
    /// Previous resolved pitch.
    pub pitch: Pitch,

    /// Previous resolved length.
    pub length: Beat,
}

impl Default for ResolveState {
    fn default() -> Self {
        Self {
            pitch: Pitch {
                pitch_class: Arc::new(Mutex::new(PitchClass {
                    name: crate::pitch::PitchName::C,
                    adjustment: 0.0,
                })),
                octave: 4,
            },
            length: Beat::ONE,
        }
    }
}
