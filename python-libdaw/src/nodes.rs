pub mod add;
pub mod constant_value;
pub mod delay;
pub mod detune;
pub mod envelope;
pub mod gain;
pub mod graph;
pub mod instrument;
pub mod multi_frequency;
pub mod multiply;
pub mod passthrough;
pub mod sawtooth_oscillator;
pub mod sine_oscillator;
pub mod square_oscillator;
pub mod triangle_oscillator;

pub use add::Add;
pub use constant_value::ConstantValue;
pub use delay::Delay;
pub use detune::Detune;
pub use envelope::Envelope;
pub use gain::Gain;
pub use graph::Graph;
pub use instrument::Instrument;
pub use multi_frequency::MultiFrequency;
pub use multiply::Multiply;
pub use passthrough::Passthrough;
pub use sawtooth_oscillator::SawtoothOscillator;
pub use sine_oscillator::SineOscillator;
pub use square_oscillator::SquareOscillator;
pub use triangle_oscillator::TriangleOscillator;

use crate::submodule;
use pyo3::{
    types::{PyModule, PyModuleMethods},
    Bound, PyResult,
};

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Add>()?;
    module.add_class::<ConstantValue>()?;
    module.add_class::<Delay>()?;
    module.add_class::<Detune>()?;
    module.add_class::<Envelope>()?;
    module.add_class::<Gain>()?;
    module.add_class::<Graph>()?;
    module.add_class::<Instrument>()?;
    module.add_class::<MultiFrequency>()?;
    module.add_class::<Multiply>()?;
    module.add_class::<Passthrough>()?;
    module.add_class::<SawtoothOscillator>()?;
    module.add_class::<SineOscillator>()?;
    module.add_class::<SquareOscillator>()?;
    module.add_class::<TriangleOscillator>()?;
    envelope::register(&submodule!(module, "libdaw.nodes", "envelope"))?;
    instrument::register(&submodule!(module, "libdaw.nodes", "instrument"))?;
    graph::register(&submodule!(module, "libdaw.nodes", "graph"))?;
    Ok(())
}
