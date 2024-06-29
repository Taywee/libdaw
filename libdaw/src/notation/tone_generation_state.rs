use super::Duration;
use crate::{
    metronome::Beat,
    pitch::{Pitch, PitchClass, PitchName},
};
use std::sync::{Arc, Mutex};

/// A running state that is used to manage context-aware bits of tone
/// generatian.
#[derive(Debug, Clone)]
pub struct ToneGenerationState {
    /// Previous resolved pitch.
    pub pitch: Pitch,

    /// Previous resolved length.
    pub length: Beat,

    /// Previous set duration.
    pub duration: Duration,

    /// The scale for scale-mode notation.
    pub scale: Vec<Pitch>,

    /// The current scale mode.
    pub mode: i64,

    /// Previous used scale step, normalized into mode 1.
    pub normalized_step: i64,

    /// Previous used scale octave.
    pub scale_octave: i8,
}

impl Default for ToneGenerationState {
    fn default() -> Self {
        Self {
            pitch: Pitch {
                pitch_class: Arc::new(Mutex::new(PitchClass {
                    name: PitchName::C,
                    adjustment: 0.0,
                })),
                octave: 4,
            },
            length: Beat::ONE,
            duration: Duration::default(),
            scale: [
                PitchName::C,
                PitchName::D,
                PitchName::E,
                PitchName::F,
                PitchName::G,
                PitchName::A,
                PitchName::B,
            ]
            .into_iter()
            .map(|name| Pitch {
                pitch_class: Arc::new(Mutex::new(PitchClass {
                    name,
                    adjustment: 0.0,
                })),
                octave: 4,
            })
            .collect(),
            mode: 1,
            normalized_step: 1,
            scale_octave: 0,
        }
    }
}
