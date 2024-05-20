use super::NotePitch;
use crate::{resolve_index, resolve_index_for_insert};
use libdaw::notation::Scale as DawScale;
use pyo3::{
    exceptions::PyIndexError, pyclass, pymethods, Bound, IntoPy as _, Py, PyResult,
    PyTraverseError, PyVisit, Python,
};
use std::sync::{Arc, Mutex};

#[pyclass(module = "libdaw.notation")]
#[derive(Debug, Clone)]
pub struct Scale {
    pub inner: Arc<Mutex<DawScale>>,
    pub pitches: Vec<NotePitch>,
}

impl Scale {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawScale>>) -> Py<Self> {
        let pitches = inner
            .lock()
            .expect("poisoned")
            .pitches
            .iter()
            .cloned()
            .map(move |pitch| NotePitch::from_inner(py, pitch))
            .collect();
        Self { inner, pitches }
            .into_py(py)
            .downcast_bound(py)
            .unwrap()
            .clone()
            .unbind()
    }
}

#[pymethods]
impl Scale {
    #[new]
    pub fn new(py: Python<'_>, pitches: Option<Vec<NotePitch>>) -> Self {
        let pitches = pitches.unwrap_or_default();
        Self {
            inner: Arc::new(Mutex::new(DawScale {
                pitches: pitches
                    .iter()
                    .map(move |pitch| pitch.as_inner(py))
                    .collect(),
            })),
            pitches,
        }
    }
    #[staticmethod]
    pub fn loads(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned"))
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned"))
    }

    pub fn __len__(&self) -> usize {
        self.pitches.len()
    }
    pub fn __getitem__(&self, index: isize) -> PyResult<NotePitch> {
        let index = resolve_index(self.pitches.len(), index)?;
        Ok(self.pitches[index].clone())
    }
    pub fn __setitem__(&mut self, py: Python<'_>, index: isize, value: NotePitch) -> PyResult<()> {
        let index = resolve_index(self.pitches.len(), index)?;
        self.inner.lock().expect("poisoned").pitches[index] = value.as_inner(py);
        self.pitches[index] = value;
        Ok(())
    }
    pub fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        self.pop(Some(index)).map(|_| ())
    }

    pub fn __iter__(&self) -> ScaleIterator {
        ScaleIterator(self.pitches.clone().into_iter())
    }

    pub fn append(&mut self, py: Python<'_>, value: NotePitch) -> PyResult<()> {
        self.inner
            .lock()
            .expect("poisoned")
            .pitches
            .push(value.as_inner(py));
        self.pitches.push(value);
        Ok(())
    }

    pub fn insert(&mut self, py: Python<'_>, index: isize, value: NotePitch) -> PyResult<()> {
        let index = resolve_index_for_insert(self.pitches.len(), index)?;
        self.inner
            .lock()
            .expect("poisoned")
            .pitches
            .insert(index, value.as_inner(py));
        self.pitches.insert(index, value);
        Ok(())
    }

    pub fn pop(&mut self, index: Option<isize>) -> PyResult<NotePitch> {
        let len = self.pitches.len();
        if len == 0 {
            return Err(PyIndexError::new_err("Pop from empty"));
        }
        let index = match index {
            Some(index) => resolve_index(len, index)?,
            None => len - 1,
        };
        self.inner.lock().expect("poisoned").pitches.remove(index);
        Ok(self.pitches.remove(index))
    }
    pub fn __getnewargs__(&self) -> (Vec<NotePitch>,) {
        (self.pitches.clone(),)
    }

    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        for pitch in &self.pitches {
            visit.call(pitch)?
        }
        Ok(())
    }

    pub fn __clear__(&mut self) {
        self.inner.lock().expect("poisoned").pitches.clear();
        self.pitches.clear();
    }
}

#[derive(Debug, Clone)]
#[pyclass(sequence, module = "libdaw.notation")]
pub struct ScaleIterator(pub std::vec::IntoIter<NotePitch>);

#[pymethods]
impl ScaleIterator {
    pub fn __iter__(self_: Bound<'_, Self>) -> Bound<'_, Self> {
        self_
    }
    pub fn __repr__(&self) -> String {
        format!("ScaleIterator<{:?}>", self.0)
    }
    pub fn __next__(&mut self) -> Option<NotePitch> {
        self.0.next()
    }
}
