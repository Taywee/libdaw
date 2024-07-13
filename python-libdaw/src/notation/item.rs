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

#[derive(Debug, Clone)]
#[pyclass(subclass, module = "libdaw.notation")]
pub struct ItemValue {
    pub inner: Arc<Mutex<dyn DawItemValue>>,
}

#[pyclass(module = "libdaw.notation")]
#[derive(Debug, Clone)]
pub struct Item {
    pub inner: Arc<Mutex<DawItem>>,
    pub value: Option<Py<ItemValue>>,
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
        } else if let Ok(value) = value.downcast::<ItemValue>() {
            let inner_item = DawItem {
                value: value.borrow().inner.clone(),
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
