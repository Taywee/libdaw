use super::{Beat, Duration};
use crate::parse::{number, IResult};
use nom::{branch::alt, character::complete::char, sequence::preceded};

fn add_length(input: &str) -> IResult<&str, Duration> {
    let (input, length) = preceded(char('+'), Beat::parse)(input)?;
    Ok((input, Duration::AddLength(length)))
}

fn subtract_length(input: &str) -> IResult<&str, Duration> {
    let (input, length) = preceded(char('-'), Beat::parse)(input)?;
    Ok((input, Duration::SubtractLength(length)))
}

fn multiply_length(input: &str) -> IResult<&str, Duration> {
    let (input, length) = preceded(char('*'), number)(input)?;
    Ok((input, Duration::MultiplyLength(length)))
}

fn constant(input: &str) -> IResult<&str, Duration> {
    let (input, length) = Beat::parse(input)?;
    Ok((input, Duration::Constant(length)))
}

pub fn duration(input: &str) -> IResult<&str, Duration> {
    alt((add_length, subtract_length, multiply_length, constant))(input)
}
