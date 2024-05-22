mod pitch;

use libdaw::pitch::{
    PitchStandard as DawPitchStandard, ScientificPitch as DawScientificPitch, A440 as DawA440,
};
pub use pitch::{Pitch, PitchClass, PitchName};
use pyo3::{
    pyclass, pymethods,
    types::{PyAnyMethods as _, PyModule, PyModuleMethods as _},
    Bound, FromPyObject, PyAny, PyClassInitializer, PyRef, PyResult,
};
use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
#[pyclass(subclass, module = "libdaw.pitch")]
pub struct PitchStandard(pub Arc<dyn DawPitchStandard>);

#[pymethods]
impl PitchStandard {
    pub fn resolve(&self, pitch: &Bound<'_, Pitch>) -> f64 {
        self.0
            .resolve(&pitch.borrow().inner.lock().expect("poisoned"))
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", (&*self.0))
    }
}

#[pyclass(extends = PitchStandard, subclass, module = "libdaw.pitch")]
pub struct A440(pub Arc<DawA440>);

#[pymethods]
impl A440 {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        let inner = Arc::new(DawA440);
        PyClassInitializer::from(PitchStandard(inner.clone())).add_subclass(Self(inner))
    }
}

#[pyclass(extends = PitchStandard, subclass, module = "libdaw.pitch")]
pub struct ScientificPitch(pub Arc<DawScientificPitch>);

#[pymethods]
impl ScientificPitch {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        let inner = Arc::new(DawScientificPitch);
        PyClassInitializer::from(PitchStandard(inner.clone())).add_subclass(Self(inner))
    }
}

/// Helper to allow taking an optional pitch standard with a default value
#[derive(Debug)]
pub enum MaybePitchStandard<'py> {
    Py(PyRef<'py, PitchStandard>),

    Owned(DawA440),
}

impl<'py> FromPyObject<'py> for MaybePitchStandard<'py> {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let pitch_standard: Bound<'py, PitchStandard> = ob.extract()?;
        Ok(Self::Py(pitch_standard.borrow()))
    }
}

impl<'py> Deref for MaybePitchStandard<'py> {
    type Target = dyn DawPitchStandard;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybePitchStandard::Py(pitch_standard) => pitch_standard.0.deref(),
            MaybePitchStandard::Owned(a440) => a440,
        }
    }
}

impl<'a> Default for MaybePitchStandard<'a> {
    fn default() -> Self {
        MaybePitchStandard::Owned(DawA440)
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PitchStandard>()?;
    module.add_class::<A440>()?;
    module.add_class::<ScientificPitch>()?;
    module.add_class::<PitchClass>()?;
    module.add_class::<Pitch>()?;
    module.add_class::<PitchName>()?;
    Ok(())
}
