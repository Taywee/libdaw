mod parse;

use super::{tone_generation_state::ToneGenerationState, Item, StateMember};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::IResult,
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Overlapped {
    pub items: Vec<Item>,
    pub state_member: Option<StateMember>,
}

impl Overlapped {
    pub(super) fn inner_tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        mut state: ToneGenerationState,
    ) -> impl Iterator<Item = Tone> + 'static
    where
        S: PitchStandard + ?Sized,
    {
        let pitches: Vec<_> = self
            .items
            .iter()
            .flat_map(move |item| {
                let resolved = item.inner_tones(offset, metronome, pitch_standard, &state);
                item.update_state(&mut state);
                resolved
            })
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
        self.inner_tones(offset, metronome, pitch_standard, Default::default())
    }

    pub(super) fn inner_length(&self, state: &ToneGenerationState) -> Beat {
        self.items
            .iter()
            .map(|item| item.inner_length(state))
            .max()
            .unwrap_or(Beat::ZERO)
    }

    pub(super) fn inner_duration(&self, state: &ToneGenerationState) -> Beat {
        self.items
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

    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        match self.state_member {
            Some(StateMember::First) => {
                if let Some(item) = self.items.get(0) {
                    item.update_state(state);
                }
            }
            Some(StateMember::Last) => {
                for item in &self.items {
                    item.update_state(state);
                }
            }
            None => (),
        }
    }
}

impl FromStr for Overlapped {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}
