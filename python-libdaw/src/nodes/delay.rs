use crate::{time::Duration, Node};
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::Arc;

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Delay(pub Arc<::libdaw::nodes::Delay>);

#[pymethods]
impl Delay {
    #[new]
    #[pyo3(signature = (delay, sample_rate = 48000))]
    pub fn new(delay: Duration, sample_rate: u32) -> PyClassInitializer<Self> {
        let inner = Arc::new(::libdaw::nodes::Delay::new(sample_rate, delay.0));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
