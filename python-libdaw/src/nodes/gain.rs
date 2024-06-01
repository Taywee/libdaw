use crate::Node;
use libdaw::nodes::Gain as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Gain(pub Arc<Mutex<Inner>>);

#[pymethods]
impl Gain {
    #[new]
    pub fn new(gain: f64) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::new(gain)));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }

    #[getter]
    pub fn get_gain(&self) -> f64 {
        self.0.lock().expect("poisoned").gain
    }
    #[setter]
    pub fn set_gain(&self, gain: f64) {
        self.0.lock().expect("poisoned").gain = gain;
    }
}
