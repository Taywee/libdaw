use crate::{Node, Sample};
use libdaw::Node as DawNode;
use pyo3::{
    pyclass, pymethods, types::PyAnyMethods as _, Py, PyAny, PyClassInitializer, PyResult,
    PyTraverseError, PyVisit, Python,
};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Inner {
    callable: Option<Py<PyAny>>,
}

impl DawNode for Inner {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [libdaw::Sample],
        outputs: &'c mut Vec<libdaw::Sample>,
    ) -> libdaw::Result<()> {
        if let Some(callable) = &self.callable {
            let result: PyResult<()> = Python::with_gil(|py| {
                let inputs: PyResult<Vec<Sample>> = inputs
                    .iter()
                    .map(|sample| Sample::new(sample.clone().into()))
                    .collect();
                let inputs = inputs?;
                let callable = callable.bind(py);
                let py_outputs: Vec<Sample> = callable.call1((inputs,))?.extract()?;
                outputs.extend(py_outputs.into_iter().map(|sample| sample.0));
                Ok(())
            });
            result?;
        } else {
            return Err("Can not run a custom node without a callable".into());
        }
        Ok(())
    }
}

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Custom(Arc<Mutex<Inner>>);

#[pymethods]
impl Custom {
    #[new]
    pub fn new(callable: Py<PyAny>) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner {
            callable: Some(callable),
        }));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }

    #[getter]
    fn get_callable(&self) -> Py<PyAny> {
        self.0
            .lock()
            .expect("poisoned")
            .callable
            .clone()
            .expect("cleared")
    }
    #[setter]
    fn set_callable(&self, callable: Py<PyAny>) {
        self.0.lock().expect("poisoned").callable = Some(callable);
    }
    fn __traverse__(&self, visit: PyVisit<'_>) -> std::result::Result<(), PyTraverseError> {
        self.0
            .lock()
            .expect("poisoned")
            .callable
            .as_ref()
            .map(|callable| visit.call(callable))
            .transpose()
            .and(Ok(()))
    }

    fn __clear__(&mut self) {
        self.0.lock().expect("poisoned").callable = None;
    }
}
