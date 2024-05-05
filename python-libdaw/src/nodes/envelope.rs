use crate::{
    time::{Duration, Time},
    Node,
};
use libdaw::nodes::envelope;
use pyo3::{
    pyclass, pymethods,
    types::{PyAnyMethods as _, PyModule, PyModuleMethods as _},
    Bound, FromPyObject, PyAny, PyClassInitializer, PyResult,
};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Default)]
pub struct Offset(pub envelope::Offset);

impl<'py> FromPyObject<'py> for Offset {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(if let Ok(delta) = ob.downcast::<Time>() {
            Self(envelope::Offset::Time(delta.borrow().0))
        } else {
            Self(envelope::Offset::Ratio(ob.extract()?))
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy, Default)]
pub struct Point(pub envelope::Point);

#[pymethods]
impl Point {
    #[new]
    pub fn new(whence: f64, volume: f64, offset: Option<Offset>) -> Self {
        Point(envelope::Point {
            offset: offset.unwrap_or_default().0,
            whence,
            volume,
        })
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Envelope(pub Arc<envelope::Envelope>);

#[pymethods]
impl Envelope {
    #[new]
    pub fn new(
        sample_rate: u32,
        length: Duration,
        envelope: Vec<Point>,
    ) -> PyClassInitializer<Self> {
        let inner = Arc::new(libdaw::nodes::Envelope::new(
            sample_rate,
            length.0,
            envelope.into_iter().map(|point| point.0),
        ));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Point>()?;
    Ok(())
}