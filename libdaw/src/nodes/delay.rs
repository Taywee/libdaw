use std::collections::VecDeque;
use std::time::Duration;

use crate::stream::Stream;
use crate::Node;

#[derive(Debug)]
pub struct Delay {
    buffers: Vec<VecDeque<Stream>>,
    buffer_size: usize,
    sample_rate: f64,
    delay: Duration,
    channels: u16,
}

impl Delay {
    pub fn new(delay: Duration) -> Self {
        let mut node = Self {
            buffers: Default::default(),
            buffer_size: Default::default(),
            delay,
            sample_rate: 48000.0,
            channels: Default::default(),
        };
        node.update_buffer_size();
        node
    }

    // We might want to remove this.  Setting a delay to a shorter time will
    // either not work or have to truncate the existing buffer.
    // Then again, the same will happen for set_sample rate.
    pub fn set_delay(&mut self, delay: Duration) {
        self.delay = delay;
        self.update_buffer_size();
    }

    pub fn get_delay(&mut self) -> Duration {
        self.delay
    }

    fn update_buffer_size(&mut self) {
        self.buffer_size = (self.delay.as_secs_f64() * self.sample_rate).round() as usize;
        for buffer in &mut self.buffers {
            let capacity = buffer.capacity();
            if capacity < self.buffer_size {
                buffer.reserve_exact(capacity - self.buffer_size);
            }
        }
    }
}

impl Node for Delay {
    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate.into();
        self.update_buffer_size();
    }

    fn process<'a, 'b>(&'a mut self, inputs: &'b [Stream], outputs: &'a mut Vec<Stream>) {
        if self.buffer_size == 0 {
            outputs.extend_from_slice(inputs);
            return;
        }
        if inputs.len() > self.buffers.len() {
            self.buffers.resize_with(inputs.len(), || {
                let mut new = VecDeque::default();
                new.reserve_exact(self.buffer_size);
                new
            });
        }

        outputs.reserve_exact(self.buffers.len());
        for (i, buffer) in self.buffers.iter_mut().enumerate() {
            // TODO: An input that is added and removed in quick succession can
            // cause an earlier-than-desired delay result. We should fix that
            // with a dedicated buffer type that does not drain until it has
            // been filled, and fills itself with zeros when it has no inputs.
            if i >= inputs.len() {
                // The buffer is being drained.
                outputs.push(
                    buffer
                        .pop_front()
                        .expect("buffer should never be left empty"),
                );
            } else {
                if buffer.len() >= self.buffer_size {
                    outputs.push(
                        buffer
                            .pop_front()
                            .expect("buffer should never be left empty"),
                    );
                } else {
                    // Return 0 while the buffer is filling.
                    outputs.push(Stream::new(self.channels.into()));
                }

                // This should always be the case unless the delay is changed during processing.
                if buffer.len() < self.buffer_size {
                    buffer.push_back(inputs[i]);
                }
            }
        }
        self.buffers.retain(|e| !e.is_empty());
    }

    fn set_channels(&mut self, channels: u16) {
        self.channels = channels;
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate as u32
    }

    fn get_channels(&self) -> u16 {
        self.channels
    }
}
