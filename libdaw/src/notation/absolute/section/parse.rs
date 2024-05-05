use crate::{
    notation::absolute::{Item, Section},
    parse::IResult,
};
use nom::{
    character::complete::{multispace0, multispace1},
    multi::separated_list1,
};

pub fn section(input: &str) -> IResult<&str, Section> {
    let (input, _) = multispace0(input)?;
    let (input, items) = separated_list1(multispace1, Item::parse)(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Section(items)))
}
