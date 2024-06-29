mod parse;

use super::tone_generation_state::ToneGenerationState;
use crate::parse::IResult;
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Mode {
    pub mode: i64,
}

impl Mode {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::mode(input)
    }
    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        state.mode = self.mode;
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mode = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(mode)
    }
}
