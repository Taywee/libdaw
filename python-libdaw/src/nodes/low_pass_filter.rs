use crate::Node;
use libdaw::nodes::LowPassFilter as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct LowPassFilter(pub Arc<Mutex<Inner>>);

#[pymethods]
impl LowPassFilter {
    #[new]
    #[pyo3(signature = (frequency, sample_rate = 48000))]
    pub fn new(frequency: f64, sample_rate: u32) -> crate::Result<PyClassInitializer<Self>> {
        let inner = Arc::new(Mutex::new(Inner::new(sample_rate, frequency)?));
        Ok(PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner)))
    }
}
