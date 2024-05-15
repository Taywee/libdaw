use crate::metronome::Beat;
use libdaw::notation::absolute::Rest as DawRest;
use pyo3::{pyclass, pymethods, IntoPy as _, Py, Python};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.notation.absolute")]
#[derive(Debug, Clone)]
pub struct Rest {
    pub inner: Arc<Mutex<DawRest>>,
}

impl Rest {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawRest>>) -> Py<Self> {
        Self { inner }
            .into_py(py)
            .downcast_bound(py)
            .unwrap()
            .clone()
            .unbind()
    }
}

#[pymethods]
impl Rest {
    #[new]
    pub fn new(length: Option<Beat>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawRest {
                length: length.map(|beat| beat.0),
            })),
        }
    }
    #[staticmethod]
    pub fn parse(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
    }

    #[getter]
    pub fn get_length_(&self) -> Option<Beat> {
        self.inner.lock().expect("poisoned").length.map(Beat)
    }
    #[setter]
    pub fn set_length_(&mut self, value: Option<Beat>) {
        self.inner.lock().expect("poisoned").length = value.map(|beat| beat.0);
    }

    pub fn length(&self, previous_length: Beat) -> Beat {
        Beat(
            self.inner
                .lock()
                .expect("poisoned")
                .length(previous_length.0),
        )
    }

    pub fn duration_(&self) -> Beat {
        Beat(self.inner.lock().expect("poisoned").duration())
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned").deref())
    }

    pub fn __getnewargs__(&self) -> (Option<Beat>,) {
        (self.get_length_(),)
    }
}
