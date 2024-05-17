mod parse;

use super::resolve_state::ResolveState;
use crate::{
    metronome::Beat,
    parse::{Error, IResult},
};
use nom::{combinator::all_consuming, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rest {
    // Conceptual length of the note in beats
    pub length: Option<Beat>,
}

impl Rest {
    pub fn inner_length(&self, state: &ResolveState) -> Beat {
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
}

impl FromStr for Rest {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(parse::rest)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}
