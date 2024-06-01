pub mod metronome;
pub mod nodes;
pub mod notation;
mod parse;
pub mod pitch;
pub mod sample;
pub mod time;

pub use sample::Sample;
use std::fmt::Debug;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

/// An audio node trait, allowing a sample_rate to be set and processing to
/// be performed. Some things like setters are self, not mut self, because we
/// need to support Arc<dyn Node> so upcasting works.  This will be fixed when
/// https://github.com/rust-lang/rust/issues/65991 is fully finished and in
/// stable rust.  When that happens, the interface will change to &mut self
/// methods.
pub trait Node: Debug + Send {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()>;
}

impl Iterator for &mut dyn Node {
    type Item = Result<Vec<Sample>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut outputs = Vec::new();
        Some(match self.process(&[], &mut outputs) {
            Err(e) => Err(e),
            Ok(()) => Ok(outputs),
        })
    }
}
