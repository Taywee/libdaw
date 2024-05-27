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

/// A linear sequence of items.
#[derive(Default, Debug, Clone)]
pub struct Sequence {
    pub items: Vec<Item>,
    pub state_member: Option<StateMember>,
}

impl FromStr for Sequence {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}

impl Sequence {
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
        let mut start = offset;
        let tones: Vec<_> = self
            .items
            .iter()
            .flat_map(move |item| {
                let resolved = item.inner_tones(start, metronome, pitch_standard, &state);
                start += item.inner_length(&state);
                item.update_state(&mut state);
                resolved
            })
            .collect();
        tones.into_iter()
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

    pub fn length(&self) -> Beat {
        self.inner_length(Default::default())
    }
    pub fn duration(&self) -> Beat {
        self.inner_duration(Default::default())
    }

    pub(super) fn inner_length(&self, mut state: ToneGenerationState) -> Beat {
        self.items
            .iter()
            .map(move |item| {
                let length = item.inner_length(&state);
                item.update_state(&mut state);
                length
            })
            .sum()
    }

    pub(super) fn inner_duration(&self, mut state: ToneGenerationState) -> Beat {
        let mut start = Beat::ZERO;
        let mut duration = Beat::ZERO;
        for item in &self.items {
            let item_duration = item.inner_duration(&state);
            let item_length = item.inner_length(&state);
            item.update_state(&mut state);
            duration = duration.max(start + item_duration);
            start += item_length;
        }
        duration
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::sequence(input)
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
