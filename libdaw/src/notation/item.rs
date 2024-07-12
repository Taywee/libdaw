mod parse;

use super::{
    tone_generation_state::ToneGenerationState, Chord, Mode, Note, Overlapped, Rest, Scale,
    Sequence, Set,
};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::IResult,
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::{
    fmt,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub enum InnerItem {
    Note(Note),
    Chord(Chord),
    Rest(Rest),
    Overlapped(Overlapped),
    Sequence(Sequence),
    Scale(Scale),
    Mode(Mode),
    Set(Set),
}

impl InnerItem {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub(super) fn inner_tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static>
    where
        S: PitchStandard + ?Sized,
    {
        match self {
            InnerItem::Note(note) => Box::new(std::iter::once(note.inner_tone(
                offset,
                metronome,
                pitch_standard,
                state,
            ))),
            InnerItem::Chord(chord) => {
                Box::new(chord.inner_tones(offset, metronome, pitch_standard, state))
            }
            InnerItem::Overlapped(overlapped) => {
                Box::new(overlapped.inner_tones(offset, metronome, pitch_standard, state.clone()))
            }
            InnerItem::Sequence(sequence) => {
                Box::new(sequence.inner_tones(offset, metronome, pitch_standard, state.clone()))
            }
            InnerItem::Scale(_) | InnerItem::Mode(_) | InnerItem::Rest(_) | InnerItem::Set(_) => {
                Box::new(std::iter::empty())
            }
        }
    }
    pub fn tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
    ) -> Box<dyn Iterator<Item = Tone> + 'static>
    where
        S: PitchStandard + ?Sized,
    {
        self.inner_tones(offset, metronome, pitch_standard, &Default::default())
    }

    pub(super) fn inner_length(&self, state: &ToneGenerationState) -> Beat {
        match self {
            InnerItem::Note(note) => note.inner_length(state),
            InnerItem::Chord(chord) => chord.inner_length(state),
            InnerItem::Rest(rest) => rest.inner_length(state),
            InnerItem::Overlapped(overlapped) => overlapped.inner_length(state),
            InnerItem::Sequence(sequence) => sequence.inner_length(state.clone()),
            InnerItem::Scale(_) | InnerItem::Mode(_) | InnerItem::Set(_) => Beat::ZERO,
        }
    }

    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        match self {
            InnerItem::Note(note) => note.update_state(state),
            InnerItem::Chord(chord) => chord.update_state(state),
            InnerItem::Rest(rest) => rest.update_state(state),
            InnerItem::Scale(scale) => scale.update_state(state),
            InnerItem::Mode(mode) => mode.update_state(state),
            InnerItem::Set(set) => set.update_state(state),
            InnerItem::Sequence(sequence) => sequence.update_state(state),
            InnerItem::Overlapped(overlapped) => overlapped.update_state(state),
        }
    }

    pub(super) fn inner_duration(&self, state: &ToneGenerationState) -> Beat {
        match self {
            InnerItem::Note(note) => note.inner_duration(state),
            InnerItem::Chord(chord) => chord.inner_duration(state),
            InnerItem::Rest(rest) => rest.duration(),
            InnerItem::Overlapped(overlapped) => overlapped.inner_duration(state),
            InnerItem::Sequence(sequence) => sequence.inner_duration(state.clone()),
            InnerItem::Scale(_) | InnerItem::Mode(_) | InnerItem::Set(_) => Beat::ZERO,
        }
    }
    pub fn length(&self) -> Beat {
        self.inner_length(&Default::default())
    }
    pub fn duration(&self) -> Beat {
        self.inner_duration(&Default::default())
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::inner_item(input)
    }
}

impl FromStr for InnerItem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub inner: InnerItem,
}

impl Item {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub(super) fn inner_tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static>
    where
        S: PitchStandard + ?Sized,
    {
        self.inner
            .inner_tones(offset, metronome, pitch_standard, state)
    }
    pub fn tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
    ) -> Box<dyn Iterator<Item = Tone> + 'static>
    where
        S: PitchStandard + ?Sized,
    {
        self.inner.tones(offset, metronome, pitch_standard)
    }

    pub(super) fn inner_length(&self, state: &ToneGenerationState) -> Beat {
        self.inner.inner_length(state)
    }

    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        self.inner.update_state(state)
    }

    pub(super) fn inner_duration(&self, state: &ToneGenerationState) -> Beat {
        self.inner.inner_duration(state)
    }
    pub fn length(&self) -> Beat {
        self.inner_length(&Default::default())
    }
    pub fn duration(&self) -> Beat {
        self.inner_duration(&Default::default())
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::item(input)
    }
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}
