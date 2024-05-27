mod parse;

use super::tone_generation_state::ToneGenerationState;
use crate::{metronome::Beat, parse::IResult};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rest {
    // Conceptual length of the note in beats
    pub length: Option<Beat>,
}

impl Rest {
    pub fn inner_length(&self, state: &ToneGenerationState) -> Beat {
        self.length.unwrap_or(state.length)
    }
    pub const fn duration(&self) -> Beat {
        Beat::ZERO
    }
    pub fn length(&self) -> Beat {
        self.inner_length(&Default::default())
    }
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::rest(input)
    }
    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        state.length = self.inner_length(state);
    }
}

impl FromStr for Rest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}
