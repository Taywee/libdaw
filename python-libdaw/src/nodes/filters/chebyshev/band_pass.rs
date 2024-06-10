use crate::Node;
use libdaw::nodes::filters::chebyshev::BandPass as Inner;
use pyo3::{pyclass, pymethods, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes.filters.chebyshev")]
#[derive(Debug, Clone)]
pub struct BandPass(pub Arc<Mutex<Inner>>);

#[pymethods]
impl BandPass {
    #[new]
    #[pyo3(signature = (n, epsilon, low_frequency, high_frequency, sample_rate = 48000))]
    pub fn new(
        n: usize,
        epsilon: f64,
        low_frequency: f64,
        high_frequency: f64,
        sample_rate: u32,
    ) -> crate::Result<PyClassInitializer<Self>> {
        let inner = Arc::new(Mutex::new(Inner::new(
            sample_rate,
            n,
            epsilon,
            low_frequency,
            high_frequency,
        )?));
        Ok(PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner)))
    }
}
