pub mod nodes;
pub mod stream;

use std::fmt::Debug;
use stream::Stream;

/// An audio node trait, allowing a sample_rate to be set and processing to
/// be performed.
pub trait Node: Debug {
    fn set_sample_rate(&mut self, sample_rate: u32);
    fn set_channels(&mut self, channels: u16);
    fn get_sample_rate(&self) -> u32;
    fn get_channels(&self) -> u16;
    fn process<'a, 'b>(&'a mut self, inputs: &'b [Stream], outputs: &'a mut Vec<Stream>);
}
