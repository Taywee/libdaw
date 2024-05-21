use crate::Node;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::Arc;

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct ConstantValue(pub Arc<::libdaw::nodes::ConstantValue>);

#[pymethods]
impl ConstantValue {
    #[new]
    #[pyo3(signature = (value, channels = 2))]
    pub fn new(value: f64, channels: u16) -> PyClassInitializer<Self> {
        let inner = Arc::new(::libdaw::nodes::ConstantValue::new(channels, value));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
