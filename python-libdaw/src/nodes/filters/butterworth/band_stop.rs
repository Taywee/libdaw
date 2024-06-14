use crate::Node;
use libdaw::nodes::filters::butterworth::BandStop as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes.filters.butterworth")]
#[derive(Debug, Clone)]
pub struct BandStop(pub Arc<Mutex<Inner>>);

#[pymethods]
impl BandStop {
    #[new]
    #[pyo3(signature = (order, low_frequency, high_frequency, sample_rate = 48000))]
    pub fn new(
        order: usize,
        low_frequency: f64,
        high_frequency: f64,
        sample_rate: u32,
    ) -> crate::Result<PyClassInitializer<Self>> {
        let inner = Arc::new(Mutex::new(Inner::new(
            sample_rate,
            order,
            low_frequency,
            high_frequency,
        )?));
        Ok(PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner)))
    }
}
