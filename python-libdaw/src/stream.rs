use crate::resolve_index;
use libdaw::Stream as DawStream;
use pyo3::{
    pyclass, pymethods,
    types::{PyAnyMethods as _, PyInt},
    Bound, PyAny, PyResult,
};

#[derive(Debug, Clone)]
#[pyclass(sequence, module = "libdaw")]
pub struct Stream(pub DawStream);

#[pymethods]
impl Stream {
    #[new]
    pub fn new(value: Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(channels) = value.downcast::<PyInt>() {
            let channels = channels.extract()?;
            Ok(Self(DawStream::new(channels)))
        } else {
            let values: Vec<f64> = value.extract()?;
            Ok(Self(values.into()))
        }
    }

    pub fn __len__(&self) -> usize {
        self.0.channels()
    }

    pub fn __getitem__(&self, index: isize) -> PyResult<f64> {
        let index = resolve_index(self.__len__(), index)?;
        Ok(self.0[index])
    }
    pub fn __setitem__(&mut self, index: isize, value: f64) -> PyResult<()> {
        let index = resolve_index(self.__len__(), index)?;
        self.0[index] = value;
        Ok(())
    }
    pub fn __repr__(&self) -> String {
        format!("Stream<{:?}>", &*self.0)
    }
    pub fn __str__(&self) -> String {
        format!("{:?}", &*self.0)
    }
    pub fn __add__(&self, other: &Bound<'_, Self>) -> Self {
        Stream(&self.0 + &other.borrow().0)
    }

    pub fn __iadd__(&mut self, other: &Bound<'_, Self>) {
        self.0 += &other.borrow().0;
    }
    pub fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(other) = other.downcast::<Self>() {
            Ok(Stream(&self.0 * &other.borrow().0))
        } else {
            let other: f64 = other.extract()?;
            Ok(Stream(&self.0 * other))
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
}

#[derive(Debug, Clone)]
#[pyclass(sequence, module = "libdaw")]
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
