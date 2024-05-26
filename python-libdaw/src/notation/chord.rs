use super::{NotePitch, StateMember};
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
    pub pitches: Vec<NotePitch>,
}

impl Chord {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawChord>>) -> Py<Self> {
        let pitches = inner
            .lock()
            .expect("poisoned")
            .pitches
            .iter()
            .cloned()
            .map(move |pitch| NotePitch::from_inner(py, pitch))
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
        py: Python<'_>,
        pitches: Option<Vec<NotePitch>>,
        length: Option<Beat>,
        duration: Option<Beat>,
        state_member: Option<StateMember>,
    ) -> Self {
        let pitches = pitches.unwrap_or_default();
        Self {
            inner: Arc::new(Mutex::new(DawChord {
                pitches: pitches
                    .iter()
                    .map(move |pitch| pitch.as_inner(py))
                    .collect(),
                length: length.map(|beat| beat.0),
                duration: duration.map(|beat| beat.0),
                state_member: state_member.map(Into::into),
            })),
            pitches,
        }
    }
    #[staticmethod]
    pub fn loads(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
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
    #[setter]
    pub fn set_length(&mut self, value: Option<Beat>) {
        self.inner.lock().expect("poisoned").length = value.map(|beat| beat.0);
    }
    #[getter]
    pub fn get_duration(&self) -> Option<Beat> {
        self.inner.lock().expect("poisoned").duration.map(Beat)
    }
    #[setter]
    pub fn set_duration(&mut self, value: Option<Beat>) {
        self.inner.lock().expect("poisoned").duration = value.map(|beat| beat.0);
    }
    #[getter]
    pub fn get_state_member(&self) -> Option<StateMember> {
        self.inner
            .lock()
            .expect("poisoned")
            .state_member
            .map(Into::into)
    }
    #[setter]
    pub fn set_state_member(&mut self, value: Option<StateMember>) {
        self.inner.lock().expect("poisoned").state_member = value.map(Into::into);
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned"))
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned"))
    }

    pub fn __len__(&self) -> usize {
        self.pitches.len()
    }
    pub fn __getitem__(&self, index: isize) -> PyResult<NotePitch> {
        let index = resolve_index(self.pitches.len(), index)?;
        Ok(self.pitches[index].clone())
    }
    pub fn __setitem__(&mut self, py: Python<'_>, index: isize, value: NotePitch) -> PyResult<()> {
        let index = resolve_index(self.pitches.len(), index)?;
        self.inner.lock().expect("poisoned").pitches[index] = value.as_inner(py);
        self.pitches[index] = value;
        Ok(())
    }
    pub fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        self.pop(Some(index)).map(|_| ())
    }

    pub fn __iter__(&self) -> ChordIterator {
        ChordIterator(self.pitches.clone().into_iter())
    }

    pub fn append(&mut self, py: Python<'_>, value: NotePitch) -> PyResult<()> {
        self.inner
            .lock()
            .expect("poisoned")
            .pitches
            .push(value.as_inner(py));
        self.pitches.push(value);
        Ok(())
    }

    pub fn insert(&mut self, py: Python<'_>, index: isize, value: NotePitch) -> PyResult<()> {
        let index = resolve_index_for_insert(self.pitches.len(), index)?;
        self.inner
            .lock()
            .expect("poisoned")
            .pitches
            .insert(index, value.as_inner(py));
        self.pitches.insert(index, value);
        Ok(())
    }

    pub fn pop(&mut self, index: Option<isize>) -> PyResult<NotePitch> {
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
    pub fn __getnewargs__(
        &self,
    ) -> (
        Vec<NotePitch>,
        Option<Beat>,
        Option<Beat>,
        Option<StateMember>,
    ) {
        let lock = self.inner.lock().expect("poisoned");
        (
            self.pitches.clone(),
            lock.length.map(Beat),
            lock.duration.map(Beat),
            lock.state_member.map(Into::into),
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
pub struct ChordIterator(pub std::vec::IntoIter<NotePitch>);

#[pymethods]
impl ChordIterator {
    pub fn __iter__(self_: Bound<'_, Self>) -> Bound<'_, Self> {
        self_
    }
    pub fn __repr__(&self) -> String {
        format!("ChordIterator<{:?}>", self.0)
    }
    pub fn __next__(&mut self) -> Option<NotePitch> {
        self.0.next()
    }
}
