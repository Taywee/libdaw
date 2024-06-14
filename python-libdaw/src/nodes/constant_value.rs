use crate::Node;
use libdaw::nodes::ConstantValue as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct ConstantValue(pub Arc<Mutex<Inner>>);

#[pymethods]
impl ConstantValue {
    #[new]
    #[pyo3(signature = (value))]
    pub fn new(value: f64) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::new(value)));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
