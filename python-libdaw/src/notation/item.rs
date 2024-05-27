use super::{Chord, Inversion, Note, Overlapped, Rest, Scale, Sequence, Set};
use crate::Result;
use libdaw::notation::Item as DawItem;
use pyo3::{
    exceptions::PyTypeError,
    pyfunction,
    types::{PyAnyMethods as _, PyTypeMethods as _},
    AsPyPointer, Bound, FromPyObject, IntoPy, Py, PyAny, PyResult, Python,
};

/// A wrapper enum for converting between Rust Items and the Python classes.
#[derive(Debug, Clone)]
pub enum Item {
    Note(Py<Note>),
    Chord(Py<Chord>),
    Rest(Py<Rest>),
    Overlapped(Py<Overlapped>),
    Sequence(Py<Sequence>),
    Scale(Py<Scale>),
    Inversion(Py<Inversion>),
    Set(Py<Set>),
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
            DawItem::Scale(scale) => Self::Scale(Scale::from_inner(py, scale)),
            DawItem::Inversion(inversion) => Self::Inversion(Inversion::from_inner(py, inversion)),
            DawItem::Set(set) => Self::Set(Set::from_inner(py, set)),
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
            Item::Scale(scale) => DawItem::Scale(scale.bind_borrowed(py).borrow().inner.clone()),
            Item::Inversion(inversion) => {
                DawItem::Inversion(inversion.bind_borrowed(py).borrow().inner.clone())
            }
            Item::Set(set) => DawItem::Set(set.bind_borrowed(py).borrow().inner.clone()),
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
        } else if let Ok(sequence) = value.downcast::<Sequence>() {
            Self::Sequence(sequence.clone().unbind())
        } else if let Ok(scale) = value.downcast::<Scale>() {
            Self::Scale(scale.clone().unbind())
        } else if let Ok(inversion) = value.downcast::<Inversion>() {
            Self::Inversion(inversion.clone().unbind())
        } else if let Ok(set) = value.downcast::<Set>() {
            Self::Set(set.clone().unbind())
        } else {
            let type_ = value.get_type();
            let type_name = type_.name()?;
            return Err(PyTypeError::new_err(format!(
                "Item was invalid type: {type_name}"
            )));
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
            Item::Scale(scale) => scale.into_py(py),
            Item::Inversion(inversion) => inversion.into_py(py),
            Item::Set(set) => set.into_py(py),
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
            Item::Scale(scale) => scale.as_ptr(),
            Item::Inversion(inversion) => inversion.as_ptr(),
            Item::Set(set) => set.as_ptr(),
        }
    }
}

#[pyfunction]
pub fn loads(py: Python<'_>, source: &str) -> Result<Item> {
    let item: DawItem = source.parse()?;
    Ok(Item::from_inner(py, item))
}
