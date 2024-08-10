use crate::{Node, Sample};
use libdaw::Node as DawNode;
use pyo3::{
    exceptions::PyRuntimeError, pyclass, pymethods, types::PyAnyMethods as _, Bound, IntoPy, Py,
    PyAny, PyClassInitializer, PyResult, PyTraverseError, PyVisit, Python,
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

/// A custom Node.
///
/// You can either pass a processing callable into this, assign it to its
/// `callable` property, or subclass this with a callable.  If you subclass
/// this, you **must** call super().__init__()
#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Custom(Arc<Mutex<Inner>>);

#[pymethods]
impl Custom {
    #[new]
    #[pyo3(signature = (callable = None))]
    pub fn new(callable: Option<Py<PyAny>>) -> PyClassInitializer<Self> {
        match callable {
            Some(callable) => {
                let inner = Arc::new(Mutex::new(Inner {
                    callable: Some(callable),
                }));
                PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
            }
            None => {
                // Need to construct as None first so it can refer to us.
                // A non-existant callable only works for the case where a
                // subclass is being used.  In this case, it should be set up in
                // the __init__ method.
                let inner = Arc::new(Mutex::new(Inner { callable: None }));
                PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner.clone()))
            }
        }
    }

    #[pyo3(signature = (callable = None))]
    pub fn __init__<'py>(self_: &Bound<'py, Self>, py: Python<'py>, callable: Option<Py<PyAny>>) {
        let inner = self_.borrow_mut().0.clone();
        let mut lock = inner.lock().expect("poisoned");
        match callable {
            Some(callable) => {
                lock.callable = Some(callable);
            }
            None => {
                lock.callable = Some(self_.clone().unbind().into_py(py));
            }
        }
    }

    #[getter]
    fn get_callable(&self) -> PyResult<Py<PyAny>> {
        let lock = self.0.lock().expect("poisoned");
        if let Some(callable) = &lock.callable {
            Ok(callable.clone())
        } else {
            Err(PyRuntimeError::new_err("Callable was None.  This probably means you forgot to set it or you forgot to call super().__init__() in your constructor.  Alternately, it could mean a bug in libdaw."))
        }
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
