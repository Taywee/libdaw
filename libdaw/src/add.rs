use super::Node;
use crate::streams::{Channels, Streams};
use smallvec::smallvec;
use std::ops::Add as _;

#[derive(Debug, Default)]
pub struct Add;

impl Node for Add {
    fn set_sample_rate(&mut self, _: u32) {}

    fn process(&mut self, input: Streams) -> Streams {
        Streams(smallvec![input
            .0
            .into_iter()
            .reduce(Channels::add)
            .unwrap_or_default()])
    }
}
