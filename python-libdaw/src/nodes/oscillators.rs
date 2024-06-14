pub mod sawtooth;
pub mod sine;
pub mod square;
pub mod triangle;

pub use sawtooth::Sawtooth;
pub use sine::Sine;
pub use square::Square;
pub use triangle::Triangle;

use pyo3::{
    types::{PyModule, PyModuleMethods},
    Bound, PyResult,
};

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Sawtooth>()?;
    module.add_class::<Sine>()?;
    module.add_class::<Square>()?;
    module.add_class::<Triangle>()?;
    Ok(())
}
