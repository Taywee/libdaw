use crate::{metronome::Beat, Result};
use pyo3::{pyclass, pymethods};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Rest(pub Arc<Mutex<libdaw::notation::absolute::Rest>>);

#[pymethods]
impl Rest {
    #[new]
    pub fn new(length: Option<Beat>) -> Self {
        Self(Arc::new(Mutex::new(libdaw::notation::absolute::Rest {
            length: length.map(|beat| beat.0),
        })))
    }
    #[staticmethod]
    pub fn parse(source: String) -> Result<Self> {
        Ok(Self(Arc::new(Mutex::new(source.parse()?))))
    }

    pub fn get_length(&self) -> Option<Beat> {
        self.0.lock().expect("poisoned").length.map(Beat)
    }
    #[pyo3(signature = (value))]
    pub fn set_length(&mut self, value: Option<Beat>) {
        self.0.lock().expect("poisoned").length = value.map(|beat| beat.0);
    }

    pub fn length(&self, default: Beat) -> Beat {
        Beat(self.0.lock().expect("poisoned").length(default.0))
    }

    pub fn duration(&self) -> Beat {
        Beat(self.0.lock().expect("poisoned").duration())
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0.lock().expect("poisoned").deref())
    }
}
