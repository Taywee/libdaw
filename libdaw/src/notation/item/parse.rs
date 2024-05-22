use super::{Chord, Inversion, Item, Note, Overlapped, Rest, Scale, Sequence, Set};
use crate::parse::IResult;
use nom::{branch::alt, combinator::map, error::context};
use std::sync::{Arc, Mutex};

pub fn item(input: &str) -> IResult<&str, Item> {
    alt((
        map(context("Set", Set::parse), move |chord| {
            Item::Set(Arc::new(Mutex::new(chord)))
        }),
        map(context("Chord", Chord::parse), move |chord| {
            Item::Chord(Arc::new(Mutex::new(chord)))
        }),
        map(
            context("Overlapped", Overlapped::parse),
            move |overlapped| Item::Overlapped(Arc::new(Mutex::new(overlapped))),
        ),
        map(context("Sequence", Sequence::parse), move |sequence| {
            Item::Sequence(Arc::new(Mutex::new(sequence)))
        }),
        map(context("Scale", Scale::parse), move |scale| {
            Item::Scale(Arc::new(Mutex::new(scale)))
        }),
        map(context("Inversion", Inversion::parse), move |inversion| {
            Item::Inversion(Arc::new(Mutex::new(inversion)))
        }),
        map(context("Rest", Rest::parse), move |rest| {
            Item::Rest(Arc::new(Mutex::new(rest)))
        }),
        map(context("Note", Note::parse), move |note| {
            Item::Note(Arc::new(Mutex::new(note)))
        }),
    ))(input)
}
