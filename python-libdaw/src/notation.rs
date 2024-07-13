mod chord;
pub mod duration;
mod item;
mod mode;
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
pub use item::{Item, ItemOrValue};
pub use mode::Mode;
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
    types::{PyModule, PyModuleMethods as _}, Bound, PyResult,
};

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Chord>()?;
    module.add_class::<Item>()?;
    module.add_class::<Mode>()?;
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
