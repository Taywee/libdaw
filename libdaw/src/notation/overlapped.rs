mod parse;

use super::{resolve_state::ResolveState, Item};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::{Error, IResult},
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Overlapped(pub Vec<Item>);

impl Overlapped {
    pub(super) fn inner_tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        state: &ResolveState,
    ) -> impl Iterator<Item = Tone> + 'static
    where
        S: PitchStandard + ?Sized,
    {
        let pitches: Vec<_> = self
            .0
            .iter()
            .flat_map(move |item| item.inner_tones(offset, metronome, pitch_standard, state))
            .collect();
        pitches.into_iter()
    }
    pub fn tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
    ) -> impl Iterator<Item = Tone> + 'static
    where
        S: PitchStandard + ?Sized,
    {
        self.inner_tones(offset, metronome, pitch_standard, &Default::default())
    }

    pub(super) fn inner_length(&self, state: &ResolveState) -> Beat {
        self.0
            .iter()
            .map(|item| item.inner_length(state))
            .max()
            .unwrap_or(Beat::ZERO)
    }

    pub(super) fn inner_duration(&self, state: &ResolveState) -> Beat {
        self.0
            .iter()
            .map(|item| item.inner_duration(state))
            .max()
            .unwrap_or(Beat::ZERO)
    }
    pub fn length(&self) -> Beat {
        self.inner_length(&Default::default())
    }
    pub fn duration(&self) -> Beat {
        self.inner_duration(&Default::default())
    }
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::overlapped(input)
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
