use crate::notation::absolute::{Chord, Note, Overlapped, Rest};
use libdaw::notation::absolute::Item as DawItem;
use pyo3::{types::PyAnyMethods as _, Bound, FromPyObject, IntoPy, Py, PyAny, PyResult, Python};

/// A wrapper enum for converting between Rust Items and the Python classes.
#[derive(Debug, Clone)]
pub struct Item(pub DawItem);

impl From<DawItem> for Item {
    fn from(value: DawItem) -> Self {
        Self(value)
    }
}
impl From<Item> for DawItem {
    fn from(value: Item) -> Self {
        value.0
    }
}

impl<'py> FromPyObject<'py> for Item {
    fn extract_bound(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        let item = if let Ok(note) = value.downcast::<Note>() {
            DawItem::Note(note.borrow().0.clone())
        } else if let Ok(chord) = value.downcast::<Chord>() {
            DawItem::Chord(chord.borrow().0.clone())
        } else if let Ok(rest) = value.downcast::<Rest>() {
            DawItem::Rest(rest.borrow().0.clone())
        } else {
            let overlapped: Bound<'_, Overlapped> = value.extract()?;
            DawItem::Overlapped(overlapped.borrow().0.clone())
        };
        Ok(item.into())
    }
}

impl IntoPy<Py<PyAny>> for Item {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self.0 {
            DawItem::Note(note) => Note(note).into_py(py),
            DawItem::Chord(chord) => Chord(chord).into_py(py),
            DawItem::Rest(rest) => Rest(rest).into_py(py),
            DawItem::Overlapped(overlapped) => Overlapped(overlapped).into_py(py),
        }
    }
}
