use crate::{Result, Sample};
use libdaw::Node as Inner;
use pyo3::{pyclass, pymethods, Bound, PyResult};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
#[pyclass(subclass, module = "libdaw")]
pub struct Node(pub Arc<Mutex<dyn Inner>>);

#[pymethods]
impl Node {
    pub fn process(&self, inputs: Vec<Bound<'_, Sample>>) -> Result<Vec<Sample>> {
        let mut outputs = Vec::new();
        let inputs: Vec<_> = inputs.into_iter().map(|i| i.borrow().0.clone()).collect();
        self.0
            .lock()
            .expect("poisoned")
            .process(&inputs, &mut outputs)?;
        let outputs: Vec<_> = outputs.into_iter().map(Sample).collect();
        Ok(outputs)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", (&*self.0))
    }

    pub fn __iter__(self_: Bound<'_, Node>) -> Bound<'_, Node> {
        self_
    }

    pub fn __next__(&self) -> PyResult<Option<Vec<Sample>>> {
        let mut lock = self.0.lock().expect("poisoned");
        let mut dynref: &mut dyn libdaw::Node = &mut *lock;
        match dynref.next() {
            Some(outputs) => Ok(Some(
                outputs
                    .map_err(|e| crate::Error::new_err(e.to_string()))?
                    .into_iter()
                    .map(Sample)
                    .collect(),
            )),
            None => Ok(None),
        }
    }
}
