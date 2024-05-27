use crate::metronome::Beat;
use libdaw::notation::Duration as DawDuration;
use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods as _, PyModule, PyModuleMethods as _},
    Bound, FromPyObject, IntoPy, Py, PyAny, PyResult, Python,
};

#[pyclass(module = "libdaw.notation.duration")]
#[derive(Debug, Clone, Copy)]
pub struct AddLength(Beat);

#[pymethods]
impl AddLength {
    #[new]
    fn new(value: Beat) -> Self {
        Self(value)
    }

    #[getter]
    fn get_value(&self) -> Beat {
        self.0
    }
    pub fn __repr__(&self) -> String {
        format!("AddLength({:?})", self.0 .0)
    }
    pub fn __str__(&self) -> String {
        format!("AddLength({:#?})", self.0 .0)
    }

    fn __getnewargs__(&self) -> (Beat,) {
        (self.0,)
    }
}
#[pyclass(module = "libdaw.notation.duration")]
#[derive(Debug, Clone, Copy)]
pub struct SubtractLength(Beat);

#[pymethods]
impl SubtractLength {
    #[new]
    fn new(value: Beat) -> Self {
        Self(value)
    }

    #[getter]
    fn get_value(&self) -> Beat {
        self.0
    }
    pub fn __repr__(&self) -> String {
        format!("SubtractLength({:?})", self.0 .0)
    }
    pub fn __str__(&self) -> String {
        format!("SubtractLength({:#?})", self.0 .0)
    }
    fn __getnewargs__(&self) -> (Beat,) {
        (self.0,)
    }
}
#[pyclass(module = "libdaw.notation.duration")]
#[derive(Debug, Clone, Copy)]
pub struct MultiplyLength(f64);

#[pymethods]
impl MultiplyLength {
    #[new]
    fn new(value: f64) -> Self {
        Self(value)
    }

    #[getter]
    fn get_value(&self) -> f64 {
        self.0
    }
    pub fn __repr__(&self) -> String {
        format!("MultiplyLength({:?})", self.0)
    }
    pub fn __str__(&self) -> String {
        format!("MultiplyLength({:#?})", self.0)
    }
    fn __getnewargs__(&self) -> (f64,) {
        (self.0,)
    }
}

#[pyclass(module = "libdaw.notation.duration")]
#[derive(Debug, Clone, Copy)]
pub struct Constant(Beat);

#[pymethods]
impl Constant {
    #[new]
    fn new(value: Beat) -> Self {
        Self(value)
    }

    #[getter]
    fn get_value(&self) -> Beat {
        self.0
    }
    pub fn __repr__(&self) -> String {
        format!("Constant({:?})", self.0 .0)
    }
    pub fn __str__(&self) -> String {
        format!("Constant({:#?})", self.0 .0)
    }
    fn __getnewargs__(&self) -> (Beat,) {
        (self.0,)
    }
}

/// A wrapper enum for converting between Rust Durations and the Python classes.
#[derive(Debug, Clone, Copy)]
pub struct Duration {
    pub inner: DawDuration,
}

impl<'py> FromPyObject<'py> for Duration {
    fn extract_bound(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(if let Ok(value) = value.downcast::<AddLength>() {
            Self {
                inner: DawDuration::AddLength(value.borrow().0 .0),
            }
        } else if let Ok(value) = value.downcast::<SubtractLength>() {
            Self {
                inner: DawDuration::SubtractLength(value.borrow().0 .0),
            }
        } else if let Ok(value) = value.downcast::<MultiplyLength>() {
            Self {
                inner: DawDuration::MultiplyLength(value.borrow().0),
            }
        } else if let Ok(value) = value.downcast::<Constant>() {
            Self {
                inner: DawDuration::Constant(value.borrow().0 .0),
            }
        } else {
            return Err(PyTypeError::new_err("Duration was invalid type"));
        })
    }
}

impl IntoPy<Py<PyAny>> for Duration {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self.inner {
            DawDuration::AddLength(value) => Beat(value).into_py(py),
            DawDuration::SubtractLength(value) => Beat(value).into_py(py),
            DawDuration::MultiplyLength(value) => value.into_py(py),
            DawDuration::Constant(value) => Beat(value).into_py(py),
        }
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<AddLength>()?;
    module.add_class::<SubtractLength>()?;
    module.add_class::<MultiplyLength>()?;
    module.add_class::<Constant>()?;
    Ok(())
}
