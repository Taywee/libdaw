use crate::indexing::{IndexOrSlice, InsertIndex, ItemOrSequence, PopIndex};
use libdaw::Sample as DawStream;
use pyo3::{
    exceptions::PyValueError, pyclass, pymethods, types::PyAnyMethods as _, Bound, PyAny, PyResult,
};

#[derive(Debug, Clone)]
#[pyclass(module = "libdaw", sequence)]
pub struct Sample(pub DawStream);

#[pymethods]
impl Sample {
    #[new]
    pub fn new(channels: Vec<f64>) -> PyResult<Self> {
        Ok(Self(channels.into()))
    }

    pub fn __len__(&self) -> usize {
        self.0.len()
    }

    pub fn __getitem__(&self, index: IndexOrSlice<'_>) -> PyResult<ItemOrSequence<f64, Self>> {
        index.get(&self.0)?.map_sequence(|v| Ok(Self(v.into())))
    }
    pub fn __setitem__(
        &mut self,
        index: IndexOrSlice<'_>,
        value: ItemOrSequence<f64>,
    ) -> PyResult<()> {
        index.normalize(self.0.len())?.set(
            value,
            &mut self.0.channels,
            move |channels, index, value| {
                channels[index] = value;
                Ok(())
            },
            move |channels, index, value| {
                channels.insert(index, value);
                Ok(())
            },
            move |channels, range| {
                channels.drain(range);
                Ok(())
            },
        )
    }
    pub fn __delitem__(&mut self, index: IndexOrSlice<'_>) -> PyResult<()> {
        index.normalize(self.0.len())?.delete(move |range| {
            self.0.channels.drain(range);
            Ok(())
        })
    }
    pub fn __repr__(&self) -> String {
        format!("Sample<{:?}>", &*self.0)
    }
    pub fn __str__(&self) -> String {
        format!("{:?}", &*self.0)
    }
    pub fn __add__(&self, other: &Bound<'_, Self>) -> Self {
        Sample(&self.0 + &other.borrow().0)
    }

    pub fn __iadd__(&mut self, other: &Bound<'_, Self>) {
        self.0 += &other.borrow().0;
    }
    pub fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(other) = other.downcast::<Self>() {
            Ok(Sample(&self.0 * &other.borrow().0))
        } else {
            let other: f64 = other.extract()?;
            Ok(Sample(&self.0 * other))
        }
    }

    pub fn __imul__(&mut self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(other) = other.downcast::<Self>() {
            self.0 *= &other.borrow().0;
        } else {
            let other: f64 = other.extract()?;
            self.0 *= other;
        }
        Ok(())
    }

    pub fn __getnewargs__(&self) -> (Vec<f64>,) {
        (self.0.iter().copied().collect(),)
    }

    pub fn __iter__(&self) -> StreamIterator {
        StreamIterator(self.0.clone().into_iter())
    }

    pub fn append(&mut self, value: f64) {
        self.0.channels.push(value);
    }

    pub fn insert(&mut self, index: InsertIndex, value: f64) -> PyResult<()> {
        let index = index.normalize(self.0.channels.len())?;
        self.0.channels.insert(index, value);
        Ok(())
    }

    #[pyo3(signature = (index = Default::default()))]
    pub fn pop(&mut self, index: PopIndex) -> PyResult<f64> {
        let index = index.normalize(self.0.channels.len())?;

        Ok(self.0.channels.remove(index))
    }

    pub fn index(&self, value: f64) -> PyResult<usize> {
        self.0
            .iter()
            .copied()
            .enumerate()
            .find(|&(_, inner_value)| value == inner_value)
            .map(|(index, _)| index)
            .ok_or_else(|| PyValueError::new_err(format!("{value} is not in the stream")))
    }
}

#[derive(Debug, Clone)]
#[pyclass(module = "libdaw")]
pub struct StreamIterator(pub <DawStream as IntoIterator>::IntoIter);

#[pymethods]
impl StreamIterator {
    pub fn __iter__(self_: Bound<'_, Self>) -> Bound<'_, Self> {
        self_
    }
    pub fn __repr__(&self) -> String {
        format!("StreamIterator<{:?}>", self.0)
    }
    pub fn __next__(&mut self) -> Option<f64> {
        self.0.next()
    }
}
