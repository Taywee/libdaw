use crate::Node;
use libdaw::nodes::SawtoothOscillator as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct SawtoothOscillator(pub Arc<Mutex<Inner>>);

#[pymethods]
impl SawtoothOscillator {
    #[new]
    #[pyo3(signature = (sample_rate = 48000, channels = 2))]
    pub fn new(sample_rate: u32, channels: u16) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::new(sample_rate, channels)));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
