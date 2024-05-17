use crate::{
    notation::{Item, Sequence},
    parse::IResult,
};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, multi::many1,
    sequence::preceded,
};

pub fn sequence(input: &str) -> IResult<&str, Sequence> {
    let (input, _) = alt((tag("+"), tag("sequence")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, items) = many1(preceded(multispace0, Item::parse))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Sequence(items)))
}
