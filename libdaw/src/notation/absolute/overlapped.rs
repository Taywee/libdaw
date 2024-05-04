use super::Section;
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::{notation::absolute as parse, Error},
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, Finish as _};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Overlapped(pub Vec<Arc<Mutex<Section>>>);

impl Overlapped {
    pub fn resolve<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
    ) -> impl Iterator<Item = Tone> + 'static
    where
        S: PitchStandard + ?Sized,
    {
        let pitches: Vec<_> = self
            .0
            .iter()
            .flat_map(move |section| {
                section
                    .lock()
                    .expect("poisoned")
                    .resolve(offset, metronome, pitch_standard)
            })
            .collect();
        pitches.into_iter()
    }

    pub fn length(&self) -> Beat {
        self.0
            .iter()
            .map(|section| section.lock().expect("poisoned").length())
            .max()
            .unwrap_or(Beat::ZERO)
    }

    pub fn duration(&self) -> Beat {
        self.0
            .iter()
            .map(|section| section.lock().expect("poisoned").duration())
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
