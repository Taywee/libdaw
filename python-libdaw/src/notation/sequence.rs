use super::{Item, StateMember};
use crate::{
    indexing::{IndexOrSlice, InsertIndex, ItemOrSequence, PopIndex},
    metronome::{Beat, MaybeMetronome},
    nodes::instrument::Tone,
    pitch::MaybePitchStandard,
};
use libdaw::{metronome::Beat as DawBeat, notation::Sequence as DawSequence};
use pyo3::{pyclass, pymethods, Bound, IntoPy, Py, PyResult, PyTraverseError, PyVisit, Python};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.notation", sequence)]
#[derive(Debug, Clone)]
pub struct Sequence {
    pub inner: Arc<Mutex<DawSequence>>,
    pub items: Vec<Item>,
}

impl Sequence {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawSequence>>) -> Py<Self> {
        let items = inner
            .lock()
            .expect("poisoned")
            .items
            .iter()
            .cloned()
            .map(move |item| Item::from_inner(py, item))
            .collect();
        Self { inner, items }
            .into_py(py)
            .downcast_bound(py)
            .unwrap()
            .clone()
            .unbind()
    }
}

#[pymethods]
impl Sequence {
    #[new]
    pub fn new(
        py: Python<'_>,
        items: Option<Vec<Item>>,
        state_member: Option<StateMember>,
    ) -> Self {
        let items = items.unwrap_or_default();
        Self {
            inner: Arc::new(Mutex::new(DawSequence {
                items: items.iter().map(move |item| item.as_inner(py)).collect(),
                state_member: state_member.map(Into::into),
            })),
            items,
        }
    }

    #[staticmethod]
    pub fn loads(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
    }

    #[pyo3(
        signature = (
            *,
            offset=Beat(DawBeat::ZERO),
            metronome=MaybeMetronome::default(),
            pitch_standard=MaybePitchStandard::default(),
        )
    )]
    pub fn tones(
        &self,
        offset: Beat,
        metronome: MaybeMetronome,
        pitch_standard: MaybePitchStandard,
    ) -> Vec<Tone> {
        self.inner
            .lock()
            .expect("poisoned")
            .tones(offset.0, &metronome, pitch_standard.deref())
            .map(Tone)
            .collect()
    }
    #[getter]
    pub fn get_state_member(&self) -> Option<StateMember> {
        self.inner
            .lock()
            .expect("poisoned")
            .state_member
            .map(Into::into)
    }
    #[setter]
    pub fn set_state_member(&mut self, value: Option<StateMember>) {
        self.inner.lock().expect("poisoned").state_member = value.map(Into::into);
    }

    pub fn length(&self) -> Beat {
        Beat(self.inner.lock().expect("poisoned").length())
    }

    pub fn duration(&self) -> Beat {
        Beat(self.inner.lock().expect("poisoned").duration())
    }

    pub fn __len__(&self) -> usize {
        self.items.len()
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned"))
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned"))
    }

    pub fn __getitem__(
        &self,
        py: Python<'_>,
        index: IndexOrSlice<'_>,
    ) -> PyResult<ItemOrSequence<Item, Self>> {
        index.get(&self.items)?.map_sequence(move |items| {
            let inner_items = items.iter().map(move |item| item.as_inner(py)).collect();
            let lock = self.inner.lock().expect("poisoned");
            let inner = Arc::new(Mutex::new(DawSequence {
                state_member: lock.state_member,
                items: inner_items,
            }));
            Ok(Self { inner, items })
        })
    }
    pub fn __setitem__(
        &mut self,
        py: Python<'_>,
        index: IndexOrSlice<'_>,
        value: ItemOrSequence<Item>,
    ) -> PyResult<()> {
        let len = self.items.len();
        let mut userdata = (self.inner.lock().expect("poisoned"), &mut self.items);
        index.normalize(len)?.set(
            value,
            &mut userdata,
            move |(lock, items), index, value| {
                lock.items[index] = value.as_inner(py);
                items[index] = value;
                Ok(())
            },
            move |(lock, items), index, value| {
                lock.items.insert(index, value.as_inner(py));
                items.insert(index, value);
                Ok(())
            },
            move |(lock, items), range| {
                lock.items.drain(range.clone());
                items.drain(range);
                Ok(())
            },
        )
    }
    pub fn __delitem__(&mut self, index: IndexOrSlice<'_>) -> PyResult<()> {
        let len = self.items.len();
        let mut lock = self.inner.lock().expect("poisoned");
        let items = &mut self.items;
        index.normalize(len)?.delete(move |range| {
            lock.items.drain(range.clone());
            items.drain(range);
            Ok(())
        })
    }

    pub fn __iter__(&self) -> SequenceIterator {
        SequenceIterator(self.items.clone().into_iter())
    }

    pub fn append(&mut self, py: Python<'_>, value: Item) -> PyResult<()> {
        self.inner
            .lock()
            .expect("poisoned")
            .items
            .push(value.as_inner(py));
        self.items.push(value);
        Ok(())
    }

    pub fn insert(&mut self, py: Python<'_>, index: InsertIndex, value: Item) -> PyResult<()> {
        let index = index.normalize(self.items.len())?;
        self.inner
            .lock()
            .expect("poisoned")
            .items
            .insert(index, value.as_inner(py));
        self.items.insert(index, value);
        Ok(())
    }

    #[pyo3(signature = (index = Default::default()))]
    pub fn pop(&mut self, index: PopIndex) -> PyResult<Item> {
        let index = index.normalize(self.items.len())?;
        self.inner.lock().expect("poisoned").items.remove(index);
        Ok(self.items.remove(index))
    }
    pub fn __getnewargs__(&self) -> (Vec<Item>, Option<StateMember>) {
        (
            self.items.clone(),
            self.inner
                .lock()
                .expect("poisoned")
                .state_member
                .map(Into::into),
        )
    }
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        for item in &self.items {
            visit.call(item)?
        }
        Ok(())
    }

    pub fn __clear__(&mut self) {
        self.inner.lock().expect("poisoned").items.clear();
        self.items.clear();
    }
}

#[derive(Debug, Clone)]
#[pyclass(module = "libdaw.notation")]
pub struct SequenceIterator(pub std::vec::IntoIter<Item>);

#[pymethods]
impl SequenceIterator {
    pub fn __iter__(self_: Bound<'_, Self>) -> Bound<'_, Self> {
        self_
    }
    pub fn __repr__(&self) -> String {
        format!("SequenceIterator<{:?}>", self.0)
    }
    pub fn __next__(&mut self) -> Option<Item> {
        self.0.next()
    }
}
