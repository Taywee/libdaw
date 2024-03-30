use super::Item;
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::{notation::absolute as parse, Error},
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, Finish as _};
use std::str::FromStr;

/// A linear sequence of items.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Section(pub Vec<Item>);

impl FromStr for Section {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(parse::section)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}

impl Section {
    pub fn resolve<'a, S>(
        &'a self,
        offset: Beat,
        metronome: &'a Metronome,
        standard: &'a S,
    ) -> impl Iterator<Item = Tone> + 'a
    where
        S: PitchStandard + ?Sized,
    {
        let mut start = offset;
        self.0.iter().flat_map(move |item| {
            let resolved = item.resolve(start, metronome, standard);
            start += item.length();
            resolved
        })
    }

    pub fn length(&self) -> Beat {
        self.0.iter().map(|item| item.length()).sum()
    }

    pub fn duration(&self) -> Beat {
        let mut start = Beat::ZERO;
        let mut duration = Beat::ZERO;
        for item in &self.0 {
            duration = duration.max(start + item.duration());
            start += item.length();
        }
        duration
    }
}
