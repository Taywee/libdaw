use crate::{
    nodes::envelope::Point,
    time::{Duration, Timestamp},
    Node, Result,
};
use libdaw::nodes::instrument;
use pyo3::{
    conversion::FromPyObject,
    pyclass, pymethods,
    types::{PyAny, PyAnyMethods as _, PyModule, PyModuleMethods as _},
    Bound, PyClassInitializer, PyObject, PyResult, PyTraverseError, PyVisit, Python,
};
use std::sync::{Arc, Mutex};

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

    #[getter]
    fn get_start(&self) -> Timestamp {
        Timestamp(self.0.start)
    }
    #[getter]
    fn get_length(&self) -> Duration {
        Duration(self.0.length)
    }
    #[getter]
    fn get_frequency(&self) -> f64 {
        self.0.frequency
    }
}

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Instrument {
    pub factory: Option<Arc<PyObject>>,
    pub inner: Arc<Mutex<instrument::Instrument>>,
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
        let factory = Arc::new(factory.unbind());
        let inner = {
            let factory = Arc::downgrade(&factory);
            Arc::new(Mutex::new(instrument::Instrument::new(
                sample_rate,
                move |tone| {
                    if let Some(factory) = factory.upgrade() {
                        Python::with_gil(|py| {
                            let factory = factory.bind(py);
                            Ok(Node::extract_bound(&factory.call1((Tone(tone),))?)?.0)
                        })
                    } else {
                        Err("factory no longer exists".into())
                    }
                },
                envelope.into_iter().map(|point| point.0),
            )))
        };
        Ok(
            PyClassInitializer::from(Node(inner.clone())).add_subclass(Self {
                inner,
                factory: Some(factory),
            }),
        )
    }

    pub fn add_tone(&self, tone: Tone) {
        self.inner.lock().expect("poisoned").add_tone(tone.0);
    }

    fn __traverse__(&self, visit: PyVisit<'_>) -> std::result::Result<(), PyTraverseError> {
        self.factory
            .as_ref()
            .map(|factory| visit.call(&**factory))
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
