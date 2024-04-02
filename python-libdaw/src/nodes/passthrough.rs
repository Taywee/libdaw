use crate::Node;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::Arc;

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Passthrough(pub Arc<::libdaw::nodes::Passthrough>);

#[pymethods]
impl Passthrough {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        let inner = Arc::new(::libdaw::nodes::Passthrough::default());
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
