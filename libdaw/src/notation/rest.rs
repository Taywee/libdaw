mod parse;

use super::{tone_generation_state::ToneGenerationState, Element};
use crate::{metronome::Beat, parse::IResult};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rest {
    // Conceptual length of the note in beats
    pub length: Option<Beat>,
}

impl Element for Rest {
    fn length(&self, state: &ToneGenerationState) -> Beat {
        self.length.unwrap_or(state.length)
    }
    fn update_state(&self, state: &mut ToneGenerationState) {
        state.length = self.length(state);
        state.offset += state.length;
    }
}

impl Rest {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::rest(input)
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
