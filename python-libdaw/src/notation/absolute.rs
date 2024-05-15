mod chord;
mod item;
mod note;
mod overlapped;
mod rest;
mod sequence;

pub use chord::Chord;
pub use item::Item;
pub use note::Note;
pub use overlapped::Overlapped;
pub use rest::Rest;
pub use sequence::Sequence;

use pyo3::{
    types::{PyModule, PyModuleMethods as _},
    wrap_pyfunction, Bound, PyResult,
};

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(item::parse, module)?)?;
    module.add_class::<Chord>()?;
    module.add_class::<Sequence>()?;
    module.add_class::<Overlapped>()?;
    module.add_class::<Note>()?;
    module.add_class::<Rest>()?;
    Ok(())
}
