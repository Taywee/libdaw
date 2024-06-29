use super::Mode;
use crate::parse::IResult;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, multispace0},
    combinator::cut,
};

pub fn mode(input: &str) -> IResult<&str, Mode> {
    let (input, _) = alt((tag("%"), tag("mode")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, mode) = cut(i64)(input)?;
    Ok((input, Mode { mode }))
}
