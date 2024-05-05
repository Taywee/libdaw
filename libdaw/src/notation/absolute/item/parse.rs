use crate::{
    notation::absolute::{Chord, Item, Note, Overlapped, Rest},
    parse::IResult,
};
use nom::{branch::alt, combinator::map};
use std::sync::{Arc, Mutex};

pub fn item(input: &str) -> IResult<&str, Item> {
    alt((
        map(Note::parse, move |note| {
            Item::Note(Arc::new(Mutex::new(note)))
        }),
        map(Chord::parse, move |chord| {
            Item::Chord(Arc::new(Mutex::new(chord)))
        }),
        map(Rest::parse, move |rest| {
            Item::Rest(Arc::new(Mutex::new(rest)))
        }),
        map(Overlapped::parse, move |overlapped| {
            Item::Overlapped(Arc::new(Mutex::new(overlapped)))
        }),
    ))(input)
}
