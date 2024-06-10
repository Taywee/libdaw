use crate::Node;
use libdaw::nodes::filters::butterworth::LowPass as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes.filters.butterworth")]
#[derive(Debug, Clone)]
pub struct LowPass(pub Arc<Mutex<Inner>>);

#[pymethods]
impl LowPass {
    #[new]
    #[pyo3(signature = (order, frequency, sample_rate = 48000))]
    pub fn new(
        order: usize,
        frequency: f64,
        sample_rate: u32,
    ) -> crate::Result<PyClassInitializer<Self>> {
        let inner = Arc::new(Mutex::new(Inner::new(sample_rate, order, frequency)?));
        Ok(PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner)))
    }
}
