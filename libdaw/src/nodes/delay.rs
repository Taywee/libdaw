use crate::stream::Stream;
use crate::Node;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

use std::time::Duration;

#[derive(Debug)]
struct Sample {
    play_sample: u64,
    stream: Stream,
}

type Buffer = VecDeque<Sample>;

#[derive(Debug)]
pub struct Delay {
    buffers: RefCell<Vec<Buffer>>,
    sample: Cell<u64>,
    delay: u64,
}

impl Delay {
    pub fn new(sample_rate: u32, delay: Duration) -> Self {
        let delay = (delay.as_secs_f64() * sample_rate as f64) as u64;
        Self {
            buffers: Default::default(),
            sample: Default::default(),
            delay,
        }
    }
}

impl Node for Delay {
    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        if self.delay == 0 {
            outputs.extend_from_slice(inputs);
            return;
        }
        let sample = self.sample.replace(self.sample.get() + 1);
        let play_sample = sample + self.delay;

        let mut buffers = self.buffers.borrow_mut();
        if inputs.len() > buffers.len() {
            let delay = self.delay as usize;
            buffers.resize_with(inputs.len(), || VecDeque::with_capacity(delay));
        }

        outputs.reserve(buffers.len());
        for (i, buffer) in buffers.iter_mut().enumerate() {
            let play = buffer
                .front()
                .map(|buffer_sample| sample >= buffer_sample.play_sample)
                .unwrap_or(false);
            if play {
                outputs.push(buffer.pop_front().expect("buffer will not be empty").stream);
            }
            if let Some(stream) = inputs.get(i).copied() {
                buffer.push_back(Sample {
                    play_sample,
                    stream,
                });
            }
        }
    }

    fn node(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn Node> {
        self
    }
}
