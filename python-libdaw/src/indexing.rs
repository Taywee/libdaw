use pyo3::{
    exceptions::{PyIndexError, PyTypeError, PyValueError},
    types::{PyAnyMethods as _, PySlice, PySliceIndices, PySliceMethods as _, PyTypeMethods as _},
    Bound, FromPyObject, IntoPy, Py, PyAny, PyResult, Python,
};
use std::{ffi::c_long, ops::Range};

#[derive(Debug, Clone, Copy)]
pub struct Index {
    index: isize,
}

impl<'py> FromPyObject<'py> for Index {
    fn extract_bound(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            index: value.extract()?,
        })
    }
}

impl Index {
    pub fn normalize(self, len: usize) -> PyResult<usize> {
        let index = self.index;
        let len = isize::try_from(len).map_err(|error| PyIndexError::new_err(error.to_string()))?;
        let index = if index < 0 { len + index } else { index };

        usize::try_from(index).map_err(|error| PyIndexError::new_err(error.to_string()))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InsertIndex {
    index: isize,
}

impl<'py> FromPyObject<'py> for InsertIndex {
    fn extract_bound(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            index: value.extract()?,
        })
    }
}

impl InsertIndex {
    pub fn normalize(self, len: usize) -> PyResult<usize> {
        let index = self.index;
        let len = isize::try_from(len).map_err(|error| PyIndexError::new_err(error.to_string()))?;
        let index = if index < 0 { len + index } else { index };
        Ok(index.clamp(0, len) as usize)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct PopIndex {
    index: Option<Index>,
}

impl<'py> FromPyObject<'py> for PopIndex {
    fn extract_bound(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            index: value.extract()?,
        })
    }
}

impl PopIndex {
    pub fn normalize(self, len: usize) -> PyResult<usize> {
        if len == 0 {
            return Err(PyIndexError::new_err("Pop from empty"));
        }
        self.index
            .map_or_else(|| Ok(len - 1), |index| index.normalize(len))
    }
}

#[derive(Debug, Clone)]
pub enum IndexOrSlice<'py> {
    Index(Index),
    Slice(Bound<'py, PySlice>),
}

impl<'py> IndexOrSlice<'py> {
    pub fn normalize(&self, len: usize) -> PyResult<NormalizedIndexOrSlice> {
        match self {
            Self::Index(index) => (*index).normalize(len).map(NormalizedIndexOrSlice::Index),
            Self::Slice(slice) => slice
                .indices(len as c_long)
                .map(NormalizedIndexOrSlice::Slice),
        }
    }
    pub fn get<T>(&self, collection: &[T]) -> PyResult<ItemOrSequence<T>>
    where
        T: Clone,
    {
        self.normalize(collection.len())
            .and_then(move |index| index.get(collection))
    }
}

impl<'py> FromPyObject<'py> for IndexOrSlice<'py> {
    fn extract_bound(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(if let Ok(slice) = value.downcast::<PySlice>() {
            Self::Slice(slice.clone())
        } else if let Ok(index) = value.extract::<Index>() {
            Self::Index(index)
        } else {
            let type_ = value.get_type();
            let type_name = type_.name()?;
            return Err(PyTypeError::new_err(format!(
                "index must be int or slice, not {type_name}"
            )));
        })
    }
}

#[derive(Debug)]
pub enum NormalizedIndexOrSlice {
    Index(usize),
    Slice(PySliceIndices),
}

impl Clone for NormalizedIndexOrSlice {
    fn clone(&self) -> Self {
        match self {
            Self::Index(index) => Self::Index(*index),
            Self::Slice(PySliceIndices {
                start,
                stop,
                step,
                slicelength,
            }) => Self::Slice(PySliceIndices {
                start: *start,
                stop: *stop,
                step: *step,
                slicelength: *slicelength,
            }),
        }
    }
}

impl NormalizedIndexOrSlice {
    pub fn get<T>(self, collection: &[T]) -> PyResult<ItemOrSequence<T>>
    where
        T: Clone,
    {
        Ok(match self {
            Self::Index(index) => ItemOrSequence::Item(collection[index].clone()),
            Self::Slice(indices) => {
                let start = indices.start as usize;
                let stop = indices.stop as usize;
                let slicelength = indices.slicelength as usize;
                let mut output = Vec::with_capacity(slicelength);
                if start < stop && indices.step > 0 {
                    let step = indices.step as usize;
                    for item in collection[start..stop].iter().step_by(step).cloned() {
                        output.push(item);
                    }
                } else if stop < start && indices.step < 0 {
                    let step = -indices.step as usize;
                    for item in collection[(stop + 1)..=start]
                        .iter()
                        .rev()
                        .step_by(step)
                        .cloned()
                    {
                        output.push(item);
                    }
                }
                ItemOrSequence::Sequence(output)
            }
        })
    }

    /// Set an item or sequence to a slice index.  This abstracts away most of
    /// the logic and checking and just lets the collection worry about seting,
    /// inserting, and deleting.
    /// When deleting is necessary, deletes from end to beginning.
    pub fn set<T, S, I, U, Set, Insert, Delete>(
        self,
        value: ItemOrSequence<T, S>,
        userdata: &mut U,
        mut set: Set,
        mut insert: Insert,
        mut delete: Delete,
    ) -> PyResult<()>
    where
        T: Clone,
        S: IntoIterator<Item = T, IntoIter = I>,
        I: Iterator<Item = T> + ExactSizeIterator,
        Set: FnMut(&mut U, usize, T) -> PyResult<()>,
        Insert: FnMut(&mut U, usize, T) -> PyResult<()>,
        Delete: FnMut(&mut U, Range<usize>) -> PyResult<()>,
    {
        match self {
            Self::Index(index) => {
                let ItemOrSequence::Item(value) = value else {
                    return Err(PyTypeError::new_err(format!(
                        "Only a single item may be specified for a non-slice index"
                    )));
                };
                set(userdata, index, value)?;
            }
            Self::Slice(slice) => {
                let ItemOrSequence::Sequence(values) = value else {
                    return Err(PyTypeError::new_err(format!(
                        "A slice must be given a list of values"
                    )));
                };
                let mut values = values.into_iter();
                let start = slice.start as usize;
                let stop = slice.stop as usize;
                let slice_len = slice.slicelength as usize;
                let values_len = values.len();
                if slice_len == values_len {
                    if start < stop && slice.step > 0 {
                        let step = slice.step as usize;
                        for (index, value) in (start..stop).step_by(step).zip(values) {
                            set(userdata, index, value)?;
                        }
                    } else if stop < start && slice.step < 0 {
                        let step = -slice.step as usize;
                        for (index, value) in ((stop + 1)..=start).rev().step_by(step).zip(values) {
                            set(userdata, index, value)?;
                        }
                    } else if slice_len != 0 {
                        unreachable!()
                    }
                } else {
                    if slice.step != 1 {
                        return Err(PyValueError::new_err(format!("attempt to assign sequence of size {values_len} to extended slice of size {slice_len}")));
                    }
                    // Delete excess elements in the slice
                    if slice_len > values_len {
                        let diff = slice_len - values_len;
                        delete(userdata, (stop - diff)..stop)?;
                    }
                    // The common subset, which can just be set with `set`
                    for i in 0..(slice_len.min(values_len)) {
                        set(userdata, start + i, values.next().unwrap())?;
                    }
                    // Extra elements, for a longer values_len
                    for (i, value) in values.enumerate() {
                        insert(userdata, stop + i, value)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Delete an index or slice.
    /// Deletes from end to beginning.
    pub fn delete<Delete>(self, mut delete: Delete) -> PyResult<()>
    where
        Delete: FnMut(Range<usize>) -> PyResult<()>,
    {
        match self {
            Self::Index(index) => {
                delete(index..(index + 1))?;
            }
            Self::Slice(slice) => {
                let start = slice.start as usize;
                let stop = slice.stop as usize;
                if start < stop && slice.step > 0 {
                    let step = slice.step as usize;
                    if step == 1 {
                        delete(start..stop)?;
                    } else {
                        for index in (start..stop).step_by(step).rev() {
                            delete(index..(index + 1))?;
                        }
                    }
                } else if stop < start && slice.step < 0 {
                    let step = -slice.step as usize;
                    if step == 1 {
                        delete((stop + 1)..(start + 1))?;
                    } else {
                        for index in ((stop + 1)..(start + 1)).rev().step_by(step).rev() {
                            delete(index..(index + 1))?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ItemOrSequence<T, S = Vec<T>> {
    Item(T),
    Sequence(S),
}

impl<T, S> ItemOrSequence<T, S> {
    pub fn map_sequence<S2, Error, F>(self, function: F) -> Result<ItemOrSequence<T, S2>, Error>
    where
        F: FnOnce(S) -> Result<S2, Error>,
    {
        match self {
            ItemOrSequence::Item(item) => Ok(ItemOrSequence::Item(item)),
            ItemOrSequence::Sequence(sequence) => function(sequence).map(ItemOrSequence::Sequence),
        }
    }
}

impl<T, S> IntoPy<Py<PyAny>> for ItemOrSequence<T, S>
where
    T: IntoPy<Py<PyAny>>,
    S: IntoPy<Py<PyAny>>,
{
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            ItemOrSequence::Item(item) => item.into_py(py),
            ItemOrSequence::Sequence(list) => list.into_py(py),
        }
    }
}
impl<'py, T, S> FromPyObject<'py> for ItemOrSequence<T, S>
where
    T: FromPyObject<'py>,
    S: FromPyObject<'py>,
{
    fn extract_bound(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(if let Ok(item) = value.extract::<T>() {
            Self::Item(item)
        } else if let Ok(sequence) = value.extract::<S>() {
            Self::Sequence(sequence)
        } else {
            let type_ = value.get_type();
            let type_name = type_.name()?;
            return Err(PyTypeError::new_err(format!("Invalid type: {type_name}")));
        })
    }
}
