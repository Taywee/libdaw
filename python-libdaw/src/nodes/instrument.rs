use crate::{
    nodes::envelope::Point,
    time::{Duration, Timestamp},
    FrequencyNode, Node, Result,
};
use libdaw::nodes::instrument;
use pyo3::{
    conversion::FromPyObject,
    pyclass, pymethods,
    types::{PyAny, PyAnyMethods as _, PyModule, PyModuleMethods as _},
    Bound, PyClassInitializer, PyObject, PyResult, PyTraverseError, PyVisit, Python,
};
use std::sync::Arc;

#[pyclass(module = "libdaw.nodes.instrument")]
#[derive(Debug, Clone, Copy)]
pub struct Tone(pub instrument::Tone);

#[pymethods]
impl Tone {
    #[new]
    pub fn new(start: Timestamp, length: Duration, frequency: f64) -> Self {
        Tone(instrument::Tone {
            start: start.0,
            length: length.0,
            frequency,
        })
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Instrument {
    pub factory: Option<PyObject>,
    pub inner: Arc<instrument::Instrument>,
}

#[pymethods]
impl Instrument {
    #[new]
    #[pyo3(signature = (factory, envelope, sample_rate = 48000))]
    pub fn new(
        factory: Bound<'_, PyAny>,
        envelope: Vec<Point>,
        sample_rate: u32,
    ) -> Result<PyClassInitializer<Self>> {
        if !factory.is_callable() {
            return Err("factory must be a callable".into());
        }
        let factory = factory.unbind();
        let inner = {
            let factory = factory.clone();
            Arc::new(libdaw::nodes::Instrument::new(
                sample_rate,
                move || {
                    Python::with_gil(|py| {
                        let factory = factory.bind(py);
                        Ok(FrequencyNode::extract_bound(&factory.call0()?)?.0)
                    })
                },
                envelope.into_iter().map(|point| point.0),
            ))
        };
        Ok(
            PyClassInitializer::from(Node(inner.clone())).add_subclass(Self {
                inner,
                factory: Some(factory),
            }),
        )
    }

    pub fn add_tone(&self, tone: Tone) {
        self.inner.add_tone(tone.0);
    }

    pub fn set_detune(&self, detune: f64) -> Result<()> {
        self.inner.set_detune(detune)?;
        Ok(())
    }

    fn __traverse__(&self, visit: PyVisit<'_>) -> std::result::Result<(), PyTraverseError> {
        self.factory
            .as_ref()
            .map(|factory| visit.call(factory))
            .transpose()
            .and(Ok(()))
    }

    fn __clear__(&mut self) {
        self.factory = None;
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Tone>()?;
    Ok(())
}
