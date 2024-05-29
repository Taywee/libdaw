use super::{duration::Duration, NotePitch, StateMember};
use crate::{
    indexing::{IndexOrSlice, InsertIndex, ItemOrSequence, PopIndex},
    metronome::{Beat, MaybeMetronome},
    nodes::instrument::Tone,
    pitch::MaybePitchStandard,
};
use libdaw::{metronome::Beat as DawBeat, notation::Chord as DawChord};
use pyo3::{
    pyclass, pymethods, Bound, IntoPy as _, Py, PyResult, PyTraverseError, PyVisit, Python,
};
use std::{
    ops::Deref as _,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.notation", sequence)]
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
        duration: Option<Duration>,
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
                duration: duration.map(|duration| duration.inner),
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
    pub fn get_duration(&self) -> Option<Duration> {
        self.inner
            .lock()
            .expect("poisoned")
            .duration
            .map(|inner| Duration { inner })
    }
    #[setter]
    pub fn set_duration(&mut self, value: Option<Duration>) {
        self.inner.lock().expect("poisoned").duration = value.map(|duration| duration.inner);
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
    pub fn __getitem__(
        &self,
        py: Python<'_>,
        index: IndexOrSlice<'_>,
    ) -> PyResult<ItemOrSequence<NotePitch, Self>> {
        index.get(&self.pitches)?.map_sequence(move |pitches| {
            let inner_pitches = pitches
                .iter()
                .map(move |pitch| pitch.as_inner(py))
                .collect();
            let lock = self.inner.lock().expect("poisoned");
            let inner = Arc::new(Mutex::new(DawChord {
                length: lock.length,
                duration: lock.duration,
                state_member: lock.state_member,
                pitches: inner_pitches,
            }));
            Ok(Self { inner, pitches })
        })
    }
    pub fn __setitem__(
        &mut self,
        py: Python<'_>,
        index: IndexOrSlice<'_>,
        value: ItemOrSequence<NotePitch>,
    ) -> PyResult<()> {
        let len = self.pitches.len();
        let mut userdata = (self.inner.lock().expect("poisoned"), &mut self.pitches);
        index.normalize(len)?.set(
            value,
            &mut userdata,
            move |(lock, pitches), index, value| {
                lock.pitches[index] = value.as_inner(py);
                pitches[index] = value;
                Ok(())
            },
            move |(lock, pitches), index, value| {
                lock.pitches.insert(index, value.as_inner(py));
                pitches.insert(index, value);
                Ok(())
            },
            move |(lock, pitches), range| {
                lock.pitches.drain(range.clone());
                pitches.drain(range);
                Ok(())
            },
        )
    }
    pub fn __delitem__(&mut self, index: IndexOrSlice<'_>) -> PyResult<()> {
        let len = self.pitches.len();
        let mut lock = self.inner.lock().expect("poisoned");
        let pitches = &mut self.pitches;
        index.normalize(len)?.delete(move |range| {
            lock.pitches.drain(range.clone());
            pitches.drain(range);
            Ok(())
        })
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

    pub fn insert(&mut self, py: Python<'_>, index: InsertIndex, value: NotePitch) -> PyResult<()> {
        let index = index.normalize(self.pitches.len())?;
        self.inner
            .lock()
            .expect("poisoned")
            .pitches
            .insert(index, value.as_inner(py));
        self.pitches.insert(index, value);
        Ok(())
    }

    #[pyo3(signature = (index = Default::default()))]
    pub fn pop(&mut self, index: PopIndex) -> PyResult<NotePitch> {
        let index = index.normalize(self.pitches.len())?;
        self.inner.lock().expect("poisoned").pitches.remove(index);
        Ok(self.pitches.remove(index))
    }
    pub fn __getnewargs__(
        &self,
    ) -> (
        Vec<NotePitch>,
        Option<Beat>,
        Option<Duration>,
        Option<StateMember>,
    ) {
        let lock = self.inner.lock().expect("poisoned");
        (
            self.pitches.clone(),
            lock.length.map(Beat),
            lock.duration.map(|inner| Duration { inner }),
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
#[pyclass(module = "libdaw.notation")]
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
