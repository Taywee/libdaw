use crate::stream::Stream;
use crate::Node;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::time::Duration;

type Buffer = VecDeque<Stream>;

#[derive(Debug)]
pub struct Delay {
    buffers: RefCell<Vec<Buffer>>,
    buffer_size: Cell<usize>,
    delay: Cell<Duration>,
    sample_rate: Cell<u32>,
    channels: Cell<u16>,
}

impl Delay {
    pub fn new(delay: Duration) -> Self {
        let node = Self {
            buffers: Default::default(),
            buffer_size: Default::default(),
            delay: delay.into(),
            sample_rate: 48000.into(),
            channels: Default::default(),
        };
        node.update_buffer_size();
        node
    }

    // We might want to remove this.  Setting a delay to a shorter time will
    // either not work or have to truncate the existing buffer.
    // Then again, the same will happen for set_sample rate.
    pub fn set_delay(&self, delay: Duration) {
        self.delay.set(delay);
        self.update_buffer_size();
    }

    pub fn get_delay(&self) -> Duration {
        self.delay.get()
    }

    fn update_buffer_size(&self) {
        let buffer_size =
            (self.delay.get().as_secs_f64() * self.sample_rate.get() as f64).round() as usize;
        self.buffer_size.set(buffer_size);
        let channels = self.channels.get() as usize;
        for buffer in self.buffers.borrow_mut().deref_mut() {
            buffer.resize(buffer_size, Stream::new(channels));
        }
    }
}

impl Node for Delay {
    fn set_sample_rate(&self, sample_rate: u32) {
        self.sample_rate.set(sample_rate);
        self.update_buffer_size();
    }

    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        let buffer_size = self.buffer_size.get();
        if buffer_size == 0 {
            outputs.extend_from_slice(inputs);
            return;
        }
        let mut buffers = self.buffers.borrow_mut();
        if inputs.len() > buffers.len() {
            let channels = self.channels.get() as usize;
            buffers.resize_with(inputs.len(), move || {
                let mut new = Buffer::default();
                new.resize(buffer_size, Stream::new(channels));
                new
            });
        }

        if buffers.is_empty() {
            // Delay always outputs at least one stream.
            outputs.push(Stream::default());
        } else {
            outputs.reserve(buffers.len());
            for (i, buffer) in buffers.iter_mut().enumerate() {
                if buffer.len() >= buffer_size {
                    outputs.push(buffer.pop_front().expect("buffer will not be empty"));
                } else {
                    // The buffer always outputs, even while it's filling.
                    outputs.push(Stream::default());
                }
                if buffer.len() < buffer_size {
                    buffer.push_back(inputs.get(i).copied().unwrap_or_default());
                }
            }
        }
    }

    fn set_channels(&self, channels: u16) {
        if self.channels.replace(channels) != channels {
            let channels = channels as usize;
            for buffer in self.buffers.borrow_mut().deref_mut() {
                let (a, b) = buffer.as_mut_slices();
                a.fill(Stream::new(channels));
                b.fill(Stream::new(channels));
            }
        }
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate.get()
    }

    fn get_channels(&self) -> u16 {
        self.channels.get()
    }

    fn node(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn Node> {
        self
    }
}
