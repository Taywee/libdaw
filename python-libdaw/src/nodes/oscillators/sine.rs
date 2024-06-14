use crate::Node;
use libdaw::nodes::oscillators::Sine as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes.oscillators")]
#[derive(Debug, Clone)]
pub struct Sine(pub Arc<Mutex<Inner>>);

#[pymethods]
impl Sine {
    #[new]
    #[pyo3(signature = (sample_rate = 48000, frequency = 0.0))]
    pub fn new(sample_rate: u32, frequency: f64) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::new(sample_rate, frequency)));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
    #[getter]
    pub fn get_frequency(&self) -> f64 {
        self.0.lock().expect("poisoned").frequency
    }

    #[setter]
    pub fn set_frequency(&self, frequency: f64) {
        self.0.lock().expect("poisoned").frequency = frequency;
    }
}
