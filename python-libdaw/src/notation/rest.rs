use super::Element;
use crate::metronome::Beat;
use libdaw::notation::Rest as DawRest;
use pyo3::{pyclass, pymethods, Py, PyClassInitializer, Python};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass(extends = Element, module = "libdaw.notation")]
#[derive(Debug, Clone)]
pub struct Rest {
    pub inner: Arc<Mutex<DawRest>>,
}

impl Rest {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawRest>>) -> Py<Self> {
        Py::new(
            py,
            PyClassInitializer::from(Element {
                inner: inner.clone(),
            })
            .add_subclass(Self { inner }),
        )
        .expect("Could not construct Py")
    }
}

#[pymethods]
impl Rest {
    #[new]
    #[pyo3(signature = (length=None))]
    pub fn new(length: Option<Beat>) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(DawRest {
            length: length.map(|beat| beat.0),
        }));
        PyClassInitializer::from(Element {
            inner: inner.clone(),
        })
        .add_subclass(Self { inner })
    }
    #[staticmethod]
    pub fn loads(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
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
        format!("{:?}", self.inner.lock().expect("poisoned").deref())
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned").deref())
    }

    pub fn __getnewargs__(&self) -> (Option<Beat>,) {
        (self.get_length(),)
    }
}
