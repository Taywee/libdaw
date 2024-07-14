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

#[derive(Debug, Clone)]
pub struct Overlapped {
    pub items: Vec<Arc<Mutex<Item>>>,
    pub state_member: Option<StateMember>,
}

impl Element for Overlapped {
    fn tones(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &dyn PitchStandard,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static> {
        let mut state = state.clone();
        let pitches: Vec<_> = self
            .items
            .iter()
            .flat_map(move |item| {
                let item = item.lock().expect("poisoned");
                let resolved = item.tones(offset, metronome, pitch_standard, &state);
                item.update_state(&mut state);
                resolved
            })
            .collect();
        Box::new(pitches.into_iter())
    }
    fn length(&self, state: &ToneGenerationState) -> Beat {
        self.items
            .iter()
            .map(|item| item.lock().expect("poisoned").length(state))
            .max()
            .unwrap_or(Beat::ZERO)
    }

    fn duration(&self, state: &ToneGenerationState) -> Beat {
        self.items
            .iter()
            .map(|item| item.lock().expect("poisoned").duration(state))
            .max()
            .unwrap_or(Beat::ZERO)
    }
    fn update_state(&self, state: &mut ToneGenerationState) {
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
    }
}
impl Overlapped {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::overlapped(input)
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
