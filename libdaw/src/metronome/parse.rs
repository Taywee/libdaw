use nom::combinator::map_res;

use crate::parse::IResult;
use crate::{metronome::Beat, parse::number};

/// Parse a number using the `number` parser and turn it into a beat.
pub fn beat(input: &str) -> IResult<&str, Beat> {
    map_res(number, |number| Beat::new(number))(input)
}
