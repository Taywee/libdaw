use libdaw::notation::Inversion as DawInversion;
use pyo3::{pyclass, pymethods, IntoPy as _, Py, Python};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.notation")]
#[derive(Debug, Clone)]
pub struct Inversion {
    pub inner: Arc<Mutex<DawInversion>>,
}

impl Inversion {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawInversion>>) -> Py<Self> {
        Self { inner }
            .into_py(py)
            .downcast_bound(py)
            .unwrap()
            .clone()
            .unbind()
    }
}

#[pymethods]
impl Inversion {
    #[new]
    pub fn new(inversion: i64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawInversion { inversion })),
        }
    }
    #[staticmethod]
    pub fn loads(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
    }

    #[getter]
    pub fn get_inversion(&self) -> i64 {
        self.inner.lock().expect("poisoned").inversion
    }
    #[setter]
    pub fn set_inversion(&mut self, value: i64) {
        self.inner.lock().expect("poisoned").inversion = value
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned").deref())
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned").deref())
    }

    pub fn __getnewargs__(&self) -> (i64,) {
        (self.get_inversion(),)
    }
}
