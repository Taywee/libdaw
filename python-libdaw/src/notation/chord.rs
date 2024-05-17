use super::Pitch;
use crate::{
    metronome::{Beat, MaybeMetronome},
    nodes::instrument::Tone,
    pitch::MaybePitchStandard,
    resolve_index, resolve_index_for_insert,
};
use libdaw::{metronome::Beat as DawBeat, notation::Chord as DawChord};
use pyo3::{
    exceptions::PyIndexError, pyclass, pymethods, Bound, IntoPy as _, Py, PyResult,
    PyTraverseError, PyVisit, Python,
};
use std::{
    ops::Deref as _,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.notation")]
#[derive(Debug, Clone)]
pub struct Chord {
    pub inner: Arc<Mutex<DawChord>>,
    pub pitches: Vec<Py<Pitch>>,
}

impl Chord {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawChord>>) -> Py<Self> {
        let pitches = inner
            .lock()
            .expect("poisoned")
            .pitches
            .iter()
            .cloned()
            .map(move |pitch| Pitch::from_inner(py, pitch))
            .collect();
        Self { inner, pitches }
            .into_py(py)
            .downcast_bound(py)
            .unwrap()
            .clone()
            .unbind()
    }
}

#[pymethods]
impl Chord {
    #[new]
    pub fn new(
        pitches: Vec<Bound<'_, Pitch>>,
        length: Option<Beat>,
        duration: Option<Beat>,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawChord {
                pitches: pitches
                    .iter()
                    .map(|pitch| pitch.borrow().inner.clone())
                    .collect(),
                length: length.map(|beat| beat.0),
                duration: duration.map(|beat| beat.0),
            })),
            pitches: pitches.into_iter().map(|pitch| pitch.unbind()).collect(),
        }
    }
    #[staticmethod]
    pub fn parse(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
    }

    #[pyo3(
        signature = (
            *,
            offset=Beat(DawBeat::ZERO),
            metronome=MaybeMetronome::default(),
            pitch_standard=MaybePitchStandard::default(),
        )
    )]
    pub fn tones(
        &self,
        offset: Beat,
        metronome: MaybeMetronome,
        pitch_standard: MaybePitchStandard,
    ) -> Vec<Tone> {
        let metronome = MaybeMetronome::from(metronome);
        let pitch_standard = MaybePitchStandard::from(pitch_standard);
        self.inner
            .lock()
            .expect("poisoned")
            .tones(offset.0, &metronome, pitch_standard.deref())
            .map(Tone)
            .collect()
    }

    #[getter]
    pub fn get_length(&self) -> Option<Beat> {
        self.inner.lock().expect("poisoned").length.map(Beat)
    }
    #[getter]
    pub fn get_duration(&self) -> Option<Beat> {
        self.inner.lock().expect("poisoned").duration.map(Beat)
    }
    #[setter]
    pub fn set_length(&mut self, value: Option<Beat>) {
        self.inner.lock().expect("poisoned").length = value.map(|beat| beat.0);
    }
    #[setter]
    pub fn set_duration(&mut self, value: Option<Beat>) {
        self.inner.lock().expect("poisoned").duration = value.map(|beat| beat.0);
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned"))
    }

    pub fn __len__(&self) -> usize {
        self.pitches.len()
    }
    pub fn __getitem__(&self, index: isize) -> PyResult<Py<Pitch>> {
        let index = resolve_index(self.pitches.len(), index)?;
        Ok(self.pitches[index].clone())
    }
    pub fn __setitem__(&mut self, index: isize, value: Bound<'_, Pitch>) -> PyResult<()> {
        let index = resolve_index(self.pitches.len(), index)?;
        self.inner.lock().expect("poisoned").pitches[index] = value.borrow().inner.clone();
        self.pitches[index] = value.unbind();
        Ok(())
    }
    pub fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        self.pop(Some(index)).map(|_| ())
    }

    pub fn __iter__(&self) -> ChordIterator {
        ChordIterator(self.pitches.clone().into_iter())
    }

    pub fn append(&mut self, value: Bound<'_, Pitch>) -> PyResult<()> {
        self.inner
            .lock()
            .expect("poisoned")
            .pitches
            .push(value.borrow().inner.clone());
        self.pitches.push(value.unbind());
        Ok(())
    }

    pub fn insert(&mut self, index: isize, value: Bound<'_, Pitch>) -> PyResult<()> {
        let index = resolve_index_for_insert(self.pitches.len(), index)?;
        self.inner
            .lock()
            .expect("poisoned")
            .pitches
            .insert(index, value.borrow().inner.clone());
        self.pitches.insert(index, value.unbind());
        Ok(())
    }

    pub fn pop(&mut self, index: Option<isize>) -> PyResult<Py<Pitch>> {
        let len = self.pitches.len();
        if len == 0 {
            return Err(PyIndexError::new_err("Pop from empty"));
        }
        let index = match index {
            Some(index) => resolve_index(len, index)?,
            None => len - 1,
        };
        self.inner.lock().expect("poisoned").pitches.remove(index);
        Ok(self.pitches.remove(index))
    }
    pub fn __getnewargs__(&self) -> (Vec<Py<Pitch>>, Option<Beat>, Option<Beat>) {
        let lock = self.inner.lock().expect("poisoned");
        (
            self.pitches.clone(),
            lock.length.map(Beat),
            lock.duration.map(Beat),
        )
    }

    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        for pitch in &self.pitches {
            visit.call(pitch)?
        }
        Ok(())
    }

    pub fn __clear__(&mut self) {
        self.inner.lock().expect("poisoned").pitches.clear();
        self.pitches.clear();
    }
}

#[derive(Debug, Clone)]
#[pyclass(sequence, module = "libdaw.notation")]
pub struct ChordIterator(pub std::vec::IntoIter<Py<Pitch>>);

#[pymethods]
impl ChordIterator {
    pub fn __iter__(self_: Bound<'_, Self>) -> Bound<'_, Self> {
        self_
    }
    pub fn __repr__(&self) -> String {
        format!("ChordIterator<{:?}>", self.0)
    }
    pub fn __next__(&mut self) -> Option<Py<Pitch>> {
        self.0.next()
    }
}