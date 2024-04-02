use crate::Node;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::Arc;

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Gain(pub Arc<::libdaw::nodes::Gain>);

#[pymethods]
impl Gain {
    #[new]
    pub fn new(gain: f64) -> PyClassInitializer<Self> {
        let inner = Arc::new(::libdaw::nodes::Gain::new(gain));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }

    #[getter]
    pub fn get_gain(&self) -> f64 {
        self.0.get_gain()
    }
    #[setter]
    pub fn set_gain(&self, gain: f64) {
        self.0.set_gain(gain);
    }
}
