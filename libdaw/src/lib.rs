pub mod nodes;
pub mod stream;

use std::{cell::RefCell, fmt::Debug, rc::Rc, time::Duration};
use stream::Stream;

/// An audio node trait, allowing a sample_rate to be set and processing to
/// be performed. Some things like setters are self, not mut self, because we
/// need to support Rc<dyn Node> so upcasting works.  This will be fixed when
/// https://github.com/rust-lang/rust/issues/65991 is fully finished and in
/// stable rust.  When that happens, the interface will change to &mut self
/// methods.
pub trait Node: Debug {
    fn set_sample_rate(&self, sample_rate: u32);
    fn set_channels(&self, channels: u16);
    fn get_sample_rate(&self) -> u32;
    fn get_channels(&self) -> u16;
    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>);
    fn node(self: Rc<Self>) -> Rc<dyn Node>;
}

/// A node with a settable frequency.
pub trait FrequencyNode: Node {
    fn get_frequency(&self) -> f64;
    fn set_frequency(&self, frequency: f64);
    fn frequency_node(self: Rc<Self>) -> Rc<dyn FrequencyNode>;
}

/// A frequency node with a length, for things like envelope application. This
/// will be mostly internal to instrument nodes, which will maintain its own
/// note graph.
pub trait NoteNode: FrequencyNode {
    fn get_length(&self) -> Duration;
    fn set_length(&self, length: Duration);
    fn note_node(self: Rc<Self>) -> Rc<dyn NoteNode>;
}
