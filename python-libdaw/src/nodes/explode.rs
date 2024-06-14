use crate::Node;
use libdaw::nodes::Explode as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Explode(pub Arc<Mutex<Inner>>);

#[pymethods]
impl Explode {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::default()));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
