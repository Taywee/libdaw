mod parse;

use super::resolve_state::ResolveState;
use crate::parse::IResult;
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Inversion {
    pub inversion: i64,
}

impl Inversion {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::inversion(input)
    }
    pub(super) fn update_state(&self, state: &mut ResolveState) {
        state.inversion = self.inversion;
    }
}

impl FromStr for Inversion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inversion = all_consuming(parse::inversion)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(inversion)
    }
}
