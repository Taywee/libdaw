pub mod metronome;
pub mod nodes;
pub mod notation;
mod parse;
pub mod pitch;
pub mod stream;
mod sync;
pub mod time;

use std::fmt::Debug;
use std::sync::Arc;
pub use stream::Stream;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

/// An audio node trait, allowing a sample_rate to be set and processing to
/// be performed. Some things like setters are self, not mut self, because we
/// need to support Arc<dyn Node> so upcasting works.  This will be fixed when
/// https://github.com/rust-lang/rust/issues/65991 is fully finished and in
/// stable rust.  When that happens, the interface will change to &mut self
/// methods.
pub trait Node: Debug + Send + Sync {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [Stream],
        outputs: &'c mut Vec<Stream>,
    ) -> Result<()>;
}

impl Iterator for &dyn Node {
    type Item = Result<Vec<Stream>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut outputs = Vec::new();
        Some(match self.process(&[], &mut outputs) {
            Err(e) => Err(e),
            Ok(()) => Ok(outputs),
        })
    }
}
/// A node with a settable frequency.
pub trait FrequencyNode: Node + DynNode {
    fn get_frequency(&self) -> Result<f64>;
    fn set_frequency(&self, frequency: f64) -> Result<()>;
}

/// Dynamic upcasting trait for Node
pub trait DynNode {
    fn node(self: Arc<Self>) -> Arc<dyn Node>;
}

impl<T> DynNode for T
where
    T: 'static + Node,
{
    fn node(self: Arc<Self>) -> Arc<dyn Node> {
        self
    }
}
/// Dynamic upcasting trait for FrequencyNode
pub trait DynFrequencyNode {
    fn frequency_node(self: Arc<Self>) -> Arc<dyn FrequencyNode>;
}

impl<T> DynFrequencyNode for T
where
    T: 'static + FrequencyNode,
{
    fn frequency_node(self: Arc<Self>) -> Arc<dyn FrequencyNode> {
        self
    }
}
