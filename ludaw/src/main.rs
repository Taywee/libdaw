use libdaw::streams::Channels;
use libdaw::{ConstantValue, Graph, Multiply, Node, SquareOscillator};
use mlua::prelude::*;
use rodio::source::Source;
use rodio::{OutputStream, Sink};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug)]
struct LibDawGraph {
    graph: Graph,
    sample: smallvec::IntoIter<[f64; 2]>,
}

impl LibDawGraph {
    fn new(mut graph: Graph) -> Self {
        graph.set_sample_rate(44100.0);
        let sample = graph
            .process(Default::default())
            .0
            .into_iter()
            .next()
            .unwrap()
            .0
            .into_iter();
        LibDawGraph { graph, sample }
    }
}

impl Source for LibDawGraph {
    fn current_frame_len(&self) -> Option<usize> {
        Some(1)
    }

    fn channels(&self) -> u16 {
        self.sample.len().try_into().expect("Too many channels")
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for LibDawGraph {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.sample.next().map(|sample| sample as f32);
        if self.sample.len() == 0 {
            self.sample = self
                .graph
                .process(Default::default())
                .0
                .into_iter()
                .next()
                .unwrap()
                .0
                .into_iter();
        }
        next
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _lua = Lua::new();
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let mut graph = Graph::default();
    let mut square = Arc::new(Mutex::new(SquareOscillator::default()));
    let mut multiply = Arc::new(Mutex::new(Multiply::default()));
    let mut multiply_constant = Arc::new(Mutex::new(ConstantValue::new(0.001)));
    graph.connect(square, 0, multiply.clone());
    graph.connect(multiply_constant, 0, multiply.clone());
    graph.sink(multiply, 0);
    sink.append(LibDawGraph::new(graph));
    sink.sleep_until_end();

    Ok(())
}
