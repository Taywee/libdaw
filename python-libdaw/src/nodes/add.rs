use crate::Node;
use libdaw::nodes::Add as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Add(pub Arc<Mutex<Inner>>);

#[pymethods]
impl Add {
    #[new]
    #[pyo3(signature = (channels = 2))]
    pub fn new(channels: u16) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::new(channels)));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
