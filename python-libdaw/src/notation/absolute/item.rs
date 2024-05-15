use crate::notation::absolute::{Chord, Note, Overlapped, Rest, Sequence};
use crate::Result;
use libdaw::notation::absolute::Item as DawItem;
use pyo3::AsPyPointer;
use pyo3::{
    pyfunction, types::PyAnyMethods as _, Bound, FromPyObject, IntoPy, Py, PyAny, PyResult, Python,
};

/// A wrapper enum for converting between Rust Items and the Python classes.
#[derive(Debug, Clone)]
pub enum Item {
    Note(Py<Note>),
    Chord(Py<Chord>),
    Rest(Py<Rest>),
    Overlapped(Py<Overlapped>),
    Sequence(Py<Sequence>),
}

impl Item {
    pub fn from_inner(py: Python<'_>, inner: DawItem) -> Self {
        match inner {
            DawItem::Note(note) => Self::Note(Note::from_inner(py, note)),
            DawItem::Chord(chord) => Self::Chord(Chord::from_inner(py, chord)),
            DawItem::Rest(rest) => Self::Rest(Rest::from_inner(py, rest)),
            DawItem::Overlapped(overlapped) => {
                Self::Overlapped(Overlapped::from_inner(py, overlapped))
            }
            DawItem::Sequence(sequence) => Self::Sequence(Sequence::from_inner(py, sequence)),
        }
    }
    pub fn as_inner(&self, py: Python<'_>) -> DawItem {
        match self {
            Item::Note(note) => DawItem::Note(note.bind_borrowed(py).borrow().inner.clone()),
            Item::Chord(chord) => DawItem::Chord(chord.bind_borrowed(py).borrow().inner.clone()),
            Item::Rest(rest) => DawItem::Rest(rest.bind_borrowed(py).borrow().inner.clone()),
            Item::Overlapped(overlapped) => {
                DawItem::Overlapped(overlapped.bind_borrowed(py).borrow().inner.clone())
            }
            Item::Sequence(sequence) => {
                DawItem::Sequence(sequence.bind_borrowed(py).borrow().inner.clone())
            }
        }
    }
}

impl<'py> FromPyObject<'py> for Item {
    fn extract_bound(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(if let Ok(note) = value.downcast::<Note>() {
            Self::Note(note.clone().unbind())
        } else if let Ok(chord) = value.downcast::<Chord>() {
            Self::Chord(chord.clone().unbind())
        } else if let Ok(rest) = value.downcast::<Rest>() {
            Self::Rest(rest.clone().unbind())
        } else if let Ok(overlapped) = value.downcast::<Overlapped>() {
            Self::Overlapped(overlapped.clone().unbind())
        } else {
            let sequence: Bound<'_, Sequence> = value.extract()?;
            Self::Sequence(sequence.clone().unbind())
        })
    }
}

impl IntoPy<Py<PyAny>> for Item {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            Item::Note(note) => note.into_py(py),
            Item::Chord(chord) => chord.into_py(py),
            Item::Rest(rest) => rest.into_py(py),
            Item::Overlapped(overlapped) => overlapped.into_py(py),
            Item::Sequence(sequence) => sequence.into_py(py),
        }
    }
}

unsafe impl AsPyPointer for Item {
    fn as_ptr(&self) -> *mut pyo3::ffi::PyObject {
        match self {
            Item::Note(note) => note.as_ptr(),
            Item::Chord(chord) => chord.as_ptr(),
            Item::Rest(rest) => rest.as_ptr(),
            Item::Overlapped(overlapped) => overlapped.as_ptr(),
            Item::Sequence(sequence) => sequence.as_ptr(),
        }
    }
}

#[pyfunction]
pub fn parse(py: Python<'_>, source: &str) -> Result<Item> {
    let item: DawItem = source.parse()?;
    Ok(Item::from_inner(py, item))
}
