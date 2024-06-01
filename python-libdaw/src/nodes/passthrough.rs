use crate::Node;
use libdaw::nodes::Passthrough as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Passthrough(pub Arc<Mutex<Inner>>);

#[pymethods]
impl Passthrough {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::default()));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
