use super::{Chord, Item, ItemElement, Mode, Note, Overlapped, Rest, Scale, Sequence, Set};
use crate::parse::IResult;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace0, multispace1},
    combinator::{cut, map},
    error::context,
    multi::{many0, separated_list0},
    sequence::preceded,
};
use std::sync::{Arc, Mutex};

pub fn item_element(input: &str) -> IResult<&str, ItemElement> {
    alt((
        map(context("Set", Set::parse), move |chord| {
            ItemElement::Set(Arc::new(Mutex::new(chord)))
        }),
        map(context("Chord", Chord::parse), move |chord| {
            ItemElement::Chord(Arc::new(Mutex::new(chord)))
        }),
        map(
            context("Overlapped", Overlapped::parse),
            move |overlapped| ItemElement::Overlapped(Arc::new(Mutex::new(overlapped))),
        ),
        map(context("Sequence", Sequence::parse), move |sequence| {
            ItemElement::Sequence(Arc::new(Mutex::new(sequence)))
        }),
        map(context("Scale", Scale::parse), move |scale| {
            ItemElement::Scale(Arc::new(Mutex::new(scale)))
        }),
        map(context("Mode", Mode::parse), move |mode| {
            ItemElement::Mode(Arc::new(Mutex::new(mode)))
        }),
        map(context("Rest", Rest::parse), move |rest| {
            ItemElement::Rest(Arc::new(Mutex::new(rest)))
        }),
        map(context("Note", Note::parse), move |note| {
            ItemElement::Note(Arc::new(Mutex::new(note)))
        }),
    ))(input)
}

pub fn item_tag(input: &str) -> IResult<&str, String> {
    let (input, _) = char('<')(input)?;
    let (input, tag) = cut(take_until(">"))(input)?;
    let (input, _) = cut(char('>'))(input)?;
    Ok((input, tag.into()))
}

pub fn item(input: &str) -> IResult<&str, Item> {
    let (input, tags) = cut(many0(preceded(multispace0, item_tag)))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, element) = item_element(input)?;
    Ok((
        input,
        Item {
            element,
            tags: tags.into_iter().collect(),
        },
    ))
}
