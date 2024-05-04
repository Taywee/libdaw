use pyo3::{
    exceptions::PyIndexError,
    pyclass, pymethods,
    types::{PyAnyMethods as _, PyInt},
    Bound, PyAny, PyResult,
};

#[derive(Debug, Clone)]
#[pyclass(sequence, module = "libdaw")]
pub struct Stream(pub ::libdaw::Stream);

impl Stream {
    /// Resolve a possibly-negative index into an adjusted one.
    /// This is still signed to make dealing with things like insert easier.
    fn resolve_index(&self, index: isize) -> PyResult<isize> {
        let len = isize::try_from(self.__len__())
            .map_err(|error| PyIndexError::new_err(error.to_string()))?;
        Ok(if index < 0 { len + index } else { index })
    }
}

#[pymethods]
impl Stream {
    #[new]
    pub fn new(value: Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(channels) = value.downcast::<PyInt>() {
            let channels = channels.extract()?;
            Ok(Self(::libdaw::Stream::new(channels)))
        } else {
            let values: Vec<f64> = value.extract()?;
            let mut inner = ::libdaw::Stream::new(values.len());
            for (l, r) in inner.iter_mut().zip(values) {
                *l = r;
            }
            Ok(Self(inner))
        }
    }

    pub fn __len__(&self) -> usize {
        self.0.channels()
    }

    pub fn __getitem__(&self, index: isize) -> PyResult<f64> {
        usize::try_from(self.resolve_index(index)?)
            .ok()
            .and_then(|index| self.0.get(index).copied())
            .ok_or_else(|| PyIndexError::new_err("Index out of range"))
    }
    pub fn __setitem__(&mut self, index: isize, value: f64) -> PyResult<()> {
        let slot = usize::try_from(self.resolve_index(index)?)
            .ok()
            .and_then(|index| self.0.get_mut(index))
            .ok_or_else(|| PyIndexError::new_err("Index out of range"))?;
        *slot = value;
        Ok(())
    }
    pub fn __repr__(&self) -> String {
        format!("Stream<{:?}>", &*self.0)
    }
    pub fn __str__(&self) -> String {
        format!("{:?}", &*self.0)
    }
    pub fn __add__(&self, other: &Bound<'_, Self>) -> Self {
        Stream(self.0 + other.borrow().0)
    }

    pub fn __iadd__(&mut self, other: &Bound<'_, Self>) {
        self.0 += other.borrow().0;
    }
    pub fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(other) = other.downcast::<Self>() {
            Ok(Stream(self.0 * other.borrow().0))
        } else {
            let other: f64 = other.extract()?;
            Ok(Stream(self.0 * other))
        }
    }

    pub fn __imul__(&mut self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(other) = other.downcast::<Self>() {
            self.0 *= other.borrow().0;
        } else {
            let other: f64 = other.extract()?;
            self.0 *= other;
        }
        Ok(())
    }

    pub fn __copy__(&self) -> Self {
        self.clone()
    }

    pub fn __deepcopy__(&self, _memo: &Bound<'_, PyAny>) -> Self {
        self.clone()
    }

    pub fn __iter__(&self) -> StreamIterator {
        StreamIterator(self.0.into_iter())
    }
}

#[derive(Debug, Clone)]
#[pyclass(sequence, module = "libdaw")]
pub struct StreamIterator(pub <::libdaw::Stream as IntoIterator>::IntoIter);

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
