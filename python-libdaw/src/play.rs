use crate::{time::Duration, Node};
use libdaw::Sample;
use pyo3::{pyfunction, Bound, Python};
use rodio::{OutputStream, Sink};
use std::sync::mpsc::{sync_channel, Receiver};

#[derive(Debug)]
enum Message {
    Sample(Sample),
    Done,
}

#[derive(Debug, Default)]
enum SourceSample {
    #[default]
    NotStarted,
    Sample(<Sample as IntoIterator>::IntoIter),
    Done,
}

/// Rodio audio source
#[derive(Debug)]
pub struct Source {
    sample_rate: u32,
    channels: u16,
    receiver: Receiver<Message>,
    sample: SourceSample,
    duration: Option<std::time::Duration>,
}

impl Source {
    fn refresh(&mut self) {
        match &mut self.sample {
            SourceSample::Sample(source_sample) => {
                if source_sample.len() > 0 {
                    return;
                }
            }
            SourceSample::NotStarted => (),
            SourceSample::Done => return,
        }
        match self.receiver.recv() {
            Ok(Message::Sample(message_sample)) => {
                self.sample = SourceSample::Sample(message_sample.into_iter())
            }
            Ok(Message::Done) | Err(_) => self.sample = SourceSample::Done,
        }
    }
}

impl rodio::source::Source for Source {
    fn current_frame_len(&self) -> Option<usize> {
        match &self.sample {
            SourceSample::Sample(sample) => match sample.len() {
                0 => None,
                len => Some(len),
            },
            SourceSample::Done => Some(0),
            SourceSample::NotStarted => None,
        }
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.duration
    }
}
impl Iterator for Source {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.refresh();
        match &mut self.sample {
            SourceSample::Sample(sample) => Some(sample.next().unwrap() as f32),
            SourceSample::Done | SourceSample::NotStarted => None,
        }
    }
}

/// Play a node to the default speakers of the system.
#[pyfunction]
#[pyo3(signature = (node, sample_rate = 48000, channels=1, duration=None, grace_sleep=true))]
pub fn play(
    py: Python,
    node: &Bound<'_, Node>,
    sample_rate: u32,
    channels: u16,
    duration: Option<Duration>,
    grace_sleep: bool,
) -> crate::Result<()> {
    let start = std::time::Instant::now();
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let (sender, receiver) = sync_channel(sample_rate as usize * 10);
    let duration = duration.map(|duration| duration.0);
    let samples = duration
        .map(|duration| (duration.seconds() * sample_rate as f64) as u64)
        .unwrap_or(u64::MAX);
    let duration = duration.map(std::time::Duration::try_from).transpose()?;
    sink.append(Source {
        sample_rate,
        channels,
        receiver,
        sample: SourceSample::NotStarted,
        duration,
    });
    let node = node.borrow();
    let mut node = node.0.lock().expect("poisoned");
    let mut outputs = Vec::new();
    for _ in 0..samples {
        py.check_signals()?;
        outputs.clear();
        node.process(&[], &mut outputs)?;
        let mut sample = outputs
            .iter()
            .fold(None, move |acc, stream| match acc {
                Some(acc) => Some(acc + stream),
                None => Some(stream.clone()),
            })
            .unwrap_or_else(move || Sample::default());
        sample.channels.resize(channels as usize, 0.0);

        sender.send(Message::Sample(sample))?;
    }
    sender.send(Message::Done)?;
    while let Some(duration) = duration.and_then(|duration| duration.checked_sub(start.elapsed())) {
        // Still have some time left, sleep in hundredths of seconds so we
        // can check for ctrl-c still.
        py.check_signals()?;
        std::thread::sleep(duration.min(std::time::Duration::from_millis(10)));
    }

    // For some reason, rodio still gives an ugly cutoff if we don't do this
    if grace_sleep {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Ok(())
}
