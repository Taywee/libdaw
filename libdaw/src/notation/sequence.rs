mod parse;

use super::{tone_generation_state::ToneGenerationState, Element, Item, StateMember};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::IResult,
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

/// A linear sequence of items.
#[derive(Default, Debug, Clone)]
pub struct Sequence {
    pub items: Vec<Arc<Mutex<Item>>>,
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

impl Element for Sequence {
    fn tones(
        &self,
        metronome: &Metronome,
        pitch_standard: &dyn PitchStandard,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static> {
        let mut state = state.clone();
        let tones: Vec<_> = self
            .items
            .iter()
            .flat_map(move |item| {
                let item = item.lock().expect("poisoned");
                let resolved = item.tones(metronome, pitch_standard, &state);
                item.update_state(&mut state);
                resolved
            })
            .collect();
        Box::new(tones.into_iter())
    }

    fn length(&self, state: &ToneGenerationState) -> Beat {
        let mut state = state.clone();
        self.items
            .iter()
            .map(move |item| {
                let item = item.lock().expect("poisoned");
                let length = item.length(&state);
                item.update_state(&mut state);
                length
            })
            .sum()
    }

    fn duration(&self, state: &ToneGenerationState) -> Beat {
        let mut state = state.clone();
        let mut start = Beat::ZERO;
        let mut duration = Beat::ZERO;
        for item in &self.items {
            let item = item.lock().expect("poisoned");
            let item_duration = item.duration(&state);
            let item_length = item.length(&state);
            item.update_state(&mut state);
            duration = duration.max(start + item_duration);
            start += item_length;
        }
        duration
    }

    fn update_state(&self, state: &mut ToneGenerationState) {
        let post_offset = state.offset + self.length(state);
        match self.state_member {
            Some(StateMember::First) => {
                if let Some(item) = self.items.get(0) {
                    item.lock().expect("poisoned").update_state(state);
                }
            }
            Some(StateMember::Last) => {
                for item in &self.items {
                    item.lock().expect("poisoned").update_state(state);
                }
            }
            None => (),
        }
        state.offset = post_offset;
    }
}
impl Sequence {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::sequence(input)
    }
}
