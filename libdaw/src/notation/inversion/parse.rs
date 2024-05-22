use super::Inversion;
use crate::parse::IResult;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, multispace0},
    combinator::cut,
};

pub fn inversion(input: &str) -> IResult<&str, Inversion> {
    let (input, _) = alt((tag("%"), tag("inversion")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, inversion) = cut(i64)(input)?;
    Ok((input, Inversion { inversion }))
}
