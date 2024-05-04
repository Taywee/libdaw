mod absolute;

use crate::submodule;
use pyo3::{types::PyModule, Bound, PyResult};

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    absolute::register(&submodule!(module, "libdaw.notation", "absolute"))?;
    Ok(())
}
