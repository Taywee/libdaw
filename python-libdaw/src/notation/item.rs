use std::sync::{Arc, Mutex};

use super::{Chord, Mode, Note, Overlapped, Rest, Scale, Sequence, Set};
use libdaw::notation::{Item as DawItem, ItemValue as DawItemValue};
use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods as _, PyTypeMethods as _},
    AsPyPointer, Bound, FromPyObject, IntoPy, Py, PyAny, PyResult, PyTraverseError, PyVisit,
    Python,
};

/// A wrapper enum for converting between Rust Items and the Python classes.
#[derive(Debug, Clone)]
pub enum ItemValue {
    Note(Py<Note>),
    Chord(Py<Chord>),
    Rest(Py<Rest>),
    Overlapped(Py<Overlapped>),
    Sequence(Py<Sequence>),
    Scale(Py<Scale>),
    Mode(Py<Mode>),
    Set(Py<Set>),
}

impl ItemValue {
    pub fn from_inner(py: Python<'_>, inner: DawItemValue) -> Self {
        match inner {
            DawItemValue::Note(note) => Self::Note(Note::from_inner(py, note)),
            DawItemValue::Chord(chord) => Self::Chord(Chord::from_inner(py, chord)),
            DawItemValue::Rest(rest) => Self::Rest(Rest::from_inner(py, rest)),
            DawItemValue::Overlapped(overlapped) => {
                Self::Overlapped(Overlapped::from_inner(py, overlapped))
            }
            DawItemValue::Sequence(sequence) => Self::Sequence(Sequence::from_inner(py, sequence)),
            DawItemValue::Scale(scale) => Self::Scale(Scale::from_inner(py, scale)),
            DawItemValue::Mode(mode) => Self::Mode(Mode::from_inner(py, mode)),
            DawItemValue::Set(set) => Self::Set(Set::from_inner(py, set)),
        }
    }
    pub fn as_inner(&self, py: Python<'_>) -> DawItemValue {
        match self {
            ItemValue::Note(note) => {
                DawItemValue::Note(note.bind_borrowed(py).borrow().inner.clone())
            }
            ItemValue::Chord(chord) => {
                DawItemValue::Chord(chord.bind_borrowed(py).borrow().inner.clone())
            }
            ItemValue::Rest(rest) => {
                DawItemValue::Rest(rest.bind_borrowed(py).borrow().inner.clone())
            }
            ItemValue::Overlapped(overlapped) => {
                DawItemValue::Overlapped(overlapped.bind_borrowed(py).borrow().inner.clone())
            }
            ItemValue::Sequence(sequence) => {
                DawItemValue::Sequence(sequence.bind_borrowed(py).borrow().inner.clone())
            }
            ItemValue::Scale(scale) => {
                DawItemValue::Scale(scale.bind_borrowed(py).borrow().inner.clone())
            }
            ItemValue::Mode(mode) => {
                DawItemValue::Mode(mode.bind_borrowed(py).borrow().inner.clone())
            }
            ItemValue::Set(set) => DawItemValue::Set(set.bind_borrowed(py).borrow().inner.clone()),
        }
    }
}

impl<'py> FromPyObject<'py> for ItemValue {
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
        } else if let Ok(mode) = value.downcast::<Mode>() {
            Self::Mode(mode.clone().unbind())
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

impl IntoPy<Py<PyAny>> for ItemValue {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            ItemValue::Note(note) => note.into_py(py),
            ItemValue::Chord(chord) => chord.into_py(py),
            ItemValue::Rest(rest) => rest.into_py(py),
            ItemValue::Overlapped(overlapped) => overlapped.into_py(py),
            ItemValue::Sequence(sequence) => sequence.into_py(py),
            ItemValue::Scale(scale) => scale.into_py(py),
            ItemValue::Mode(mode) => mode.into_py(py),
            ItemValue::Set(set) => set.into_py(py),
        }
    }
}

unsafe impl AsPyPointer for ItemValue {
    fn as_ptr(&self) -> *mut pyo3::ffi::PyObject {
        match self {
            ItemValue::Note(note) => note.as_ptr(),
            ItemValue::Chord(chord) => chord.as_ptr(),
            ItemValue::Rest(rest) => rest.as_ptr(),
            ItemValue::Overlapped(overlapped) => overlapped.as_ptr(),
            ItemValue::Sequence(sequence) => sequence.as_ptr(),
            ItemValue::Scale(scale) => scale.as_ptr(),
            ItemValue::Mode(mode) => mode.as_ptr(),
            ItemValue::Set(set) => set.as_ptr(),
        }
    }
}

#[pyclass(module = "libdaw.notation")]
#[derive(Debug, Clone)]
pub struct Item {
    pub inner: Arc<Mutex<DawItem>>,
    pub value: Option<ItemValue>,
}

impl Item {
    pub fn from_inner<'py>(py: Python<'py>, inner: Arc<Mutex<DawItem>>) -> Bound<'py, Self> {
        let value = ItemValue::from_inner(py, inner.lock().expect("poisoned").value.clone());
        Self {
            inner,
            value: Some(value),
        }
        .into_py(py)
        .downcast_bound(py)
        .unwrap()
        .clone()
    }
}

#[pymethods]
impl Item {
    #[new]
    pub fn new(py: Python<'_>, value: ItemValue) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawItem {
                value: value.as_inner(py),
            })),
            value: Some(value),
        }
    }
    #[getter]
    pub fn get_value(&self) -> ItemValue {
        self.value.clone().expect("cleared")
    }
    #[setter]
    pub fn set_value(&mut self, value: ItemValue) {
        self.value = Some(value);
    }
    #[staticmethod]
    pub fn loads<'py>(py: Python<'py>, source: &str) -> crate::Result<Bound<'py, Self>> {
        let item: DawItem = source.parse()?;
        Ok(Self::from_inner(py, Arc::new(Mutex::new(item))))
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.lock().expect("poisoned"))
    }
    pub fn __str__(&self) -> String {
        format!("{:#?}", self.inner.lock().expect("poisoned"))
    }
    pub fn __getnewargs__(&self) -> (ItemValue,) {
        (self.value.clone().expect("cleared"),)
    }
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        if let Some(value) = &self.value {
            visit.call(value)?
        }
        Ok(())
    }
    pub fn __clear__(&mut self) {
        self.value = None;
    }
}

/// A FromPyObject wrappper that makes it easy to take an Item or ItemValue,
/// automatically wrapping in an Item.
#[derive(Debug, Clone)]
pub struct ItemOrValue<'py>(pub Bound<'py, Item>);

impl<'py> FromPyObject<'py> for ItemOrValue<'py> {
    fn extract_bound(value: &Bound<'py, PyAny>) -> PyResult<Self> {
        let py = value.py();
        Ok(if let Ok(item) = value.downcast::<Item>() {
            Self(item.clone())
        } else if let Ok(value) = value.extract::<ItemValue>() {
            let inner_item = DawItem {
                value: value.as_inner(py),
            };
            Self(Item::from_inner(py, Arc::new(Mutex::new(inner_item))))
        } else {
            let type_ = value.get_type();
            let type_name = type_.name()?;
            return Err(PyTypeError::new_err(format!(
                "Item was invalid type: {type_name}"
            )));
        })
    }
}
