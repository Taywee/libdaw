use crate::{FrequencyNode, Node};
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::Arc;

#[pyclass(extends = FrequencyNode, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct MultiFrequency(pub Arc<libdaw::nodes::MultiFrequency>);

#[pymethods]
impl MultiFrequency {
    #[new]
    pub fn new(nodes: Vec<FrequencyNode>) -> PyClassInitializer<Self> {
        let inner = Arc::new(libdaw::nodes::MultiFrequency::new(
            nodes.into_iter().map(|node| node.0),
        ));
        PyClassInitializer::from(Node(inner.clone()))
            .add_subclass(FrequencyNode(inner.clone()))
            .add_subclass(Self(inner))
    }
}
