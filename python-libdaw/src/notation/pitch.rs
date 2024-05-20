use crate::pitch::PitchClass;
use libdaw::notation::Pitch as DawPitch;
use pyo3::{marker::Python, pyclass, pymethods, Bound, IntoPy, Py, PyTraverseError, PyVisit};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.pitch")]
#[derive(Debug, Clone)]
pub struct Pitch {
    pub inner: Arc<Mutex<DawPitch>>,
    pub pitch_class: Option<Py<PitchClass>>,
}

impl Pitch {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawPitch>>) -> Py<Self> {
        let pitch_class =
            PitchClass::from_inner(py, inner.lock().expect("poisoned").pitch_class.clone());
        Self {
            inner,
            pitch_class: Some(pitch_class),
        }
        .into_py(py)
        .downcast_bound(py)
        .unwrap()
        .clone()
        .unbind()
    }
}

#[pymethods]
impl Pitch {
    #[new]
    #[pyo3(signature = (pitch_class, octave = None, octave_shift = 0))]
    pub fn new(pitch_class: &Bound<'_, PitchClass>, octave: Option<i8>, octave_shift: i8) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawPitch {
                pitch_class: pitch_class.borrow().inner.clone(),
                octave,
                octave_shift,
            })),
            pitch_class: Some(pitch_class.as_unbound().clone()),
        }
    }
    #[getter]
    pub fn get_pitch_class(&self) -> &Py<PitchClass> {
        self.pitch_class.as_ref().expect("cleared")
    }
    #[setter]
    pub fn set_pitch_class(&mut self, value: &Bound<'_, PitchClass>) {
        self.inner.lock().expect("poisoned").pitch_class = value.borrow().inner.clone();
        self.pitch_class = Some(value.as_unbound().clone());
    }
    #[getter]
    pub fn get_octave(&self) -> Option<i8> {
        self.inner.lock().expect("poisoned").octave
    }
    #[setter]
    pub fn set_octave(&self, value: Option<i8>) {
        self.inner.lock().expect("poisoned").octave = value;
    }
    #[getter]
    pub fn get_octave_shift(&self) -> i8 {
        self.inner.lock().expect("poisoned").octave_shift
    }
    #[setter]
    pub fn set_octave_shift(&self, value: i8) {
        self.inner.lock().expect("poisoned").octave_shift = value;
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned").deref())
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned").deref())
    }
    pub fn __getnewargs__(&self) -> (&Py<PitchClass>, Option<i8>, i8) {
        let lock = self.inner.lock().expect("poisoned");
        (
            self.pitch_class.as_ref().expect("cleared"),
            lock.octave,
            lock.octave_shift,
        )
    }

    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        if let Some(pitch_class) = &self.pitch_class {
            visit.call(pitch_class)?
        }
        Ok(())
    }

    pub fn __clear__(&mut self) {
        self.pitch_class = None;
    }
}
