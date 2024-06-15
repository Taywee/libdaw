use crate::{time::Timestamp, Result};
use libdaw::metronome::Metronome as DawMetronome;
use pyo3::{
    pyclass,
    pyclass::CompareOp,
    pymethods,
    types::{PyAnyMethods as _, PyModule, PyModuleMethods as _},
    Bound, FromPyObject, PyAny, PyRef, PyResult,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash as _, Hasher as _},
    ops::Deref,
};

#[pyclass(module = "libdaw.metronome")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Beat(pub libdaw::metronome::Beat);

#[pymethods]
impl Beat {
    #[new]
    pub fn new(value: f64) -> Result<Self> {
        libdaw::metronome::Beat::new(value)
            .map(Self)
            .map_err(Into::into)
    }

    #[getter]
    pub fn get_value(&self) -> f64 {
        self.0.get()
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    pub fn __richcmp__(&self, other: &Bound<'_, Self>, op: CompareOp) -> bool {
        op.matches(self.0.cmp(&other.borrow().0))
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn __getnewargs__(&self) -> (f64,) {
        (self.0.get(),)
    }
}

#[pyclass(module = "libdaw.metronome")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct BeatsPerMinute(pub libdaw::metronome::BeatsPerMinute);

#[pymethods]
impl BeatsPerMinute {
    #[new]
    pub fn new(value: f64) -> Result<Self> {
        libdaw::metronome::BeatsPerMinute::new(value)
            .map(Self)
            .map_err(Into::into)
    }

    #[getter]
    pub fn get_value(&self) -> f64 {
        self.0.get()
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    pub fn __richcmp__(&self, other: &Bound<'_, Self>, op: CompareOp) -> bool {
        op.matches(self.0.cmp(&other.borrow().0))
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn __getnewargs__(&self) -> (f64,) {
        (self.0.get(),)
    }
}

#[pyclass(module = "libdaw.metronome")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct TempoInstruction(pub libdaw::metronome::TempoInstruction);

#[pymethods]
impl TempoInstruction {
    #[new]
    pub fn new(beat: Beat, tempo: BeatsPerMinute) -> Self {
        Self(libdaw::metronome::TempoInstruction {
            beat: beat.0,
            tempo: tempo.0,
        })
    }

    #[getter]
    pub fn get_beat(&self) -> Beat {
        Beat(self.0.beat)
    }
    #[getter]
    pub fn get_tempo(&self) -> BeatsPerMinute {
        BeatsPerMinute(self.0.tempo)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    pub fn __richcmp__(&self, other: &Bound<'_, Self>, op: CompareOp) -> bool {
        op.matches(self.0.cmp(&other.borrow().0))
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn __getnewargs__(&self) -> (Beat, BeatsPerMinute) {
        (self.get_beat(), self.get_tempo())
    }
}

#[pyclass(module = "libdaw.metronome")]
#[derive(Debug)]
pub struct Metronome(pub libdaw::metronome::Metronome);

#[pymethods]
impl Metronome {
    #[new]
    pub fn new() -> Self {
        Self(libdaw::metronome::Metronome::default())
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
    pub fn add_tempo_instruction(&mut self, instruction: TempoInstruction) {
        self.0.add_tempo_instruction(instruction.0);
    }
    pub fn beat_to_time(&mut self, beat: Beat) -> Timestamp {
        Timestamp(self.0.beat_to_time(beat.0))
    }
}

/// Helper to allow taking an optional metronome with a default value
#[derive(Debug)]
pub enum MaybeMetronome<'py> {
    Py(PyRef<'py, Metronome>),

    Owned(DawMetronome),
}

impl<'py> FromPyObject<'py> for MaybeMetronome<'py> {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let metronome: Bound<'py, Metronome> = ob.extract()?;
        Ok(Self::Py(metronome.borrow()))
    }
}

impl<'py> Deref for MaybeMetronome<'py> {
    type Target = DawMetronome;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeMetronome::Py(metronome) => &metronome.0,
            MaybeMetronome::Owned(metronome) => metronome,
        }
    }
}

impl<'a> Default for MaybeMetronome<'a> {
    fn default() -> Self {
        MaybeMetronome::Owned(Default::default())
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Beat>()?;
    module.add_class::<BeatsPerMinute>()?;
    module.add_class::<TempoInstruction>()?;
    module.add_class::<Metronome>()?;
    Ok(())
}
