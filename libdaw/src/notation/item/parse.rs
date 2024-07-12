use super::{Chord, InnerItem, Item, Mode, Note, Overlapped, Rest, Scale, Sequence, Set};
use crate::parse::IResult;
use nom::{branch::alt, combinator::map, error::context};
use std::sync::{Arc, Mutex};

pub fn inner_item(input: &str) -> IResult<&str, InnerItem> {
    alt((
        map(context("Set", Set::parse), move |chord| {
            InnerItem::Set(chord)
        }),
        map(context("Chord", Chord::parse), move |chord| {
            InnerItem::Chord(chord)
        }),
        map(
            context("Overlapped", Overlapped::parse),
            move |overlapped| InnerItem::Overlapped(overlapped),
        ),
        map(context("Sequence", Sequence::parse), move |sequence| {
            InnerItem::Sequence(sequence)
        }),
        map(context("Scale", Scale::parse), move |scale| {
            InnerItem::Scale(scale)
        }),
        map(context("Mode", Mode::parse), move |mode| {
            InnerItem::Mode(mode)
        }),
        map(context("Rest", Rest::parse), move |rest| {
            InnerItem::Rest(rest)
        }),
        map(context("Note", Note::parse), move |note| {
            InnerItem::Note(note)
        }),
    ))(input)
}

pub fn item(input: &str) -> IResult<&str, Item> {
    let (input, inner) = inner_item(input)?;
    Ok((input, Item { inner }))
}
