use crate::{
    notation::{Item, Overlapped},
    parse::IResult,
};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, multi::many1,
    sequence::preceded,
};

pub fn overlapped(input: &str) -> IResult<&str, Overlapped> {
    let (input, _) = alt((tag("*"), tag("overlapped")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, items) = many1(preceded(multispace0, Item::parse))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Overlapped(items)))
}
