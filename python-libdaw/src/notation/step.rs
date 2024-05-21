use libdaw::notation::Step as DawStep;
use pyo3::{marker::Python, pyclass, pymethods, IntoPy, Py};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.pitch")]
#[derive(Debug, Clone)]
pub struct Step {
    pub inner: Arc<Mutex<DawStep>>,
}

impl Step {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawStep>>) -> Py<Self> {
        Self { inner }
            .into_py(py)
            .downcast_bound(py)
            .unwrap()
            .clone()
            .unbind()
    }
}

#[pymethods]
impl Step {
    #[new]
    #[pyo3(signature = (step, octave_shift = 0, adjustment = 0.0))]
    pub fn new(step: i64, octave_shift: i8, adjustment: f64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawStep {
                step,
                octave_shift,
                adjustment,
            })),
        }
    }
    #[getter]
    pub fn get_step(&self) -> i64 {
        self.inner.lock().expect("poisoned").step
    }
    #[setter]
    pub fn set_step(&self, value: i64) {
        self.inner.lock().expect("poisoned").step = value;
    }
    #[getter]
    pub fn get_octave_shift(&self) -> i8 {
        self.inner.lock().expect("poisoned").octave_shift
    }
    #[setter]
    pub fn set_octave_shift(&self, value: i8) {
        self.inner.lock().expect("poisoned").octave_shift = value;
    }
    #[getter]
    pub fn get_adjustment(&self) -> f64 {
        self.inner.lock().expect("poisoned").adjustment
    }
    #[setter]
    pub fn set_adjustment(&self, value: f64) {
        self.inner.lock().expect("poisoned").adjustment = value;
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned").deref())
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned").deref())
    }
    pub fn __getnewargs__(&self) -> (i64, i8) {
        let lock = self.inner.lock().expect("poisoned");
        (lock.step, lock.octave_shift)
    }
}
