use super::Inversion;
use crate::parse::IResult;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, u64},
    combinator::{cut, map_res},
};

pub fn inversion(input: &str) -> IResult<&str, Inversion> {
    let (input, _) = alt((tag("%"), tag("inversion")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, inversion) = cut(map_res(u64, usize::try_from))(input)?;
    Ok((input, Inversion { inversion }))
}
