use crate::Node;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::Arc;

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct ConstantValue(pub Arc<::libdaw::nodes::ConstantValue>);

#[pymethods]
impl ConstantValue {
    #[new]
    pub fn new(channels: u16, value: f64) -> PyClassInitializer<Self> {
        let inner = Arc::new(::libdaw::nodes::ConstantValue::new(channels, value));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
