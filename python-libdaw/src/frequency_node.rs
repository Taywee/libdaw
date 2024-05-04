use super::Node;
use pyo3::{pyclass, pymethods};
use std::sync::Arc;

#[derive(Debug, Clone)]
#[pyclass(extends = Node, subclass, module = "libdaw")]
pub struct FrequencyNode(pub Arc<dyn ::libdaw::FrequencyNode>);

#[pymethods]
impl FrequencyNode {
    #[getter]
    pub fn get_frequency(&self) -> crate::Result<f64> {
        Ok(self.0.get_frequency()?)
    }
    #[setter]
    pub fn set_frequency(&self, value: f64) -> crate::Result<()> {
        Ok(self.0.set_frequency(value)?)
    }
}
