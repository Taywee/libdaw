use super::{Chord, Item, ItemValue, Mode, Note, Overlapped, Rest, Scale, Sequence, Set};
use crate::parse::IResult;
use nom::{branch::alt, combinator::map, error::context};
use std::sync::{Arc, Mutex};

fn item_value(input: &str) -> IResult<&str, Arc<Mutex<dyn ItemValue>>> {
    alt((
        map(context("Set", Set::parse), move |set| {
            Arc::new(Mutex::new(set)) as Arc<Mutex<dyn ItemValue>>
        }),
        map(context("Chord", Chord::parse), move |chord| {
            Arc::new(Mutex::new(chord)) as Arc<Mutex<dyn ItemValue>>
        }),
        map(
            context("Overlapped", Overlapped::parse),
            move |overlapped| Arc::new(Mutex::new(overlapped)) as Arc<Mutex<dyn ItemValue>>,
        ),
        map(context("Sequence", Sequence::parse), move |sequence| {
            Arc::new(Mutex::new(sequence)) as Arc<Mutex<dyn ItemValue>>
        }),
        map(context("Scale", Scale::parse), move |scale| {
            Arc::new(Mutex::new(scale)) as Arc<Mutex<dyn ItemValue>>
        }),
        map(context("Mode", Mode::parse), move |mode| {
            Arc::new(Mutex::new(mode)) as Arc<Mutex<dyn ItemValue>>
        }),
        map(context("Rest", Rest::parse), move |rest| {
            Arc::new(Mutex::new(rest)) as Arc<Mutex<dyn ItemValue>>
        }),
        map(context("Note", Note::parse), move |note| {
            Arc::new(Mutex::new(note)) as Arc<Mutex<dyn ItemValue>>
        }),
    ))(input)
}

pub fn item(input: &str) -> IResult<&str, Item> {
    let (input, inner) = item_value(input)?;
    Ok((input, Item { value: inner }))
}
