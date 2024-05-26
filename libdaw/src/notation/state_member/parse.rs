use super::StateMember;
use crate::parse::IResult;
use nom::{branch::alt, character::complete::char};

pub fn state_member(input: &str) -> IResult<&str, StateMember> {
    let (input, symbol) = alt((char('<'), char('>')))(input)?;
    Ok((
        input,
        match symbol {
            '<' => StateMember::First,
            '>' => StateMember::Last,
            _ => unreachable!(),
        },
    ))
}
