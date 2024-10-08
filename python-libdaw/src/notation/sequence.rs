use super::{Element, Item, ItemOrElement, StateMember};
use crate::indexing::{IndexOrSlice, InsertIndex, ItemOrSequence, PopIndex};
use libdaw::notation::Sequence as DawSequence;
use pyo3::{
    pyclass, pymethods, Bound, Py, PyClassInitializer, PyResult, PyTraverseError, PyVisit, Python,
};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Element, module = "libdaw.notation", sequence)]
#[derive(Debug, Clone)]
pub struct Sequence {
    pub inner: Arc<Mutex<DawSequence>>,
    pub items: Vec<Py<Item>>,
}

impl Sequence {
    pub fn from_inner(py: Python<'_>, inner: Arc<Mutex<DawSequence>>) -> Py<Self> {
        let items = inner
            .lock()
            .expect("poisoned")
            .items
            .iter()
            .cloned()
            .map(move |item| Item::from_inner(py, item).unbind())
            .collect();
        Py::new(
            py,
            PyClassInitializer::from(Element {
                inner: inner.clone(),
            })
            .add_subclass(Self { inner, items }),
        )
        .expect("Could not construct Py")
    }
}

#[pymethods]
impl Sequence {
    #[new]
    #[pyo3(signature = (items=None, state_member=None))]
    pub fn new(
        items: Option<Vec<ItemOrElement<'_>>>,
        state_member: Option<StateMember>,
    ) -> PyClassInitializer<Self> {
        let items = items.unwrap_or_default();
        let inner = Arc::new(Mutex::new(DawSequence {
            items: items
                .iter()
                .map(move |item| item.item.borrow().inner.clone())
                .collect(),
            state_member: state_member.map(Into::into),
        }));
        PyClassInitializer::from(Element {
            inner: inner.clone(),
        })
        .add_subclass(Self {
            inner,
            items: items.into_iter().map(|item| item.item.unbind()).collect(),
        })
    }

    #[staticmethod]
    pub fn loads(py: Python<'_>, source: String) -> crate::Result<Py<Self>> {
        Ok(Self::from_inner(py, Arc::new(Mutex::new(source.parse()?))))
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
    ) -> PyResult<ItemOrSequence<Py<Item>, Py<Self>>> {
        index.get(&self.items)?.map_sequence(move |items| {
            let inner_items = items
                .iter()
                .map(move |item| item.borrow(py).inner.clone())
                .collect();
            let lock = self.inner.lock().expect("poisoned");
            let inner = Arc::new(Mutex::new(DawSequence {
                state_member: lock.state_member,
                items: inner_items,
            }));
            Py::new(
                py,
                PyClassInitializer::from(Element {
                    inner: inner.clone(),
                })
                .add_subclass(Self { inner, items }),
            )
        })
    }
    pub fn __setitem__<'py>(
        &mut self,
        index: IndexOrSlice<'py>,
        value: ItemOrSequence<ItemOrElement<'py>>,
    ) -> PyResult<()> {
        let len = self.items.len();
        let mut userdata = (self.inner.lock().expect("poisoned"), &mut self.items);
        index.normalize(len)?.set(
            value,
            &mut userdata,
            move |(lock, items), index, value| {
                lock.items[index] = value.item.borrow().inner.clone();
                items[index] = value.item.unbind();
                Ok(())
            },
            move |(lock, items), index, value| {
                lock.items.insert(index, value.item.borrow().inner.clone());
                items.insert(index, value.item.unbind());
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

    pub fn append(&mut self, value: ItemOrElement<'_>) -> PyResult<()> {
        self.inner
            .lock()
            .expect("poisoned")
            .items
            .push(value.item.borrow().inner.clone());
        self.items.push(value.item.unbind());
        Ok(())
    }

    pub fn insert(&mut self, index: InsertIndex, value: ItemOrElement<'_>) -> PyResult<()> {
        let index = index.normalize(self.items.len())?;
        self.inner
            .lock()
            .expect("poisoned")
            .items
            .insert(index, value.item.borrow().inner.clone());
        self.items.insert(index, value.item.unbind());
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
pub struct SequenceIterator(pub std::vec::IntoIter<Py<Item>>);

#[pymethods]
impl SequenceIterator {
    pub fn __iter__(self_: Bound<'_, Self>) -> Bound<'_, Self> {
        self_
    }
    pub fn __repr__(&self) -> String {
        format!("SequenceIterator<{:?}>", self.0)
    }
    pub fn __next__(&mut self) -> Option<Py<Item>> {
        self.0.next()
    }
}
