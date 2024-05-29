use crate::indexing::{IndexOrSlice, InsertIndex, ItemOrSequence, PopIndex};

use super::NotePitch;
use libdaw::notation::Scale as DawScale;
use pyo3::{
    pyclass, pymethods, Bound, IntoPy as _, Py, PyResult, PyTraverseError, PyVisit, Python,
};
use std::sync::{Arc, Mutex};

#[pyclass(module = "libdaw.notation", sequence)]
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
            .pitches()
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
    pub fn new(py: Python<'_>, pitches: Vec<NotePitch>) -> crate::Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(DawScale::new(
                pitches
                    .iter()
                    .map(move |pitch| pitch.as_inner(py))
                    .collect(),
            )?)),
            pitches,
        })
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
    pub fn __getitem__(
        &self,
        py: Python<'_>,
        index: IndexOrSlice<'_>,
    ) -> PyResult<ItemOrSequence<NotePitch, Self>> {
        index.get(&self.pitches)?.map_sequence(move |pitches| {
            let inner_pitches = pitches
                .iter()
                .map(move |pitch| pitch.as_inner(py))
                .collect();
            let inner = Arc::new(Mutex::new(
                DawScale::new(inner_pitches).map_err(|e| crate::Error::new_err(e.to_string()))?,
            ));
            Ok(Self { inner, pitches })
        })
    }
    pub fn __setitem__(
        &mut self,
        py: Python<'_>,
        index: IndexOrSlice<'_>,
        value: ItemOrSequence<NotePitch>,
    ) -> PyResult<()> {
        let len = self.pitches.len();
        let mut userdata = (self.inner.lock().expect("poisoned"), &mut self.pitches);
        index.normalize(len)?.set(
            value,
            &mut userdata,
            move |(lock, pitches), index, value| {
                lock.pitches_mut()[index] = value.as_inner(py);
                pitches[index] = value;
                Ok(())
            },
            move |(lock, pitches), index, value| {
                lock.insert(index, value.as_inner(py));
                pitches.insert(index, value);
                Ok(())
            },
            move |(lock, pitches), range| {
                lock.drain(range.clone())
                    .map_err(|e| crate::Error::new_err(e.to_string()))?;
                pitches.drain(range);
                Ok(())
            },
        )
    }
    pub fn __delitem__(&mut self, index: IndexOrSlice<'_>) -> PyResult<()> {
        let len = self.pitches.len();
        let mut lock = self.inner.lock().expect("poisoned");
        let pitches = &mut self.pitches;
        index.normalize(len)?.delete(move |range| {
            lock.drain(range.clone())
                .map_err(|e| crate::Error::new_err(e.to_string()))?;
            pitches.drain(range);
            Ok(())
        })
    }

    pub fn __iter__(&self) -> ScaleIterator {
        ScaleIterator(self.pitches.clone().into_iter())
    }

    pub fn append(&mut self, py: Python<'_>, value: NotePitch) -> PyResult<()> {
        self.inner
            .lock()
            .expect("poisoned")
            .push(value.as_inner(py));
        self.pitches.push(value);
        Ok(())
    }

    pub fn insert(&mut self, py: Python<'_>, index: InsertIndex, value: NotePitch) -> PyResult<()> {
        let index = index.normalize(self.pitches.len())?;
        self.inner
            .lock()
            .expect("poisoned")
            .insert(index, value.as_inner(py));
        self.pitches.insert(index, value);
        Ok(())
    }
    #[pyo3(signature = (index = Default::default()))]
    pub fn pop(&mut self, index: PopIndex) -> PyResult<NotePitch> {
        let index = index.normalize(self.pitches.len())?;
        self.inner
            .lock()
            .expect("poisoned")
            .remove(index)
            .map_err(|e| crate::Error::new_err(e.to_string()))?;

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
        self.inner.lock().expect("poisoned").clear();
        self.pitches.clear();
    }
}

#[derive(Debug, Clone)]
#[pyclass(module = "libdaw.notation")]
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
