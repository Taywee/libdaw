use super::{Item, Overlapped, StateMember};
use crate::parse::IResult;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::{cut, opt},
    multi::separated_list0,
};

pub fn overlapped(input: &str) -> IResult<&str, Overlapped> {
    let (input, _) = alt((tag("*"), tag("overlapped")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, state_member) = opt(StateMember::parse)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(char('('))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, items) = cut(separated_list0(multispace1, Item::parse))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(char(')'))(input)?;
    Ok((
        input,
        Overlapped {
            items,
            state_member,
        },
    ))
}
