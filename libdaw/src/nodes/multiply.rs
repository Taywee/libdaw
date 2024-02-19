use crate::streams::{Channels, Streams};
use crate::Node;
use smallvec::smallvec;
use std::ops::Mul as _;

#[derive(Debug, Default)]
pub struct Multiply;

impl Node for Multiply {
    fn set_sample_rate(&mut self, _: u32) {}

    fn process(&mut self, input: Streams) -> Streams {
        Streams(smallvec![input
            .0
            .into_iter()
            .reduce(Channels::mul)
            .unwrap_or_default()])
    }
}
