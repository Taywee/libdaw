use std::sync::{Arc, Mutex};

use crate::{
    notation::absolute::{Overlapped, Section},
    parse::IResult,
};
use nom::{
    bytes::complete::tag, character::complete::multispace0, combinator::map, multi::many1,
    sequence::preceded,
};

fn overlapped_subsection(input: &str) -> IResult<&str, Section> {
    let (input, _) = tag("[")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, section) = Section::parse(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, section))
}

pub fn overlapped(input: &str) -> IResult<&str, Overlapped> {
    let (input, _) = tag("[")(input)?;
    let (input, subsections) = many1(preceded(
        multispace0,
        map(overlapped_subsection, move |section| {
            Arc::new(Mutex::new(section))
        }),
    ))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Overlapped(subsections)))
}
