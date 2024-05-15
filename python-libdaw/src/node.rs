use crate::{Result, Stream};
use pyo3::{pyclass, pymethods, Bound};
use std::{
    hash::{DefaultHasher, Hasher},
    sync::Arc,
};

#[derive(Debug, Clone)]
#[pyclass(subclass, module = "libdaw")]
pub struct Node(pub Arc<dyn ::libdaw::Node>);

#[pymethods]
impl Node {
    pub fn process(&self, inputs: Vec<Bound<'_, Stream>>) -> Result<Vec<Stream>> {
        let mut outputs = Vec::new();
        let inputs: Vec<_> = inputs.into_iter().map(|i| i.borrow().0.clone()).collect();
        self.0.process(&inputs, &mut outputs)?;
        let outputs: Vec<_> = outputs.into_iter().map(Stream).collect();
        Ok(outputs)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", (&*self.0))
    }

    pub fn __eq__(&self, other: Bound<'_, Self>) -> bool {
        std::ptr::addr_eq(Arc::as_ptr(&self.0), Arc::as_ptr(&other.borrow().0))
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        std::ptr::hash(Arc::as_ptr(&self.0), &mut hasher);
        hasher.finish()
    }
}
