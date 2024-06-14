use crate::{
    nodes::{envelope::Point, ConstantValue, Envelope, Graph},
    time::{Duration, Timestamp},
    Node, Result,
};
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt,
    sync::{Arc, Mutex},
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
    tone: Tone,
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
    graph: Arc<Mutex<Graph>>,
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

/// A node that can play a sequence of tones from a node creator.
pub struct Instrument {
    node_creator: Box<dyn FnMut(Tone) -> Result<Arc<Mutex<dyn Node>>> + Send>,
    graph: Graph,
    queue: BinaryHeap<Reverse<QueuedTone>>,
    playing: BinaryHeap<Reverse<PlayingTone>>,
    envelope: Vec<Point>,
    sample_rate: u32,
    sample: u64,
}

impl fmt::Debug for Instrument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instrument")
            .field("graph", &self.graph)
            .field("queue", &self.queue)
            .field("playing", &self.playing)
            .field("envelope", &self.envelope)
            .field("sample_rate", &self.sample_rate)
            .field("sample", &self.sample)
            .finish()
    }
}

impl Instrument {
    pub fn new(
        sample_rate: u32,
        frequency_node_creator: impl 'static + FnMut(Tone) -> Result<Arc<Mutex<dyn Node>>> + Send,
        envelope: impl IntoIterator<Item = Point>,
    ) -> Self {
        Self {
            sample_rate,
            node_creator: Box::new(frequency_node_creator),
            graph: Default::default(),
            queue: Default::default(),
            playing: Default::default(),
            envelope: envelope.into_iter().collect(),
            sample: Default::default(),
        }
    }

    pub fn add_tone(&mut self, tone: Tone) {
        let start_sample = (tone.start.seconds() * self.sample_rate as f64) as u64;
        let end = tone.start + tone.length;
        let end_sample = (end.seconds() * self.sample_rate as f64) as u64;
        if end_sample > start_sample {
            self.queue.push(Reverse(QueuedTone {
                start_sample,
                end_sample,
                length: tone.length,
                frequency: tone.frequency,
                tone,
            }));
        }
    }
}

impl Node for Instrument {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [crate::sample::Sample],
        outputs: &'c mut Vec<crate::sample::Sample>,
    ) -> Result<()> {
        let sample = self.sample;
        self.sample += 1;

        if self.queue.is_empty() && self.playing.is_empty() {
            return Ok(());
        }

        // Spawn all ready queued tones.
        loop {
            let Some(tone) = self.queue.peek() else {
                break;
            };
            if sample < tone.0.start_sample {
                break;
            }

            let tone = self.queue.pop().unwrap().0;
            let constant_value = Arc::new(Mutex::new(ConstantValue::new(tone.frequency)));
            let frequency_node = (self.node_creator)(tone.tone)?;

            let envelope = Arc::new(Mutex::new(Envelope::new(
                self.sample_rate,
                tone.length,
                self.envelope.iter().copied(),
            )));
            let mut graph = Graph::default();
            graph.connect(constant_value.clone(), frequency_node.clone(), None);
            graph.connect(frequency_node.clone(), envelope.clone(), None);
            graph.output(envelope.clone(), None);
            let graph = Arc::new(Mutex::new(graph));
            self.graph.output(graph.clone(), None);
            self.playing.push(Reverse(PlayingTone {
                end_sample: tone.end_sample,
                graph,
            }));
        }

        while self
            .playing
            .peek()
            .is_some_and(|tone| sample >= tone.0.end_sample)
        {
            let tone = self.playing.pop().unwrap().0;
            self.graph.remove(tone.graph);
        }

        // Play graph
        self.graph.process(inputs, outputs)
    }
}
