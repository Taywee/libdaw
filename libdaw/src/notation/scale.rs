mod parse;

use super::{tone_generation_state::ToneGenerationState, NotePitch};
use crate::parse::IResult;
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Scale {
    pub pitches: Vec<NotePitch>,
}

impl Scale {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::scale(input)
    }
    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        let mut scale = Vec::new();
        let mut running_state = state.clone();
        for pitch in &self.pitches {
            let absolute = pitch.absolute(&running_state);
            scale.push(absolute.clone());
            running_state.pitch = absolute;
        }
        state.scale = scale;
        state.step = 0;
        state.inversion = 0;
        state.scale_octave = 0;
    }
}

impl FromStr for Scale {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scale = all_consuming(parse::scale)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(scale)
    }
}
