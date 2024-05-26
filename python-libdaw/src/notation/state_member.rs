use libdaw::notation::StateMember as DawStateMember;
use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyResult};

#[pyclass(module = "libdaw.notation")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateMember {
    First,
    Last,
}

#[pymethods]
impl StateMember {
    #[new]
    pub fn new(name: &str) -> PyResult<Self> {
        match name.to_lowercase().as_str() {
            "first" => Ok(Self::First),
            "last" => Ok(Self::Last),
            name => Err(PyValueError::new_err(format!("Unknown name {name}"))),
        }
    }

    pub fn __getnewargs__(&self) -> (&str,) {
        match self {
            StateMember::First => ("first",),
            StateMember::Last => ("last",),
        }
    }
}

impl From<StateMember> for DawStateMember {
    fn from(value: StateMember) -> Self {
        match value {
            StateMember::First => DawStateMember::First,
            StateMember::Last => DawStateMember::Last,
        }
    }
}
impl From<DawStateMember> for StateMember {
    fn from(value: DawStateMember) -> Self {
        match value {
            DawStateMember::First => StateMember::First,
            DawStateMember::Last => StateMember::Last,
        }
    }
}
