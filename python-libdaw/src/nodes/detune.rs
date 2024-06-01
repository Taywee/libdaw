use crate::Node;
use libdaw::nodes::Detune as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Detune(pub Arc<Mutex<Inner>>);

#[pymethods]
impl Detune {
    #[new]
    pub fn new(detune: Option<f64>) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner {
            detune: detune.unwrap_or(0.0),
        }));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }

    #[getter]
    pub fn get_detune(&self) -> f64 {
        self.0.lock().expect("poisoned").detune
    }

    #[setter]
    pub fn set_detune(&self, detune: f64) {
        self.0.lock().expect("poisoned").detune = detune;
    }
}
