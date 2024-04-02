use pyo3::{pyclass, pymethods, Bound, PyAny};

#[pyclass(module = "libdaw.pitch")]
#[derive(Debug, Clone)]
pub enum PitchName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl From<PitchName> for libdaw::pitch::PitchName {
    fn from(value: PitchName) -> Self {
        match value {
            PitchName::C => libdaw::pitch::PitchName::C,
            PitchName::D => libdaw::pitch::PitchName::D,
            PitchName::E => libdaw::pitch::PitchName::E,
            PitchName::F => libdaw::pitch::PitchName::F,
            PitchName::G => libdaw::pitch::PitchName::G,
            PitchName::A => libdaw::pitch::PitchName::A,
            PitchName::B => libdaw::pitch::PitchName::B,
        }
    }
}
impl From<libdaw::pitch::PitchName> for PitchName {
    fn from(value: libdaw::pitch::PitchName) -> Self {
        match value {
            libdaw::pitch::PitchName::C => PitchName::C,
            libdaw::pitch::PitchName::D => PitchName::D,
            libdaw::pitch::PitchName::E => PitchName::E,
            libdaw::pitch::PitchName::F => PitchName::F,
            libdaw::pitch::PitchName::G => PitchName::G,
            libdaw::pitch::PitchName::A => PitchName::A,
            libdaw::pitch::PitchName::B => PitchName::B,
        }
    }
}

#[pyclass(module = "libdaw.pitch")]
#[derive(Debug, Clone)]
pub struct PitchClass(pub libdaw::pitch::PitchClass);

#[pymethods]
impl PitchClass {
    #[new]
    pub fn new(name: PitchName, adjustment: Option<f64>) -> Self {
        Self(libdaw::pitch::PitchClass {
            name: name.into(),
            adjustment: adjustment.unwrap_or_default(),
        })
    }
    #[getter]
    pub fn get_name(&self) -> PitchName {
        self.0.name.into()
    }
    #[getter]
    pub fn get_adjustment(&self) -> f64 {
        self.0.adjustment
    }
    pub fn __eq__(&self, other: &Bound<'_, Self>) -> bool {
        self.0 == other.borrow().0
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    pub fn __copy__(&self) -> Self {
        self.clone()
    }

    pub fn __deepcopy__(&self, _memo: &Bound<'_, PyAny>) -> Self {
        self.clone()
    }
}

#[pyclass(module = "libdaw.pitch")]
#[derive(Debug, Clone)]
pub struct Pitch(pub libdaw::pitch::Pitch);

#[pymethods]
impl Pitch {
    #[new]
    pub fn new(pitch_class: PitchClass, octave: i8) -> Self {
        Self(libdaw::pitch::Pitch {
            pitch_class: pitch_class.0,
            octave,
        })
    }
    /// Get a **copy** of the pitch class.
    #[getter]
    pub fn get_pitch_class(&self) -> PitchClass {
        PitchClass(self.0.pitch_class.clone())
    }
    #[getter]
    pub fn get_octave(&self) -> i8 {
        self.0.octave
    }
    pub fn __eq__(&self, other: &Bound<'_, Self>) -> bool {
        self.0 == other.borrow().0
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    pub fn __copy__(&self) -> Self {
        self.clone()
    }

    pub fn __deepcopy__(&self, _memo: &Bound<'_, PyAny>) -> Self {
        self.clone()
    }
}
