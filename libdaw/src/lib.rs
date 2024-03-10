pub mod nodes;
pub mod stream;

use std::{fmt::Debug, rc::Rc};
pub use stream::Stream;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

/// An audio node trait, allowing a sample_rate to be set and processing to
/// be performed. Some things like setters are self, not mut self, because we
/// need to support Rc<dyn Node> so upcasting works.  This will be fixed when
/// https://github.com/rust-lang/rust/issues/65991 is fully finished and in
/// stable rust.  When that happens, the interface will change to &mut self
/// methods.
pub trait Node: Debug {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [Stream],
        outputs: &'c mut Vec<Stream>,
    ) -> Result<()>;
}

/// A node with a settable frequency.
pub trait FrequencyNode: Node + DynNode {
    fn get_frequency(&self) -> Result<f64>;
    fn set_frequency(&self, frequency: f64) -> Result<()>;
}

/// Dynamic upcasting trait for Node
pub trait DynNode {
    fn node(self: Rc<Self>) -> Rc<dyn Node>;
}

impl<T> DynNode for T
where
    T: 'static + Node,
{
    fn node(self: Rc<Self>) -> Rc<dyn Node> {
        self
    }
}
/// Dynamic upcasting trait for FrequencyNode
pub trait DynFrequencyNode {
    fn frequency_node(self: Rc<Self>) -> Rc<dyn FrequencyNode>;
}

impl<T> DynFrequencyNode for T
where
    T: 'static + FrequencyNode,
{
    fn frequency_node(self: Rc<Self>) -> Rc<dyn FrequencyNode> {
        self
    }
}
