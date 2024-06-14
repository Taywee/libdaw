pub mod add;
pub mod callback;
pub mod constant_value;
pub mod custom;
pub mod delay;
pub mod detune;
pub mod envelope;
pub mod explode;
pub mod filters;
pub mod gain;
pub mod graph;
pub mod implode;
pub mod instrument;
pub mod multiply;
pub mod oscillators;
pub mod passthrough;

pub use add::Add;
pub use callback::Callback;
pub use constant_value::ConstantValue;
pub use custom::Custom;
pub use delay::Delay;
pub use detune::Detune;
pub use envelope::Envelope;
pub use explode::Explode;
pub use gain::Gain;
pub use graph::Graph;
pub use implode::Implode;
pub use instrument::Instrument;
pub use multiply::Multiply;
pub use passthrough::Passthrough;

use crate::submodule;
use pyo3::{
    types::{PyModule, PyModuleMethods},
    Bound, PyResult,
};

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Add>()?;
    module.add_class::<Callback>()?;
    module.add_class::<ConstantValue>()?;
    module.add_class::<Custom>()?;
    module.add_class::<Delay>()?;
    module.add_class::<Detune>()?;
    module.add_class::<Envelope>()?;
    module.add_class::<Explode>()?;
    module.add_class::<Implode>()?;
    module.add_class::<Gain>()?;
    module.add_class::<Graph>()?;
    module.add_class::<Instrument>()?;
    module.add_class::<Multiply>()?;
    module.add_class::<Passthrough>()?;
    envelope::register(&submodule!(module, "libdaw.nodes", "envelope"))?;
    filters::register(&submodule!(module, "libdaw.nodes", "filters"))?;
    instrument::register(&submodule!(module, "libdaw.nodes", "instrument"))?;
    oscillators::register(&submodule!(module, "libdaw.nodes", "oscillators"))?;
    Ok(())
}
