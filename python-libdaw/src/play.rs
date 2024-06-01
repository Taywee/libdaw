use crate::Node;
use libdaw::Sample;
use pyo3::{pyfunction, Bound, Python};
use rodio::{OutputStream, Sink};
use std::sync::mpsc::{sync_channel, Receiver};

/// Rodio audio source
#[derive(Debug)]
pub struct Source {
    sample_rate: u32,
    channels: u16,
    receiver: Receiver<Sample>,
    sample: <Sample as IntoIterator>::IntoIter,
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
#[pyo3(signature = (node, sample_rate = 48000, channels=2))]
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
        sample: Sample::default().into_iter(),
    });
    let node = node.borrow();
    let mut node = node.0.lock().expect("poisoned");
    let mut outputs = Vec::new();
    loop {
        py.check_signals()?;
        outputs.clear();
        node.process(&[], &mut outputs)?;
        let sample = outputs
            .iter()
            .fold(None, move |acc, stream| match acc {
                Some(acc) => Some(acc + stream),
                None => Some(stream.clone()),
            })
            .unwrap_or_else(move || Sample::zeroed(channels as usize));

        sender.send(sample)?;
    }
}
