use libdaw::notation::Mode as DawMode;
use pyo3::{pyclass, pymethods, IntoPy as _, Py, Python};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.notation")]
#[derive(Debug, Clone)]
pub struct Mode {
    pub inner: Arc<Mutex<DawMode>>,
}

impl Mode {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawMode>>) -> Py<Self> {
        Self { inner }
            .into_py(py)
            .downcast_bound(py)
            .unwrap()
            .clone()
            .unbind()
    }
}

#[pymethods]
impl Mode {
    #[new]
    pub fn new(mode: i64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawMode { mode })),
        }
    }
    #[staticmethod]
    pub fn loads(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
    }

    #[getter]
    pub fn get_mode(&self) -> i64 {
        self.inner.lock().expect("poisoned").mode
    }
    #[setter]
    pub fn set_mode(&mut self, value: i64) {
        self.inner.lock().expect("poisoned").mode = value
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned").deref())
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned").deref())
    }

    pub fn __getnewargs__(&self) -> (i64,) {
        (self.get_mode(),)
    }
}
