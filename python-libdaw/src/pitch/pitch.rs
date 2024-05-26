use libdaw::pitch::{Pitch as DawPitch, PitchClass as DawPitchClass, PitchName as DawPitchName};
use pyo3::{
    exceptions::PyValueError, marker::Python, pyclass, pymethods, Bound, IntoPy, Py, PyResult,
    PyTraverseError, PyVisit,
};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.pitch")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitchName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[pymethods]
impl PitchName {
    #[new]
    pub fn new(name: &str) -> PyResult<Self> {
        match name {
            "C" | "c" => Ok(Self::C),
            "D" | "d" => Ok(Self::D),
            "E" | "e" => Ok(Self::E),
            "F" | "f" => Ok(Self::F),
            "G" | "g" => Ok(Self::G),
            "A" | "a" => Ok(Self::A),
            "B" | "b" => Ok(Self::B),
            name => Err(PyValueError::new_err(format!("Unknown name {name}"))),
        }
    }

    pub fn __getnewargs__(&self) -> (&str,) {
        match self {
            PitchName::C => ("C",),
            PitchName::D => ("D",),
            PitchName::E => ("E",),
            PitchName::F => ("F",),
            PitchName::G => ("G",),
            PitchName::A => ("A",),
            PitchName::B => ("B",),
        }
    }
}

impl From<PitchName> for DawPitchName {
    fn from(value: PitchName) -> Self {
        match value {
            PitchName::C => DawPitchName::C,
            PitchName::D => DawPitchName::D,
            PitchName::E => DawPitchName::E,
            PitchName::F => DawPitchName::F,
            PitchName::G => DawPitchName::G,
            PitchName::A => DawPitchName::A,
            PitchName::B => DawPitchName::B,
        }
    }
}
impl From<DawPitchName> for PitchName {
    fn from(value: DawPitchName) -> Self {
        match value {
            DawPitchName::C => PitchName::C,
            DawPitchName::D => PitchName::D,
            DawPitchName::E => PitchName::E,
            DawPitchName::F => PitchName::F,
            DawPitchName::G => PitchName::G,
            DawPitchName::A => PitchName::A,
            DawPitchName::B => PitchName::B,
        }
    }
}

#[pyclass(module = "libdaw.pitch")]
#[derive(Debug, Clone)]
pub struct PitchClass {
    pub inner: Arc<Mutex<DawPitchClass>>,
}

impl PitchClass {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawPitchClass>>) -> Py<Self> {
        Self { inner }
            .into_py(py)
            .downcast_bound(py)
            .unwrap()
            .clone()
            .unbind()
    }
}

#[pymethods]
impl PitchClass {
    #[new]
    pub fn new(name: PitchName, adjustment: Option<f64>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawPitchClass {
                name: name.into(),
                adjustment: adjustment.unwrap_or_default(),
            })),
        }
    }
    #[getter]
    pub fn get_name(&self) -> PitchName {
        self.inner.lock().expect("poisoned").name.into()
    }
    #[getter]
    pub fn get_adjustment(&self) -> f64 {
        self.inner.lock().expect("poisoned").adjustment
    }
    #[setter]
    pub fn set_name(&self, value: PitchName) {
        self.inner.lock().expect("poisoned").name = value.into();
    }
    #[setter]
    pub fn set_adjustment(&self, value: f64) {
        self.inner.lock().expect("poisoned").adjustment = value;
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned").deref())
    }
    pub fn __getnewargs__(&self) -> (PitchName, f64) {
        let lock = self.inner.lock().expect("poisoned");
        (lock.name.into(), lock.adjustment)
    }
}

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
    pub fn new(pitch_class: &Bound<'_, PitchClass>, octave: i8) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawPitch {
                pitch_class: pitch_class.borrow().inner.clone(),
                octave,
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
    pub fn get_octave(&self) -> i8 {
        self.inner.lock().expect("poisoned").octave
    }
    #[setter]
    pub fn set_octave(&self, value: i8) {
        self.inner.lock().expect("poisoned").octave = value;
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned").deref())
    }
    pub fn __getnewargs__(&self) -> (&Py<PitchClass>, i8) {
        let lock = self.inner.lock().expect("poisoned");
        (self.pitch_class.as_ref().expect("cleared"), lock.octave)
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
