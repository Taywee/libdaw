use crate::{
    metronome::{Beat, MaybeMetronome},
    nodes::instrument::Tone,
    pitch::MaybePitchStandard,
};
use libdaw::notation::{Element as Inner, ItemElement as DawItemElement};
use pyo3::{pyclass, pymethods, types::PyAnyMethods as _, Bound, Python};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use super::{Chord, Mode, Note, Overlapped, Rest, Scale, Sequence, Set};

#[derive(Debug, Clone)]
#[pyclass(subclass, module = "libdaw.notation")]
pub struct Element {
    pub inner: Arc<Mutex<dyn Inner>>,
}

impl Element {
    pub fn from_inner<'py>(py: Python<'py>, inner: DawItemElement) -> Bound<'py, Self> {
        let element = match inner {
            DawItemElement::Note(note) => Note::from_inner(py, note).into_any(),
            DawItemElement::Chord(chord) => Chord::from_inner(py, chord).into_any(),
            DawItemElement::Rest(rest) => Rest::from_inner(py, rest).into_any(),
            DawItemElement::Overlapped(overlapped) => {
                Overlapped::from_inner(py, overlapped).into_any()
            }
            DawItemElement::Sequence(sequence) => Sequence::from_inner(py, sequence).into_any(),
            DawItemElement::Scale(scale) => Scale::from_inner(py, scale).into_any(),
            DawItemElement::Mode(mode) => Mode::from_inner(py, mode).into_any(),
            DawItemElement::Set(set) => Set::from_inner(py, set).into_any(),
        };
        element
            .downcast_bound::<Self>(py)
            .expect("Could not upcast Note")
            .clone()
    }

    pub fn as_inner<'py>(self_: &Bound<'py, Self>) -> DawItemElement {
        let any = self_.as_any();
        if let Ok(note) = any.downcast::<Note>() {
            DawItemElement::Note(note.borrow().inner.clone())
        } else if let Ok(chord) = any.downcast::<Chord>() {
            DawItemElement::Chord(chord.borrow().inner.clone())
        } else if let Ok(rest) = any.downcast::<Rest>() {
            DawItemElement::Rest(rest.borrow().inner.clone())
        } else if let Ok(overlapped) = any.downcast::<Overlapped>() {
            DawItemElement::Overlapped(overlapped.borrow().inner.clone())
        } else if let Ok(sequence) = any.downcast::<Sequence>() {
            DawItemElement::Sequence(sequence.borrow().inner.clone())
        } else if let Ok(scale) = any.downcast::<Scale>() {
            DawItemElement::Scale(scale.borrow().inner.clone())
        } else if let Ok(mode) = any.downcast::<Mode>() {
            DawItemElement::Mode(mode.borrow().inner.clone())
        } else if let Ok(set) = any.downcast::<Set>() {
            DawItemElement::Set(set.borrow().inner.clone())
        } else {
            unreachable!()
        }
    }
}

#[pymethods]
impl Element {
    /// Resolve all the section's notes to playable instrument tones.
    #[pyo3(
        signature = (
            *,
            metronome=MaybeMetronome::default(),
            pitch_standard=MaybePitchStandard::default(),
        )
    )]
    pub fn tones(
        &self,
        metronome: MaybeMetronome,
        pitch_standard: MaybePitchStandard,
    ) -> Vec<Tone> {
        self.inner
            .lock()
            .expect("poisoned")
            .tones(&metronome, pitch_standard.deref(), &Default::default())
            .map(Tone)
            .collect()
    }

    pub fn length_(&self) -> Beat {
        Beat(
            self.inner
                .lock()
                .expect("poisoned")
                .length(&Default::default()),
        )
    }

    pub fn duration_(&self) -> Beat {
        Beat(
            self.inner
                .lock()
                .expect("poisoned")
                .duration(&Default::default()),
        )
    }
}
