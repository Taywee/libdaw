use crate::{
    notation::absolute::{Chord, Item, Note, Overlapped, Rest, Section},
    parse::{beat, pitch::pitch, IResult},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::preceded,
};

pub fn note(input: &str) -> IResult<&str, Note> {
    let (input, pitch) = pitch(input)?;
    let (input, length) = opt(preceded(tag(":"), beat))(input)?;
    let (input, duration) = opt(preceded(tag(":"), beat))(input)?;
    Ok((
        input,
        Note {
            pitch,
            length,
            duration,
        },
    ))
}

pub fn chord(input: &str) -> IResult<&str, Chord> {
    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, pitches) = separated_list1(multispace1, pitch)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, length) = opt(preceded(tag(":"), beat))(input)?;
    let (input, duration) = opt(preceded(tag(":"), beat))(input)?;
    Ok((
        input,
        Chord {
            pitches,
            length,
            duration,
        },
    ))
}

pub fn rest(input: &str) -> IResult<&str, Rest> {
    let (input, _) = tag("r")(input)?;
    let (input, length) = opt(preceded(tag(":"), beat))(input)?;
    Ok((input, Rest { length }))
}

fn overlapped_subsection(input: &str) -> IResult<&str, Section> {
    let (input, _) = tag("[")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, section) = section(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, section))
}

pub fn overlapped(input: &str) -> IResult<&str, Overlapped> {
    let (input, _) = tag("[")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, subsections) = many1(preceded(multispace0, overlapped_subsection))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Overlapped(subsections)))
}

pub fn item(input: &str) -> IResult<&str, Item> {
    alt((
        map(note, Item::Note),
        map(chord, Item::Chord),
        map(rest, Item::Rest),
        map(overlapped, Item::Overlapped),
    ))(input)
}

pub fn section(input: &str) -> IResult<&str, Section> {
    let (input, _) = multispace0(input)?;
    let (input, items) = separated_list1(multispace1, item)(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Section(items)))
}
