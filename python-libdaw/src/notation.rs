mod chord;
pub mod duration;
mod inversion;
mod item;
mod note;
mod overlapped;
mod pitch;
mod rest;
mod scale;
mod sequence;
mod set;
mod state_member;
mod step;

pub use chord::Chord;
pub use inversion::Inversion;
pub use item::Item;
pub use note::{Note, NotePitch};
pub use overlapped::Overlapped;
pub use pitch::Pitch;
pub use rest::Rest;
pub use scale::Scale;
pub use sequence::Sequence;
pub use set::Set;
pub use state_member::StateMember;
pub use step::Step;

use crate::submodule;
use pyo3::{
    types::{PyModule, PyModuleMethods as _},
    wrap_pyfunction, Bound, PyResult,
};

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(item::loads, module)?)?;
    module.add_class::<Chord>()?;
    module.add_class::<Inversion>()?;
    module.add_class::<Note>()?;
    module.add_class::<Overlapped>()?;
    module.add_class::<Pitch>()?;
    module.add_class::<Rest>()?;
    module.add_class::<Scale>()?;
    module.add_class::<Sequence>()?;
    module.add_class::<Set>()?;
    module.add_class::<StateMember>()?;
    module.add_class::<Step>()?;
    duration::register(&submodule!(module, "libdaw.notation", "duration"))?;
    Ok(())
}
