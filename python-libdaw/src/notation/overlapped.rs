use super::{Item, ItemOrValue, StateMember};
use crate::{
    indexing::{IndexOrSlice, InsertIndex, ItemOrSequence, PopIndex},
    metronome::{Beat, MaybeMetronome},
    nodes::instrument::Tone,
    pitch::MaybePitchStandard,
};
use libdaw::{metronome::Beat as DawBeat, notation::Overlapped as DawOverlapped};
use pyo3::{
    pyclass, pymethods, Bound, IntoPy as _, Py, PyResult, PyTraverseError, PyVisit, Python,
};
use std::{
    ops::Deref as _,
    sync::{Arc, Mutex},
};

#[pyclass(module = "libdaw.notation", sequence)]
#[derive(Debug, Clone)]
pub struct Overlapped {
    pub inner: Arc<Mutex<DawOverlapped>>,
    pub items: Vec<Py<Item>>,
}

impl Overlapped {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawOverlapped>>) -> Py<Self> {
        let items = inner
            .lock()
            .expect("poisoned")
            .items
            .iter()
            .cloned()
            .map(move |item| Item::from_inner(py, item).unbind())
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
impl Overlapped {
    #[new]
    #[pyo3(signature = (items=None, state_member=None))]
    pub fn new(items: Option<Vec<ItemOrValue<'_>>>, state_member: Option<StateMember>) -> Self {
        let items = items.unwrap_or_default();
        Self {
            inner: Arc::new(Mutex::new(DawOverlapped {
                items: items
                    .iter()
                    .map(move |item| item.0.borrow().inner.clone())
                    .collect(),
                state_member: state_member.map(Into::into),
            })),
            items: items.into_iter().map(|item| item.0.unbind()).collect(),
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
        let metronome = MaybeMetronome::from(metronome);
        let pitch_standard = MaybePitchStandard::from(pitch_standard);
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

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned"))
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned"))
    }

    pub fn __len__(&self) -> usize {
        self.items.len()
    }
    pub fn __getitem__(
        &self,
        py: Python<'_>,
        index: IndexOrSlice<'_>,
    ) -> PyResult<ItemOrSequence<Py<Item>, Self>> {
        index.get(&self.items)?.map_sequence(move |items| {
            let inner_items = items
                .iter()
                .map(move |item| item.borrow(py).inner.clone())
                .collect();
            let lock = self.inner.lock().expect("poisoned");
            let inner = Arc::new(Mutex::new(DawOverlapped {
                state_member: lock.state_member,
                items: inner_items,
            }));
            Ok(Self { inner, items })
        })
    }
    pub fn __setitem__<'py>(
        &mut self,
        index: IndexOrSlice<'py>,
        value: ItemOrSequence<ItemOrValue<'py>>,
    ) -> PyResult<()> {
        let len = self.items.len();
        let mut userdata = (self.inner.lock().expect("poisoned"), &mut self.items);
        index.normalize(len)?.set(
            value,
            &mut userdata,
            move |(lock, items), index, value| {
                lock.items[index] = value.0.borrow().inner.clone();
                items[index] = value.0.unbind();
                Ok(())
            },
            move |(lock, items), index, value| {
                lock.items.insert(index, value.0.borrow().inner.clone());
                items.insert(index, value.0.unbind());
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
    pub fn __iter__(&self) -> OverlappedIterator {
        OverlappedIterator(self.items.clone().into_iter())
    }

    pub fn append(&mut self, value: ItemOrValue<'_>) -> PyResult<()> {
        self.inner
            .lock()
            .expect("poisoned")
            .items
            .push(value.0.borrow().inner.clone());
        self.items.push(value.0.unbind());
        Ok(())
    }

    pub fn insert(&mut self, index: InsertIndex, value: ItemOrValue<'_>) -> PyResult<()> {
        let index = index.normalize(self.items.len())?;
        self.inner
            .lock()
            .expect("poisoned")
            .items
            .insert(index, value.0.borrow().inner.clone());
        self.items.insert(index, value.0.unbind());
        Ok(())
    }

    #[pyo3(signature = (index = Default::default()))]
    pub fn pop(&mut self, index: PopIndex) -> PyResult<Py<Item>> {
        let index = index.normalize(self.items.len())?;
        self.inner.lock().expect("poisoned").items.remove(index);
        Ok(self.items.remove(index))
    }

    pub fn __getnewargs__(&self) -> (Vec<Py<Item>>, Option<StateMember>) {
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
pub struct OverlappedIterator(pub std::vec::IntoIter<Py<Item>>);

#[pymethods]
impl OverlappedIterator {
    pub fn __iter__(self_: Bound<'_, Self>) -> Bound<'_, Self> {
        self_
    }
    pub fn __repr__(&self) -> String {
        format!("OverlappedIterator<{:?}>", self.0)
    }
    pub fn __next__(&mut self) -> Option<Py<Item>> {
        self.0.next()
    }
}
