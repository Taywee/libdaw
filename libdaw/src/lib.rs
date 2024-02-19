pub mod nodes;
pub mod streams;

use std::fmt::Debug;
use streams::Streams;

/// An audio node trait, allowing a sample_rate to be set and processing to
/// be performed.
pub trait Node: Debug {
    fn set_sample_rate(&mut self, sample_rate: u32);
    fn process(&mut self, inputs: Streams) -> Streams;
}
