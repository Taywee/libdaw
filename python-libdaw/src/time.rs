use crate::ErrorWrapper;
use pyo3::{
    exceptions::PyValueError,
    pyclass,
    pyclass::CompareOp,
    pymethods,
    types::{PyAnyMethods as _, PyDelta, PyDeltaAccess as _, PyModule, PyModuleMethods as _},
    Bound, PyAny, PyResult, Python,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as _, Hasher as _};

#[pyclass(module = "libdaw.time")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(pub libdaw::time::Duration);

#[pymethods]
impl Duration {
    #[new]
    pub fn new(seconds: &Bound<'_, PyAny>) -> PyResult<Self> {
        let seconds = if let Ok(delta) = seconds.downcast::<PyDelta>() {
            delta.get_days() as f64 * 86400.0
                + delta.get_seconds() as f64
                + delta.get_microseconds() as f64 * (1.0f64 / 1_000_000.0)
        } else {
            seconds.extract()?
        };
        libdaw::time::Duration::from_seconds(seconds)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn seconds(&self) -> f64 {
        self.0.seconds()
    }

    pub fn timedelta<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDelta>> {
        let seconds = self.0.seconds();
        let whole_seconds = seconds as u64;
        let microseconds = (seconds.fract() * 1e6) as i32;
        let days = (whole_seconds / 86400)
            .try_into()
            .map_err(ErrorWrapper::from)?;
        let seconds = (whole_seconds % 86400)
            .try_into()
            .map_err(ErrorWrapper::from)?;
        PyDelta::new_bound(py, days, seconds, microseconds, false)
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
        (self.0.seconds(),)
    }
}

#[pyclass(module = "libdaw.time")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time(pub libdaw::time::Time);

#[pymethods]
impl Time {
    #[new]
    pub fn new(seconds: &Bound<'_, PyAny>) -> PyResult<Self> {
        let seconds = if let Ok(delta) = seconds.downcast::<PyDelta>() {
            delta.get_days() as f64 * 86400.0
                + delta.get_seconds() as f64
                + delta.get_microseconds() as f64 * (1.0f64 / 1_000_000.0)
        } else {
            seconds.extract()?
        };
        libdaw::time::Time::from_seconds(seconds)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn seconds(&self) -> f64 {
        self.0.seconds()
    }

    pub fn timedelta<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDelta>> {
        let seconds = self.0.seconds();
        let whole_seconds = seconds as u64;
        let microseconds = (seconds.fract() * 1e6) as i32;
        let days = (whole_seconds / 86400)
            .try_into()
            .map_err(ErrorWrapper::from)?;
        let seconds = (whole_seconds % 86400)
            .try_into()
            .map_err(ErrorWrapper::from)?;
        PyDelta::new_bound(py, days, seconds, microseconds, false)
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
        (self.0.seconds(),)
    }
}

#[pyclass(module = "libdaw.time")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub libdaw::time::Timestamp);

#[pymethods]
impl Timestamp {
    #[new]
    pub fn new(seconds: &Bound<'_, PyAny>) -> PyResult<Self> {
        let seconds = if let Ok(delta) = seconds.downcast::<PyDelta>() {
            delta.get_days() as f64 * 86400.0
                + delta.get_seconds() as f64
                + delta.get_microseconds() as f64 * (1.0f64 / 1_000_000.0)
        } else {
            seconds.extract()?
        };
        libdaw::time::Timestamp::from_seconds(seconds)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn seconds(&self) -> f64 {
        self.0.seconds()
    }

    pub fn timedelta<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDelta>> {
        let seconds = self.0.seconds();
        let whole_seconds = seconds as u64;
        let microseconds = (seconds.fract() * 1e6) as i32;
        let days = (whole_seconds / 86400)
            .try_into()
            .map_err(ErrorWrapper::from)?;
        let seconds = (whole_seconds % 86400)
            .try_into()
            .map_err(ErrorWrapper::from)?;
        PyDelta::new_bound(py, days, seconds, microseconds, false)
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
        (self.0.seconds(),)
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Time>()?;
    module.add_class::<Timestamp>()?;
    module.add_class::<Duration>()?;
    Ok(())
}
