pub mod butterworth;
pub mod chebyshev;
pub mod moving_average;

pub use moving_average::MovingAverage;

use crate::submodule;
use pyo3::{
    types::{PyModule, PyModuleMethods},
    Bound, PyResult,
};

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<MovingAverage>()?;
    butterworth::register(&submodule!(module, "libdaw.nodes.filters", "butterworth"))?;
    chebyshev::register(&submodule!(module, "libdaw.nodes.filters", "chebyshev"))?;
    Ok(())
}
