use crate::{Result, Sample};
use pyo3::{pyclass, pymethods, Bound};
use std::sync::Arc;

#[derive(Debug, Clone)]
#[pyclass(subclass, module = "libdaw")]
pub struct Node(pub Arc<dyn ::libdaw::Node>);

#[pymethods]
impl Node {
    pub fn process(&self, inputs: Vec<Bound<'_, Sample>>) -> Result<Vec<Sample>> {
        let mut outputs = Vec::new();
        let inputs: Vec<_> = inputs.into_iter().map(|i| i.borrow().0.clone()).collect();
        self.0.process(&inputs, &mut outputs)?;
        let outputs: Vec<_> = outputs.into_iter().map(Sample).collect();
        Ok(outputs)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", (&*self.0))
    }

    pub fn __iter__(self_: Bound<'_, Node>) -> Bound<'_, Node> {
        self_
    }

    pub fn __next__(&self) -> Result<Option<Vec<Sample>>> {
        match (&*self.0).next() {
            Some(outputs) => Ok(Some(outputs?.into_iter().map(Sample).collect())),
            None => Ok(None),
        }
    }
}
