mod add;
mod constant_value;
mod graph;
mod multiply;
mod sawtooth_oscillator;
mod square_oscillator;
pub mod streams;

pub use add::Add;
pub use constant_value::ConstantValue;
pub use graph::Graph;
pub use multiply::Multiply;
pub use sawtooth_oscillator::SawtoothOscillator;
pub use square_oscillator::SquareOscillator;

use crate::streams::Streams;

use std::fmt::Debug;

/// An audio node trait, allowing a sample_rate to be set and processing to
/// be performed.
pub trait Node: Debug {
    fn set_sample_rate(&mut self, sample_rate: u32);
    fn process(&mut self, inputs: Streams) -> Streams;
}
