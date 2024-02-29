use crate::{stream::Stream, FrequencyNode, Node};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

#[derive(Debug)]
pub enum Offset {
    /// Calculate time after `whence`.
    TimeForward(Duration),

    /// Calculate time before `whence`.
    TimeBackward(Duration),

    /// Calculate time as a ratio of the length.  This may be negative.
    Ratio(f64),
}

impl Default for Offset {
    fn default() -> Self {
        Self::TimeForward(Duration::ZERO)
    }
}

#[derive(Debug, Default)]
pub struct EnvelopePoint {
    /// The offset, relative to `whence`
    pub offset: Offset,

    /// As a ratio of the note length.  0 is the beginning of the note, and 1 is the end of the note.
    pub whence: f64,

    /// From 0 to 1, the volume of the point.
    pub volume: f64,
}

/// Internal envelope point, with offset and whence turned into a concrete
/// start time.
#[derive(Debug, Default)]
struct CalculatedEnvelopePoint {
    time: Duration,
    sample: f64,
    volume: f64,
}

impl PartialOrd for CalculatedEnvelopePoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl PartialEq for CalculatedEnvelopePoint {
    fn eq(&self, other: &Self) -> bool {
        self.time.eq(&other.time)
    }
}

impl Eq for CalculatedEnvelopePoint {}

impl Ord for CalculatedEnvelopePoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

/// A frequency node wrapper that applies a volume envelope to the node.
#[derive(Debug)]
pub struct EnvelopedFrequencyNode {
    node: Rc<dyn FrequencyNode>,
    envelope: RefCell<Vec<CalculatedEnvelopePoint>>,
    sample: Cell<u64>,
}

impl EnvelopedFrequencyNode {
    pub fn new(
        node: Rc<dyn FrequencyNode>,
        length: Duration,
        envelope: impl IntoIterator<Item = EnvelopePoint>,
    ) -> Self {
        let mut envelope: Vec<CalculatedEnvelopePoint> = envelope
            .into_iter()
            .flat_map(move |point| {
                Duration::try_from_secs_f64(length.as_secs_f64() * point.whence)
                    .ok()
                    .and_then(move |whence| {
                        let start_time = match point.offset {
                            Offset::TimeForward(offset) => Some(whence + offset),
                            Offset::TimeBackward(offset) => whence.checked_sub(offset),
                            Offset::Ratio(offset) => {
                                if offset >= 0.0 {
                                    let offset = length.mul_f64(offset);
                                    Some(whence + offset)
                                } else {
                                    let offset = length.mul_f64(-offset);
                                    whence.checked_sub(offset)
                                }
                            }
                        };
                        start_time.map(move |start_time| CalculatedEnvelopePoint {
                            time: start_time,
                            sample: 0.0,
                            volume: point.volume,
                        })
                    })
            })
            .collect();
        envelope.sort();
        let node = Self {
            node,
            envelope: RefCell::new(envelope),
            sample: 0.into(),
        };
        node.calculate_samples();
        node
    }

    fn calculate_samples(&self) {
        let sample_rate = self.node.get_sample_rate() as f64;
        let mut envelope = self.envelope.borrow_mut();
        for point in envelope.iter_mut() {
            let seconds = point.time.as_secs_f64();
            point.sample = sample_rate * seconds;
        }
    }
}

impl Node for EnvelopedFrequencyNode {
    fn set_sample_rate(&self, sample_rate: u32) {
        self.node.set_sample_rate(sample_rate);
        self.calculate_samples();
    }

    fn set_channels(&self, channels: u16) {
        self.node.set_channels(channels);
    }

    fn get_sample_rate(&self) -> u32 {
        self.node.get_sample_rate()
    }

    fn get_channels(&self) -> u16 {
        self.node.get_channels()
    }

    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        self.node.process(inputs, outputs);

        let envelope = self.envelope.borrow_mut();
        let sample = self.sample.replace(self.sample.get() + 1) as f64;

        let envelope_len = envelope.len();
        let volume = match envelope_len {
            0 => return,
            1 => envelope[0].volume,
            _ => {
                match envelope.binary_search_by(|point| point.sample.partial_cmp(&sample).unwrap())
                {
                    Ok(index) => envelope[index].volume,
                    Err(index) => {
                        // Find the interpolaton points based on the insertion.
                        let (a, b) = if index == 0 {
                            // Before beginning; extrapolate backward.
                            (&envelope[0], &envelope[1])
                        } else if index == envelope_len {
                            // After end; extrapolate forward.
                            (&envelope[envelope_len - 2], &envelope[envelope_len - 1])
                        } else {
                            // Between two points; interpolate.
                            (&envelope[index - 1], &envelope[index])
                        };
                        // Lerp, given x as a time scale and y as volume.
                        a.volume
                            + (sample - a.sample) * (b.volume - a.volume) / (b.sample - a.sample)
                    }
                }
            }
        };
        for output in outputs {
            *output *= volume;
        }
    }

    fn node(self: Rc<Self>) -> Rc<dyn Node> {
        self
    }
}

impl FrequencyNode for EnvelopedFrequencyNode {
    fn get_frequency(&self) -> f64 {
        self.node.get_frequency()
    }

    fn set_frequency(&self, frequency: f64) {
        self.node.set_frequency(frequency);
    }

    fn frequency_node(self: Rc<Self>) -> Rc<dyn FrequencyNode> {
        self
    }
}
