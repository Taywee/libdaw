use crate::{FrequencyNode, Node, Result};
use pyo3::{pyclass, pymethods, Bound, PyClassInitializer};
use std::sync::Arc;

#[pyclass(extends = FrequencyNode, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Detune(pub Arc<::libdaw::nodes::Detune>);

#[pymethods]
impl Detune {
    #[new]
    pub fn new(node: Bound<'_, FrequencyNode>) -> PyClassInitializer<Self> {
        let inner = Arc::new(::libdaw::nodes::Detune::new(node.borrow().0.clone()));
        PyClassInitializer::from(Node(inner.clone()))
            .add_subclass(FrequencyNode(inner.clone()))
            .add_subclass(Self(inner))
    }

    #[getter]
    pub fn get_detune(&self) -> f64 {
        self.0.get_detune()
    }

    #[setter]
    pub fn set_detune(&self, detune: f64) -> Result<()> {
        self.0.set_detune(detune)?;
        Ok(())
    }
}
