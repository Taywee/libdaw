mod note_pitch;

pub use note_pitch::NotePitch;

use super::{duration::Duration, Element};
use crate::{
    metronome::{Beat},
};
use libdaw::{notation::Note as DawNote};
use pyo3::{
    pyclass, pymethods, Py, PyClassInitializer, PyTraverseError, PyVisit, Python,
};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass(extends = Element, module = "libdaw.notation")]
#[derive(Debug, Clone)]
pub struct Note {
    pub inner: Arc<Mutex<DawNote>>,

    /// Either the Pitch or the Step
    pub pitch: Option<NotePitch>,
}

impl Note {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawNote>>) -> Py<Self> {
        let pitch = NotePitch::from_inner(py, inner.lock().expect("poisoned").pitch.clone());
        Py::new(
            py,
            PyClassInitializer::from(Element {
                inner: inner.clone(),
            })
            .add_subclass(Self {
                inner,
                pitch: Some(pitch),
            }),
        )
        .expect("Could not construct Py")
    }
}

#[pymethods]
impl Note {
    #[new]
    #[pyo3(signature = (pitch, length=None, duration=None))]
    pub fn new(
        py: Python<'_>,
        pitch: NotePitch,
        length: Option<Beat>,
        duration: Option<Duration>,
    ) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(DawNote {
            pitch: pitch.as_inner(py),
            length: length.map(|beat| beat.0),
            duration: duration.map(move |duration| duration.inner),
        }));
        PyClassInitializer::from(Element {
            inner: inner.clone(),
        })
        .add_subclass(Self {
            inner,
            pitch: Some(pitch),
        })
    }

    #[staticmethod]
    pub fn loads(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
    }

    #[getter]
    pub fn get_pitch(&self) -> NotePitch {
        self.pitch.clone().expect("cleared")
    }
    #[setter]
    pub fn set_pitch(&mut self, py: Python<'_>, value: NotePitch) {
        self.inner.lock().expect("poisoned").pitch = value.as_inner(py);
        self.pitch = Some(value);
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

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned").deref())
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned").deref())
    }

    pub fn __getnewargs__(&self) -> (NotePitch, Option<Beat>, Option<Duration>) {
        let lock = self.inner.lock().expect("poisoned");
        (
            self.pitch.clone().expect("cleared"),
            lock.length.map(Beat),
            lock.duration.map(|inner| Duration { inner }),
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
