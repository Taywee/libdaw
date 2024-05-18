use crate::{
    notation::{Item, Overlapped},
    parse::IResult,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::cut,
    multi::separated_list1,
};

pub fn overlapped(input: &str) -> IResult<&str, Overlapped> {
    let (input, _) = alt((tag("*"), tag("overlapped")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(tag("("))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, items) = cut(separated_list1(multispace1, Item::parse))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(tag(")"))(input)?;
    Ok((input, Overlapped(items)))
}
