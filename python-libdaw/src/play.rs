use crate::Node;
use libdaw::Stream;
use pyo3::{pyfunction, Bound, Python};
use rodio::{OutputStream, Sink};
use std::{
    ops::Add,
    sync::mpsc::{sync_channel, Receiver},
};

/// Rodio audio source
#[derive(Debug)]
pub struct Source {
    sample_rate: u32,
    channels: u16,
    receiver: Receiver<Stream>,
    sample: <Stream as IntoIterator>::IntoIter,
}

impl Source {
    fn refresh(&mut self) {
        if self.sample.len() == 0 {
            if let Ok(sample) = self.receiver.recv() {
                self.sample = sample.into_iter();
            }
        }
    }
}

impl rodio::source::Source for Source {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
impl Iterator for Source {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.refresh();
        self.sample.next().map(|sample| sample as f32)
    }
}

/// Play a node to the default speakers of the system.
#[pyfunction]
pub fn play(
    py: Python,
    node: &Bound<'_, Node>,
    sample_rate: u32,
    channels: u16,
) -> crate::Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let (sender, receiver) = sync_channel(sample_rate as usize * 10);
    sink.append(Source {
        sample_rate,
        channels,
        receiver,
        // The initial sample is empty.
        sample: Stream::default().into_iter(),
    });
    let node = node.borrow().0.clone();
    let mut outputs = Vec::new();
    loop {
        py.check_signals()?;
        outputs.clear();
        node.process(&[], &mut outputs)?;
        let sample = outputs.iter().copied().reduce(Add::add);

        let sample = sample.unwrap_or_else(|| Stream::new(channels as usize));
        sender.send(sample)?;
    }
}