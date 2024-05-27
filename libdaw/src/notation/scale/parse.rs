use super::Scale;
use crate::{notation::NotePitch, parse::IResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::cut,
    multi::separated_list1,
};

pub fn scale(input: &str) -> IResult<&str, Scale> {
    let (input, _) = alt((tag("@"), tag("scale")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(char('('))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, pitches) = cut(separated_list1(multispace1, NotePitch::parse))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(char(')'))(input)?;
    Ok((input, Scale { pitches }))
}
