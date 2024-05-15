use crate::{
    metronome::{Beat, MaybeMetronome},
    nodes::instrument::Tone,
    pitch::{MaybePitchStandard, Pitch},
};
use libdaw::metronome::Beat as DawBeat;
use libdaw::notation::absolute::Note as DawNote;
use pyo3::{pyclass, pymethods, Bound, IntoPy as _, Py, PyTraverseError, PyVisit, Python};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.notation.absolute")]
#[derive(Debug, Clone)]
pub struct Note {
    pub inner: Arc<Mutex<DawNote>>,
    pub pitch: Option<Py<Pitch>>,
}

impl Note {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawNote>>) -> Py<Self> {
        let pitch = Pitch::from_inner(py, inner.lock().expect("poisoned").pitch.clone());
        Self {
            inner,
            pitch: Some(pitch),
        }
        .into_py(py)
        .downcast_bound(py)
        .unwrap()
        .clone()
        .unbind()
    }
}

#[pymethods]
impl Note {
    #[new]
    pub fn new(pitch: Bound<'_, Pitch>, length: Option<Beat>, duration: Option<Beat>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawNote {
                pitch: pitch.borrow().inner.clone(),
                length: length.map(|beat| beat.0),
                duration: duration.map(|beat| beat.0),
            })),
            pitch: Some(pitch.unbind()),
        }
    }

    #[staticmethod]
    pub fn parse(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
    }

    #[getter]
    pub fn get_pitch(&self) -> Py<Pitch> {
        self.pitch.clone().expect("cleared")
    }
    #[setter]
    pub fn set_pitch(&mut self, value: Bound<'_, Pitch>) {
        self.inner.lock().expect("poisoned").pitch = value.borrow().inner.clone();
        self.pitch = Some(value.unbind());
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
        Tone(self.inner.lock().expect("poisoned").resolve(
            offset.0,
            &metronome,
            pitch_standard.deref(),
            previous_length.0,
        ))
    }

    #[getter]
    pub fn get_length_(&self) -> Option<Beat> {
        self.inner.lock().expect("poisoned").length.map(Beat)
    }
    #[getter]
    pub fn get_duration_(&self) -> Option<Beat> {
        self.inner.lock().expect("poisoned").duration.map(Beat)
    }
    #[setter]
    pub fn set_length_(&mut self, value: Option<Beat>) {
        self.inner.lock().expect("poisoned").length = value.map(|beat| beat.0);
    }
    #[setter]
    pub fn set_duration_(&mut self, value: Option<Beat>) {
        self.inner.lock().expect("poisoned").duration = value.map(|beat| beat.0);
    }

    pub fn length(&self, previous_length: Beat) -> Beat {
        Beat(
            self.inner
                .lock()
                .expect("poisoned")
                .length(previous_length.0),
        )
    }

    pub fn duration(&self, previous_length: Beat) -> Beat {
        Beat(
            self.inner
                .lock()
                .expect("poisoned")
                .duration(previous_length.0),
        )
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned").deref())
    }

    pub fn __getnewargs__(&self) -> (Py<Pitch>, Option<Beat>, Option<Beat>) {
        let lock = self.inner.lock().expect("poisoned");
        (
            self.pitch.clone().expect("cleared"),
            lock.length.map(Beat),
            lock.duration.map(Beat),
        )
    }

    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        if let Some(pitch) = &self.pitch {
            visit.call(pitch)?
        }
        Ok(())
    }

    pub fn __clear__(&mut self) {
        self.pitch = None;
    }
}
