use crate::{time::Duration, Node};
use libdaw::nodes::filters::MovingAverage as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes.filters")]
#[derive(Debug, Clone)]
pub struct MovingAverage(pub Arc<Mutex<Inner>>);

#[pymethods]
impl MovingAverage {
    #[new]
    #[pyo3(signature = (window, sample_rate = 48000))]
    pub fn new(window: Duration, sample_rate: u32) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::new(sample_rate, window.0)));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }
}
