use super::{envelope_node::EnvelopePoint, graph::Index, Detune, EnvelopeNode, Graph};
use crate::{DynNode as _, FrequencyNode, Node};
use std::{
    cell::{Cell, RefCell},
    cmp::Reverse,
    collections::BinaryHeap,
    fmt,
    rc::Rc,
    time::Duration,
};

/// A single note definition.  Defined by frequency, not note name, to not tie
/// it to any particular tuning or scale.
/// Detuning and pitch bend should be done to the underlying frequency node.
#[derive(Debug, Clone)]
pub struct Note {
    pub start: Duration,
    pub length: Duration,
    pub frequency: f64,
}

#[derive(Debug, Clone)]
struct QueuedNote {
    start_sample: u64,
    length: Duration,
    frequency: f64,
}
impl PartialOrd for QueuedNote {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start_sample.partial_cmp(&other.start_sample)
    }
}

impl Ord for QueuedNote {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_sample.cmp(&other.start_sample)
    }
}
impl PartialEq for QueuedNote {
    fn eq(&self, other: &Self) -> bool {
        self.start_sample.eq(&other.start_sample)
    }
}
impl Eq for QueuedNote {}

#[derive(Debug)]
struct PlayingNote {
    end_sample: u64,
    frequency_node: Rc<Detune>,
    frequency_node_index: Index,
    envelope_node_index: Index,
}

impl PartialOrd for PlayingNote {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.end_sample.partial_cmp(&other.end_sample)
    }
}

impl Ord for PlayingNote {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end_sample.cmp(&other.end_sample)
    }
}
impl PartialEq for PlayingNote {
    fn eq(&self, other: &Self) -> bool {
        self.end_sample.eq(&other.end_sample)
    }
}
impl Eq for PlayingNote {}

/// A node that can play a sequence of notes from a frequency node creator.
pub struct Instrument {
    frequency_node_creator: Box<RefCell<dyn FnMut() -> Rc<dyn FrequencyNode>>>,
    graph: Graph,
    queue: RefCell<BinaryHeap<Reverse<QueuedNote>>>,
    playing: RefCell<BinaryHeap<Reverse<PlayingNote>>>,
    envelope: Vec<EnvelopePoint>,
    sample_rate: u32,
    sample: Cell<u64>,
    detune: Cell<f64>,
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
        frequency_node_creator: impl 'static + FnMut() -> Rc<dyn FrequencyNode>,
        envelope: impl IntoIterator<Item = EnvelopePoint>,
    ) -> Self {
        Self {
            sample_rate,
            frequency_node_creator: Box::new(RefCell::new(frequency_node_creator)),
            graph: Default::default(),
            queue: Default::default(),
            playing: Default::default(),
            envelope: envelope.into_iter().collect(),
            detune: Default::default(),
            sample: Default::default(),
        }
    }

    pub fn add_note(&self, note: Note) {
        let start_sample = (note.start.as_secs_f64() * self.sample_rate as f64) as u64;
        self.queue.borrow_mut().push(Reverse(QueuedNote {
            start_sample,
            length: note.length,
            frequency: note.frequency,
        }));
    }

    /// Set the detune in the same way as the Detune.
    pub fn set_detune(&self, detune: f64) {
        if self.detune.replace(detune) != detune {
            for note in self.playing.borrow().iter() {
                let note = &note.0;
                note.frequency_node.set_detune(detune);
            }
        }
    }
}

impl Node for Instrument {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [crate::stream::Stream],
        outputs: &'c mut Vec<crate::stream::Stream>,
    ) {
        let sample = self.sample.replace(self.sample.get() + 1);
        let detune = self.detune.get();

        let mut queue = self.queue.borrow_mut();
        let mut playing = self.playing.borrow_mut();
        let mut frequency_node_creator = self.frequency_node_creator.borrow_mut();
        if queue.is_empty() && playing.is_empty() {
            return;
        }

        // Spawn all ready queued notes.
        loop {
            let Some(note) = queue.peek() else {
                break;
            };
            if sample < note.0.start_sample {
                break;
            }

            let note = queue.pop().unwrap().0;
            let sample_length = (note.length.as_secs_f64() * self.sample_rate as f64) as u64;
            let end_sample = note.start_sample + sample_length;

            let frequency_node = Rc::new(Detune::new(frequency_node_creator()));
            frequency_node.set_frequency(note.frequency);
            frequency_node.set_detune(detune);

            let envelope_node = Rc::new(EnvelopeNode::new(
                self.sample_rate,
                note.length,
                self.envelope.iter().copied(),
            ));
            let frequency_node_index = self.graph.add(frequency_node.clone().node());
            let envelope_node_index = self.graph.add(envelope_node.clone());
            self.graph
                .connect(frequency_node_index, envelope_node_index, None);
            self.graph.output(envelope_node_index, None);
            self.graph.input(frequency_node_index, None);
            playing.push(Reverse(PlayingNote {
                end_sample,
                frequency_node,
                envelope_node_index,
                frequency_node_index,
            }));
        }

        // Remove finished notes
        loop {
            let Some(note) = playing.peek() else {
                break;
            };
            if sample < note.0.end_sample {
                break;
            }

            let note = playing.pop().unwrap().0;
            self.graph.remove(note.frequency_node_index);
            self.graph.remove(note.envelope_node_index);
        }

        // Play graph
        self.graph.process(inputs, outputs);
    }
}
