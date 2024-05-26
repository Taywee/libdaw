mod parse;

use crate::{parse::IResult, pitch::Pitch};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::str::FromStr;

use super::tone_generation_state::ToneGenerationState;

/// A notation-specific scale step specification
#[derive(Debug, Clone)]
pub struct Step {
    pub step: i64,
    pub octave_shift: i8,
    pub adjustment: f64,
}

impl Step {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::step(input)
    }

    /// Resolve to an absolute pitch
    pub(super) fn absolute(&self, state: &ToneGenerationState) -> Pitch {
        let scale_octave = self.scale_octave(state);
        let index = (self.step - 1 + state.inversion).rem_euclid(state.scale.len() as i64) as usize;
        let scale_pitch = &state.scale[index];
        let pitch_class = scale_pitch.pitch_class.clone();
        pitch_class.lock().expect("poisoned").adjustment += self.adjustment;
        Pitch {
            pitch_class,
            octave: scale_pitch.octave.saturating_add(scale_octave),
        }
    }

    pub(super) fn scale_octave(&self, state: &ToneGenerationState) -> i8 {
        let half_scale = state.scale.len() / 2;
        let step = (self.step - 1 + state.inversion).rem_euclid(state.scale.len() as i64) as usize;
        let state_step = (state.step - 1).rem_euclid(state.scale.len() as i64) as usize;
        let relative_shift = if state_step + half_scale < step {
            -1
        } else if step + half_scale < state_step {
            1
        } else {
            0
        };
        relative_shift + self.octave_shift + state.scale_octave
    }
    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        let scale_step = (self.step - 1 + state.inversion).rem_euclid(state.scale.len() as i64) + 1;
        let scale_octave = self.scale_octave(state);
        state.step = scale_step;
        state.scale_octave = scale_octave;
    }
}
impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let step = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(step)
    }
}
