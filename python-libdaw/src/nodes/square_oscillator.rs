use crate::{FrequencyNode, Node};
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::Arc;

#[pyclass(extends = FrequencyNode, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct SquareOscillator(pub Arc<::libdaw::nodes::SquareOscillator>);

#[pymethods]
impl SquareOscillator {
    #[new]
    #[pyo3(signature = (sample_rate = 48000, channels = 2))]
    pub fn new(sample_rate: u32, channels: u16) -> PyClassInitializer<Self> {
        let inner = Arc::new(::libdaw::nodes::SquareOscillator::new(
            sample_rate,
            channels,
        ));
        PyClassInitializer::from(Node(inner.clone()))
            .add_subclass(FrequencyNode(inner.clone()))
            .add_subclass(Self(inner))
    }
}
