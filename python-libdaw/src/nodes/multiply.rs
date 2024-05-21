use crate::Node;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::Arc;

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Multiply(pub Arc<::libdaw::nodes::Multiply>);

#[pymethods]
impl Multiply {
    #[new]
    #[pyo3(signature = (channels = 2))]
    pub fn new(channels: u16) -> PyClassInitializer<Self> {
        let inner = Arc::new(::libdaw::nodes::Multiply::new(channels));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
