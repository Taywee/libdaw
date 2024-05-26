mod parse;

use crate::parse::IResult;
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::str::FromStr;

/// The item of a collection that will update tone generation
/// state.
#[derive(Debug, Clone, Copy)]
pub enum StateMember {
    First,
    Last,
}

impl StateMember {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::state_member(input)
    }
}

impl FromStr for StateMember {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}
