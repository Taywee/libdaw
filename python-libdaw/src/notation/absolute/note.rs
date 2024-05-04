use crate::{
    metronome::{Beat, MaybeMetronome},
    nodes::instrument::Tone,
    pitch::{MaybePitchStandard, Pitch},
    Result,
};
use libdaw::metronome::Beat as DawBeat;
use pyo3::{pyclass, pymethods};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Note(pub Arc<Mutex<libdaw::notation::absolute::Note>>);

#[pymethods]
impl Note {
    #[new]
    pub fn new(pitch: Pitch, length: Option<Beat>, duration: Option<Beat>) -> Self {
        Self(Arc::new(Mutex::new(libdaw::notation::absolute::Note {
            pitch: pitch.0,
            length: length.map(|beat| beat.0),
            duration: duration.map(|beat| beat.0),
        })))
    }
    #[staticmethod]
    pub fn parse(source: String) -> Result<Self> {
        Ok(Self(Arc::new(Mutex::new(source.parse()?))))
    }

    #[getter]
    pub fn get_pitch(&self) -> Pitch {
        Pitch(self.0.lock().expect("poisoned").pitch)
    }
    #[setter]
    pub fn set_pitch(&mut self, value: Pitch) {
        self.0.lock().expect("poisoned").pitch = value.0
    }

    #[pyo3(
        signature = (
            *,
            offset=Beat(DawBeat::ZERO),
            metronome=MaybeMetronome::default(),
            pitch_standard=MaybePitchStandard::default(),
            previous_length=Beat(DawBeat::ONE),
        )
    )]
    pub fn resolve(
        &self,
        offset: Beat,
        metronome: MaybeMetronome,
        pitch_standard: MaybePitchStandard,
        previous_length: Beat,
    ) -> Tone {
        let metronome = MaybeMetronome::from(metronome);
        let pitch_standard = MaybePitchStandard::from(pitch_standard);
        Tone(self.0.lock().expect("poisoned").resolve(
            offset.0,
            &metronome,
            pitch_standard.deref(),
            previous_length.0,
        ))
    }

    pub fn get_length(&self) -> Option<Beat> {
        self.0.lock().expect("poisoned").length.map(Beat)
    }
    pub fn get_duration(&self) -> Option<Beat> {
        self.0.lock().expect("poisoned").duration.map(Beat)
    }
    #[pyo3(signature = (value))]
    pub fn set_length(&mut self, value: Option<Beat>) {
        self.0.lock().expect("poisoned").length = value.map(|beat| beat.0);
    }
    #[pyo3(signature = (value))]
    pub fn set_duration(&mut self, value: Option<Beat>) {
        self.0.lock().expect("poisoned").duration = value.map(|beat| beat.0);
    }

    pub fn length(&self, previous_length: Beat) -> Beat {
        Beat(self.0.lock().expect("poisoned").length(previous_length.0))
    }

    pub fn duration(&self, previous_length: Beat) -> Beat {
        Beat(self.0.lock().expect("poisoned").duration(previous_length.0))
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0.lock().expect("poisoned").deref())
    }
}
