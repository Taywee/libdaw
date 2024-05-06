mod parse;

use super::Item;
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::{Error, IResult},
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, Finish as _};
use std::str::FromStr;

/// A linear sequence of items.
#[derive(Default, Debug, Clone)]
pub struct Sequence(pub Vec<Item>);

impl FromStr for Sequence {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(parse::sequence)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}

impl Sequence {
    pub fn resolve<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        mut previous_length: Beat,
    ) -> impl Iterator<Item = Tone> + 'static
    where
        S: PitchStandard + ?Sized,
    {
        let mut start = offset;
        let tones: Vec<_> = self
            .0
            .iter()
            .flat_map(move |item| {
                let resolved = item.resolve(start, metronome, pitch_standard, previous_length);
                start += item.length(previous_length);
                previous_length = item.next_previous_length(previous_length);
                resolved
            })
            .collect();
        tones.into_iter()
    }

    pub fn length(&self, mut previous_length: Beat) -> Beat {
        self.0
            .iter()
            .map(move |item| {
                previous_length = item.length(previous_length);
                previous_length
            })
            .sum()
    }

    pub fn duration(&self, mut previous_length: Beat) -> Beat {
        let mut start = Beat::ZERO;
        let mut duration = Beat::ZERO;
        for item in &self.0 {
            let item_duration = item.duration(previous_length);
            previous_length = item.length(previous_length);
            duration = duration.max(start + item_duration);
            start += previous_length;
        }
        duration
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::sequence(input)
    }
    pub fn deep_clone(&self) -> Self {
        Self(self.0.iter().map(Item::deep_clone).collect())
    }
}
