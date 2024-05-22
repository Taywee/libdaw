use super::NotePitch;
use crate::metronome::Beat;
use libdaw::notation::Set as DawSet;
use pyo3::{pyclass, pymethods, IntoPy as _, Py, PyTraverseError, PyVisit, Python};
use std::sync::{Arc, Mutex};

#[pyclass(module = "libdaw.notation")]
#[derive(Debug, Clone)]
pub struct Set {
    pub inner: Arc<Mutex<DawSet>>,
    pub pitch: Option<NotePitch>,
}

impl Set {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawSet>>) -> Py<Self> {
        let pitch = inner
            .lock()
            .expect("poisoned")
            .pitch
            .as_ref()
            .map(|pitch| NotePitch::from_inner(py, pitch.clone()));
        Self { inner, pitch }
            .into_py(py)
            .downcast_bound(py)
            .unwrap()
            .clone()
            .unbind()
    }
}

#[pymethods]
impl Set {
    #[new]
    pub fn new(py: Python<'_>, pitch: Option<NotePitch>, length: Option<Beat>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawSet {
                pitch: pitch.as_ref().map(move |pitch| pitch.as_inner(py)),
                length: length.map(|beat| beat.0),
            })),
            pitch,
        }
    }
    #[staticmethod]
    pub fn loads(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
    }
    #[getter]
    pub fn get_pitch(&self) -> Option<NotePitch> {
        self.pitch.clone()
    }
    #[setter]
    pub fn set_pitch(&mut self, py: Python<'_>, value: Option<NotePitch>) {
        self.inner.lock().expect("poisoned").pitch =
            value.as_ref().map(move |value| value.as_inner(py));
        self.pitch = value;
    }
    #[getter]
    pub fn get_length(&self) -> Option<Beat> {
        self.inner.lock().expect("poisoned").length.map(Beat)
    }
    #[setter]
    pub fn set_length(&mut self, value: Option<Beat>) {
        self.inner.lock().expect("poisoned").length = value.map(|beat| beat.0);
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned"))
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned"))
    }

    pub fn __getnewargs__(&self) -> (Option<NotePitch>, Option<Beat>) {
        let lock = self.inner.lock().expect("poisoned");
        (self.pitch.clone(), lock.length.map(Beat))
    }

    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        for pitch in &self.pitch {
            visit.call(pitch)?
        }
        Ok(())
    }

    pub fn __clear__(&mut self) {
        self.inner.lock().expect("poisoned").pitch = None;
        self.pitch = None;
    }
}
