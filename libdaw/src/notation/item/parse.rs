use super::{Chord, Item, ItemValue, Mode, Note, Overlapped, Rest, Scale, Sequence, Set};
use crate::parse::IResult;
use nom::{branch::alt, combinator::map, error::context};
use std::sync::{Arc, Mutex};

pub fn item_value(input: &str) -> IResult<&str, ItemValue> {
    alt((
        map(context("Set", Set::parse), move |chord| {
            ItemValue::Set(Arc::new(Mutex::new(chord)))
        }),
        map(context("Chord", Chord::parse), move |chord| {
            ItemValue::Chord(Arc::new(Mutex::new(chord)))
        }),
        map(
            context("Overlapped", Overlapped::parse),
            move |overlapped| ItemValue::Overlapped(Arc::new(Mutex::new(overlapped))),
        ),
        map(context("Sequence", Sequence::parse), move |sequence| {
            ItemValue::Sequence(Arc::new(Mutex::new(sequence)))
        }),
        map(context("Scale", Scale::parse), move |scale| {
            ItemValue::Scale(Arc::new(Mutex::new(scale)))
        }),
        map(context("Mode", Mode::parse), move |mode| {
            ItemValue::Mode(Arc::new(Mutex::new(mode)))
        }),
        map(context("Rest", Rest::parse), move |rest| {
            ItemValue::Rest(Arc::new(Mutex::new(rest)))
        }),
        map(context("Note", Note::parse), move |note| {
            ItemValue::Note(Arc::new(Mutex::new(note)))
        }),
    ))(input)
}

pub fn item(input: &str) -> IResult<&str, Item> {
    let (input, inner) = item_value(input)?;
    Ok((input, Item { value: inner }))
}
