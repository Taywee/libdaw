use crate::{
    metronome::Beat,
    parse::{notation::absolute as parse, Error},
};
use nom::{combinator::all_consuming, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rest {
    // Conceptual length of the note in beats
    pub length: Option<Beat>,
}

impl Rest {
    pub fn length(&self, default_length: Beat) -> Beat {
        self.length.unwrap_or(default_length)
    }
    pub const fn duration(&self) -> Beat {
        Beat::ZERO
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
