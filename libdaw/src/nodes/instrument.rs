use crate::{
    nodes::{envelope::Point, graph::Index, Detune, Envelope, Graph},
    sync::AtomicF64,
    time::{Duration, Timestamp},
    DynNode as _, FrequencyNode, Node, Result,
};
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

/// A single tone definition.  Defined by frequency, not note name, to not tie
/// it to any particular tuning or scale.
/// Detuning and pitch bend should be done to the underlying frequency node.
#[derive(Debug, Clone, Copy)]
pub struct Tone {
    pub start: Timestamp,
    pub length: Duration,
    pub frequency: f64,
}

#[derive(Debug, Clone, Copy)]
struct QueuedTone {
    start_sample: u64,
    end_sample: u64,
    length: Duration,
    frequency: f64,
}
impl PartialOrd for QueuedTone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start_sample.partial_cmp(&other.start_sample)
    }
}

impl Ord for QueuedTone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_sample.cmp(&other.start_sample)
    }
}
impl PartialEq for QueuedTone {
    fn eq(&self, other: &Self) -> bool {
        self.start_sample.eq(&other.start_sample)
    }
}
impl Eq for QueuedTone {}

#[derive(Debug)]
struct PlayingTone {
    end_sample: u64,
    frequency_node: Arc<Detune>,
    frequency_node_index: Index,
    envelope_index: Index,
}

impl PartialOrd for PlayingTone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.end_sample.partial_cmp(&other.end_sample)
    }
}

impl Ord for PlayingTone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end_sample.cmp(&other.end_sample)
    }
}
impl PartialEq for PlayingTone {
    fn eq(&self, other: &Self) -> bool {
        self.end_sample.eq(&other.end_sample)
    }
}
impl Eq for PlayingTone {}

/// A node that can play a sequence of tones from a frequency node creator.
pub struct Instrument {
    frequency_node_creator: Box<Mutex<dyn FnMut() -> Result<Arc<dyn FrequencyNode>> + Send>>,
    graph: Graph,
    queue: Mutex<BinaryHeap<Reverse<QueuedTone>>>,
    playing: Mutex<BinaryHeap<Reverse<PlayingTone>>>,
    envelope: Vec<Point>,
    sample_rate: u32,
    sample: AtomicU64,
    detune: AtomicF64,
}

impl fmt::Debug for Instrument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instrument")
            .field("graph", &self.graph)
            .field("queue", &self.queue)
            .field("playing", &self.playing)
            .field("envelope", &self.envelope)
            .field("sample_rate", &self.sample_rate)
            .field("detune", &self.detune)
            .field("sample", &self.sample)
            .finish()
    }
}

impl Instrument {
    pub fn new(
        sample_rate: u32,
        frequency_node_creator: impl 'static + FnMut() -> Result<Arc<dyn FrequencyNode>> + Send,
        envelope: impl IntoIterator<Item = Point>,
    ) -> Self {
        Self {
            sample_rate,
            frequency_node_creator: Box::new(Mutex::new(frequency_node_creator)),
            graph: Default::default(),
            queue: Default::default(),
            playing: Default::default(),
            envelope: envelope.into_iter().collect(),
            detune: Default::default(),
            sample: Default::default(),
        }
    }

    pub fn add_tone(&self, tone: Tone) {
        let start_sample = (tone.start.seconds() * self.sample_rate as f64) as u64;
        let end = tone.start + tone.length;
        let end_sample = (end.seconds() * self.sample_rate as f64) as u64;
        if end_sample > start_sample {
            self.queue
                .lock()
                .expect("mutex poisoned")
                .push(Reverse(QueuedTone {
                    start_sample,
                    end_sample,
                    length: tone.length,
                    frequency: tone.frequency,
                }));
        }
    }

    /// Set the detune in the same way as the Detune.
    pub fn set_detune(&self, detune: f64) -> Result<()> {
        if self.detune.swap(detune, Ordering::Relaxed) != detune {
            for tone in self.playing.lock().expect("mutex poisoned").iter() {
                let tone = &tone.0;
                tone.frequency_node.set_detune(detune)?;
            }
        }
        Ok(())
    }
}

impl Node for Instrument {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [crate::stream::Stream],
        outputs: &'c mut Vec<crate::stream::Stream>,
    ) -> Result<()> {
        let sample = self
            .sample
            .swap(self.sample.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
        let detune = self.detune.load(Ordering::Relaxed);

        let mut queue = self.queue.lock().expect("mutex poisoned");
        let mut playing = self.playing.lock().expect("mutex poisoned");
        let mut frequency_node_creator =
            self.frequency_node_creator.lock().expect("mutex poisoned");
        if queue.is_empty() && playing.is_empty() {
            return Ok(());
        }

        // Spawn all ready queued tones.
        loop {
            let Some(tone) = queue.peek() else {
                break;
            };
            if sample < tone.0.start_sample {
                break;
            }

            let tone = queue.pop().unwrap().0;
            let frequency_node = Arc::new(Detune::new(frequency_node_creator()?));
            frequency_node.set_frequency(tone.frequency)?;
            frequency_node.set_detune(detune)?;

            let envelope = Arc::new(Envelope::new(
                self.sample_rate,
                tone.length,
                self.envelope.iter().copied(),
            ));
            let frequency_node_index = self.graph.add(frequency_node.clone().node());
            let envelope_index = self.graph.add(envelope.clone());
            self.graph
                .connect(frequency_node_index, envelope_index, None)?;
            self.graph.output(envelope_index, None)?;
            self.graph.input(frequency_node_index, None)?;
            playing.push(Reverse(PlayingTone {
                end_sample: tone.end_sample,
                frequency_node,
                envelope_index,
                frequency_node_index,
            }));
        }

        loop {
            let Some(tone) = playing.peek() else {
                break;
            };
            if sample < tone.0.end_sample {
                break;
            }

            let tone = playing.pop().unwrap().0;
            self.graph.remove(tone.frequency_node_index)?;
            self.graph.remove(tone.envelope_index)?;
        }

        // Play graph
        self.graph.process(inputs, outputs)
    }
}
