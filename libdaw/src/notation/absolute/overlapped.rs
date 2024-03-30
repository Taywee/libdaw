use super::Section;
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::{notation::absolute as parse, Error},
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Overlapped(pub Vec<Section>);

impl Overlapped {
    pub fn resolve<'a, S>(
        &'a self,
        offset: Beat,
        metronome: &'a Metronome,
        standard: &'a S,
    ) -> impl Iterator<Item = Tone> + 'a
    where
        S: PitchStandard + ?Sized,
    {
        self.0
            .iter()
            .flat_map(move |section| section.resolve(offset, metronome, standard))
    }

    pub fn length(&self) -> Beat {
        self.0
            .iter()
            .map(|section| section.length())
            .max()
            .unwrap_or(Beat::ZERO)
    }

    pub fn duration(&self) -> Beat {
        self.0
            .iter()
            .map(|section| section.duration())
            .max()
            .unwrap_or(Beat::ZERO)
    }
}

impl FromStr for Overlapped {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(parse::overlapped)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}
