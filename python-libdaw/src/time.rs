use crate::ErrorWrapper;
use libdaw::time::{Duration as DawDuration, Time as DawTime, Timestamp as DawTimestamp};
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
pub struct Duration(pub DawDuration);

#[pymethods]
impl Duration {
    #[classattr]
    pub const ZERO: Duration = Duration(DawDuration::ZERO);
    #[classattr]
    pub const MAX: Duration = Duration(DawDuration::MAX);
    #[classattr]
    pub const MIN: Duration = Duration(DawDuration::MIN);

    #[new]
    pub fn new(seconds: &Bound<'_, PyAny>) -> PyResult<Self> {
        let seconds = if let Ok(delta) = seconds.downcast::<PyDelta>() {
            delta.get_days() as f64 * 86400.0
                + delta.get_seconds() as f64
                + delta.get_microseconds() as f64 * (1.0f64 / 1_000_000.0)
        } else {
            seconds.extract()?
        };
        DawDuration::from_seconds(seconds)
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
pub struct Time(pub DawTime);

#[pymethods]
impl Time {
    #[classattr]
    pub const ZERO: Time = Time(DawTime::ZERO);
    #[classattr]
    pub const MAX: Time = Time(DawTime::MAX);
    #[classattr]
    pub const MIN: Time = Time(DawTime::MIN);

    #[new]
    pub fn new(seconds: &Bound<'_, PyAny>) -> PyResult<Self> {
        let seconds = if let Ok(delta) = seconds.downcast::<PyDelta>() {
            delta.get_days() as f64 * 86400.0
                + delta.get_seconds() as f64
                + delta.get_microseconds() as f64 * (1.0f64 / 1_000_000.0)
        } else {
            seconds.extract()?
        };
        DawTime::from_seconds(seconds)
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
pub struct Timestamp(pub DawTimestamp);

#[pymethods]
impl Timestamp {
    #[classattr]
    pub const ZERO: Timestamp = Timestamp(DawTimestamp::ZERO);
    #[classattr]
    pub const MAX: Timestamp = Timestamp(DawTimestamp::MAX);
    #[classattr]
    pub const MIN: Timestamp = Timestamp(DawTimestamp::MIN);

    #[new]
    pub fn new(seconds: &Bound<'_, PyAny>) -> PyResult<Self> {
        let seconds = if let Ok(delta) = seconds.downcast::<PyDelta>() {
            delta.get_days() as f64 * 86400.0
                + delta.get_seconds() as f64
                + delta.get_microseconds() as f64 * (1.0f64 / 1_000_000.0)
        } else {
            seconds.extract()?
        };
        DawTimestamp::from_seconds(seconds)
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
