use super::super::{Pitch, Step};
use libdaw::notation::NotePitch as DawNotePitch;
use pyo3::{
    exceptions::PyTypeError, types::PyAnyMethods as _, AsPyPointer, Bound, FromPyObject, IntoPy,
    Py, PyAny, PyResult, Python,
};

/// A wrapper enum for converting between Rust NotePitchs and the Python classes.
#[derive(Debug, Clone)]
pub enum NotePitch {
    Pitch(Py<Pitch>),
    Step(Py<Step>),
}

impl NotePitch {
    pub fn from_inner(py: Python<'_>, inner: DawNotePitch) -> Self {
        match inner {
            DawNotePitch::Pitch(pitch) => Self::Pitch(Pitch::from_inner(py, pitch)),
            DawNotePitch::Step(step) => Self::Step(Step::from_inner(py, step)),
        }
    }
    pub fn as_inner(&self, py: Python<'_>) -> DawNotePitch {
        match self {
            NotePitch::Pitch(pitch) => {
                DawNotePitch::Pitch(pitch.bind_borrowed(py).borrow().inner.clone())
            }
            NotePitch::Step(step) => {
                DawNotePitch::Step(step.bind_borrowed(py).borrow().inner.clone())
            }
        }
    }
}

impl<'py> FromPyObject<'py> for NotePitch {
    fn extract_bound(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(if let Ok(pitch) = value.downcast::<Pitch>() {
            Self::Pitch(pitch.clone().unbind())
        } else if let Ok(step) = value.downcast::<Step>() {
            Self::Step(step.clone().unbind())
        } else {
            return Err(PyTypeError::new_err("NotePitch was invalid type"));
        })
    }
}

impl IntoPy<Py<PyAny>> for NotePitch {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            NotePitch::Pitch(pitch) => pitch.into_py(py),
            NotePitch::Step(step) => step.into_py(py),
        }
    }
}

unsafe impl AsPyPointer for NotePitch {
    fn as_ptr(&self) -> *mut pyo3::ffi::PyObject {
        match self {
            NotePitch::Pitch(pitch) => pitch.as_ptr(),
            NotePitch::Step(step) => step.as_ptr(),
        }
    }
}
