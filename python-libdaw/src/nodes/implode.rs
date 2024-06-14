use crate::Node;
use libdaw::nodes::Implode as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Implode(pub Arc<Mutex<Inner>>);

#[pymethods]
impl Implode {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::default()));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
