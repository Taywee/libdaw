mod parse;

use super::{tone_generation_state::ToneGenerationState, NotePitch, Pitch};
use crate::{
    parse::IResult,
    pitch::{PitchClass, PitchName},
};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::{
    ops::RangeBounds,
    str::FromStr,
    sync::{Arc, Mutex},
    vec::Drain,
};

#[derive(Debug, Clone)]
pub struct Scale {
    pitches: Vec<NotePitch>,
}

impl Scale {
    pub fn new(pitches: Vec<NotePitch>) -> crate::Result<Self> {
        if pitches.is_empty() {
            return Err("Scale may not be empty".into());
        }
        Ok(Self { pitches })
    }

    pub fn pitches(&self) -> &[NotePitch] {
        &self.pitches
    }

    pub fn pitches_mut(&mut self) -> &mut [NotePitch] {
        &mut self.pitches
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::scale(input)
    }
    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        let mut scale = Vec::new();
        let mut running_state = state.clone();
        for pitch in &self.pitches {
            let absolute = pitch.absolute(&running_state);
            scale.push(absolute.clone());
            running_state.pitch = absolute;
        }
        state.scale = scale;
        state.step = 0;
        state.inversion = 0;
        state.scale_octave = 0;
    }

    pub fn push(&mut self, value: NotePitch) {
        self.pitches.push(value)
    }

    pub fn insert(&mut self, index: usize, element: NotePitch) {
        self.pitches.insert(index, element)
    }

    pub fn remove(&mut self, index: usize) -> crate::Result<NotePitch> {
        if self.pitches.len() <= 1 {
            return Err("Can not empty scale".into());
        }
        Ok(self.pitches.remove(index))
    }

    /// Clear all pitches, leaving a single C4 in the scale.
    pub fn clear(&mut self) {
        let pitch = NotePitch::Pitch(Arc::new(Mutex::new(Pitch {
            pitch_class: Arc::new(Mutex::new(PitchClass {
                name: PitchName::C,
                adjustment: 0.0,
            })),
            octave: Some(4),
            octave_shift: 0,
        })));
        self.pitches = vec![pitch];
    }

    pub fn drain<R>(&mut self, range: R) -> crate::Result<Drain<'_, NotePitch>>
    where
        R: RangeBounds<usize>,
    {
        if range.contains(&0) && range.contains(&(self.pitches.len() - 1)) {
            return Err("Can not empty scale".into());
        }
        Ok(self.pitches.drain(range))
    }
}

impl FromStr for Scale {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scale = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(scale)
    }
}
