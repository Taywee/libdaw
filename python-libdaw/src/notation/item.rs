use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use super::Element;
use libdaw::notation::Item as DawItem;
use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods as _, PyTypeMethods as _},
    Bound, FromPyObject, IntoPy, Py, PyAny, PyResult, PyTraverseError, PyVisit, Python,
};

#[pyclass(module = "libdaw.notation")]
#[derive(Debug, Clone)]
pub struct Item {
    pub inner: Arc<Mutex<DawItem>>,
    pub element: Option<Py<Element>>,
}

impl Item {
    pub fn from_inner<'py>(py: Python<'py>, inner: Arc<Mutex<DawItem>>) -> Bound<'py, Self> {
        let element =
            Element::from_inner(py, inner.lock().expect("poisoned").element.clone()).unbind();
        Self {
            inner,
            element: Some(element),
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
    #[pyo3(signature = (element, tags = Default::default()))]
    pub fn new(element: &Bound<'_, Element>, tags: HashSet<String>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(DawItem {
                element: Element::as_inner(element),
                tags,
            })),
            element: Some(element.clone().unbind()),
        }
    }
    #[getter]
    pub fn get_element(&self) -> Py<Element> {
        self.element.clone().expect("cleared")
    }
    #[setter]
    pub fn set_element(&mut self, element: Bound<'_, Element>) {
        self.inner.lock().expect("poisoned").element = Element::as_inner(&element);
        self.element = Some(element.unbind());
    }
    #[getter]
    pub fn get_tags(&self) -> HashSet<String> {
        self.inner.lock().expect("poisoned").tags.clone()
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
    pub fn __getnewargs__(&self) -> (Py<Element>, HashSet<String>) {
        (self.element.clone().expect("cleared"), self.get_tags())
    }
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        if let Some(element) = &self.element {
            visit.call(element)?
        }
        Ok(())
    }
    pub fn __clear__(&mut self) {
        self.element = None;
    }
}

/// A FromPyObject wrappper that makes it easy to take an Item or Py<Element>,
/// automatically wrapping in an Item.
#[derive(Debug, Clone)]
pub struct ItemOrElement<'py> {
    pub item: Bound<'py, Item>,
}

impl<'py> FromPyObject<'py> for ItemOrElement<'py> {
    fn extract_bound(element: &Bound<'py, PyAny>) -> PyResult<Self> {
        let py = element.py();
        Ok(if let Ok(item) = element.downcast::<Item>() {
            Self { item: item.clone() }
        } else if let Ok(element) = element.downcast::<Element>() {
            let item = Bound::new(py, Item::new(element, Default::default()))
                .expect("Could not build Item from element");
            Self { item }
        } else {
            let type_ = element.get_type();
            let type_name = type_.name()?;
            return Err(PyTypeError::new_err(format!(
                "Item was invalid type: {type_name}"
            )));
        })
    }
}
