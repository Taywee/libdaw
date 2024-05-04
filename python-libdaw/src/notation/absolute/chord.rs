use crate::{
    metronome::{Beat, MaybeMetronome},
    nodes::instrument::Tone,
    pitch::{MaybePitchStandard, Pitch},
    Result,
};
use libdaw::metronome::Beat as DawBeat;
use pyo3::{exceptions::PyIndexError, pyclass, pymethods, Bound, PyResult};
use std::{
    ops::Deref as _,
    sync::{Arc, Mutex},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Chord(pub Arc<Mutex<libdaw::notation::absolute::Chord>>);

impl Chord {
    fn resolve_index(&self, index: isize) -> PyResult<isize> {
        let len = isize::try_from(self.__len__())
            .map_err(|error| PyIndexError::new_err(error.to_string()))?;
        Ok(if index < 0 { len + index } else { index })
    }
}

#[pymethods]
impl Chord {
    #[new]
    pub fn new(pitches: Vec<Pitch>, length: Option<Beat>, duration: Option<Beat>) -> Self {
        Self(Arc::new(Mutex::new(libdaw::notation::absolute::Chord {
            pitches: pitches.into_iter().map(|pitch| pitch.0).collect(),
            length: length.map(|beat| beat.0),
            duration: duration.map(|beat| beat.0),
        })))
    }
    #[staticmethod]
    pub fn parse(source: String) -> Result<Self> {
        Ok(Self(Arc::new(Mutex::new(source.parse()?))))
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
    ) -> Vec<Tone> {
        let metronome = MaybeMetronome::from(metronome);
        let pitch_standard = MaybePitchStandard::from(pitch_standard);
        self.0
            .lock()
            .expect("poisoned")
            .resolve(
                offset.0,
                &metronome,
                pitch_standard.deref(),
                previous_length.0,
            )
            .map(Tone)
            .collect()
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
        format!("{:?}", self.0.lock().expect("poisoned"))
    }

    pub fn __len__(&self) -> usize {
        self.0.lock().expect("poisoned").pitches.len()
    }
    pub fn __getitem__(&self, index: isize) -> PyResult<Pitch> {
        usize::try_from(self.resolve_index(index)?)
            .ok()
            .and_then(|index| {
                self.0
                    .lock()
                    .expect("poisoned")
                    .pitches
                    .get(index)
                    .cloned()
                    .map(Pitch)
            })
            .ok_or_else(|| PyIndexError::new_err("Index out of range"))
    }
    pub fn __setitem__(&mut self, index: isize, value: Pitch) -> PyResult<()> {
        let mut lock = self.0.lock().expect("poisoned");
        let slot = usize::try_from(self.resolve_index(index)?)
            .ok()
            .and_then(|index| lock.pitches.get_mut(index))
            .ok_or_else(|| PyIndexError::new_err("Index out of range"))?;
        *slot = value.0;
        Ok(())
    }
    pub fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        self.pop(Some(index)).map(|_| ())
    }

    pub fn __iter__(&self) -> ChordIterator {
        ChordIterator(self.0.lock().expect("poisoned").pitches.clone().into_iter())
    }

    pub fn append(&mut self, value: Pitch) -> PyResult<()> {
        self.0.lock().expect("poisoned").pitches.push(value.0);
        Ok(())
    }

    pub fn insert(&mut self, index: isize, value: Pitch) -> PyResult<()> {
        let index = usize::try_from(self.resolve_index(index)?).unwrap_or(0);
        if index >= self.0.lock().expect("poisoned").pitches.len() {
            self.0.lock().expect("poisoned").pitches.push(value.0);
        } else {
            self.0
                .lock()
                .expect("poisoned")
                .pitches
                .insert(index, value.0);
        }
        Ok(())
    }

    pub fn pop(&mut self, index: Option<isize>) -> PyResult<Pitch> {
        let len = self.0.lock().expect("poisoned").pitches.len();
        if len == 0 {
            return Err(PyIndexError::new_err("Pop from empty"));
        }
        let index = match index {
            Some(index) => usize::try_from(self.resolve_index(index)?)
                .ok()
                .filter(move |index| *index < len)
                .ok_or_else(|| PyIndexError::new_err("Index out of range"))?,
            None => len - 1,
        };

        Ok(Pitch(
            self.0.lock().expect("poisoned").pitches.remove(index),
        ))
    }
}

#[derive(Debug, Clone)]
#[pyclass(sequence, module = "libdaw.notation.absolute")]
pub struct ChordIterator(pub std::vec::IntoIter<libdaw::pitch::Pitch>);

#[pymethods]
impl ChordIterator {
    pub fn __iter__(self_: Bound<'_, Self>) -> Bound<'_, Self> {
        self_
    }
    pub fn __repr__(&self) -> String {
        format!("ChordIterator<{:?}>", self.0)
    }
    pub fn __next__(&mut self) -> Option<Pitch> {
        self.0.next().map(Pitch)
    }
}
