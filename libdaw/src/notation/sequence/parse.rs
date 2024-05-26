use super::{Item, Sequence, StateMember};
use crate::parse::IResult;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{cut, opt},
    multi::separated_list1,
};

pub fn sequence(input: &str) -> IResult<&str, Sequence> {
    let (input, _) = alt((tag("+"), tag("sequence")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, state_member) = opt(StateMember::parse)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(tag("("))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, items) = cut(separated_list1(multispace1, Item::parse))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(tag(")"))(input)?;
    Ok((
        input,
        Sequence {
            items,
            state_member,
        },
    ))
}
