use crate::{
    stream::Stream,
    time::{Duration, Time},
    Node, Result,
};
use std::cell::Cell;

#[derive(Debug, Clone, Copy)]
pub enum Offset {
    /// Calculate time from `whence`.
    Time(Time),

    /// Calculate time as a ratio of the length.  This may be negative.
    Ratio(f64),
}

impl Default for Offset {
    fn default() -> Self {
        Self::Time(Time::ZERO)
    }
}

#[derive(Debug, Default, Clone, Copy)]
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
#[derive(Debug, Default, Clone, Copy)]
struct CalculatedEnvelopePoint {
    sample: u64,
    volume: f64,
}

impl PartialOrd for CalculatedEnvelopePoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.sample.partial_cmp(&other.sample)
    }
}

impl PartialEq for CalculatedEnvelopePoint {
    fn eq(&self, other: &Self) -> bool {
        self.sample.eq(&other.sample)
    }
}

impl Eq for CalculatedEnvelopePoint {}

impl Ord for CalculatedEnvelopePoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sample.cmp(&other.sample)
    }
}

/// A frequency node wrapper that applies a volume envelope to the node.
#[derive(Debug)]
pub struct Envelope {
    envelope: Box<[CalculatedEnvelopePoint]>,
    sample: Cell<u64>,
}

impl Envelope {
    /// Construct the envelope.  If you give zero envelope points, this will
    /// effectively be a PassthroughNode.
    pub fn new(
        sample_rate: u32,
        length: Duration,
        envelope: impl IntoIterator<Item = EnvelopePoint>,
    ) -> Self {
        let mut envelope: Vec<CalculatedEnvelopePoint> = envelope
            .into_iter()
            .flat_map(move |point| {
                let length = length.seconds();
                let whence = length * point.whence;
                let time = match point.offset {
                    Offset::Time(offset) => whence + offset.seconds(),
                    Offset::Ratio(offset) => {
                        let offset = length * offset;
                        whence + offset
                    }
                };
                if time.is_nan() {
                    return None;
                }
                Some(CalculatedEnvelopePoint {
                    sample: (time * sample_rate as f64) as u64,
                    volume: point.volume,
                })
            })
            .collect();
        envelope.sort();
        Self {
            envelope: envelope.into(),
            sample: 0.into(),
        }
    }
}

impl Node for Envelope {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [Stream],
        outputs: &'c mut Vec<Stream>,
    ) -> Result<()> {
        outputs.extend_from_slice(inputs);

        let envelope_len = self.envelope.len();
        let volume = match envelope_len {
            0 => return Ok(()),
            1 => self.envelope[0].volume,
            _ => {
                let sample = self.sample.replace(self.sample.get() + 1);
                match self
                    .envelope
                    .binary_search_by_key(&sample, |point| point.sample)
                {
                    Ok(index) => self.envelope[index].volume,
                    Err(index) => {
                        // Find the interpolaton points based on the insertion.
                        let (a, b) = if index == 0 {
                            // Before beginning; extrapolate backward.
                            (&self.envelope[0], &self.envelope[1])
                        } else if index == envelope_len {
                            // After end; extrapolate forward.
                            (
                                &self.envelope[envelope_len - 2],
                                &self.envelope[envelope_len - 1],
                            )
                        } else {
                            // Between two points; interpolate.
                            (&self.envelope[index - 1], &self.envelope[index])
                        };
                        let sample = sample as f64;
                        let a_sample = a.sample as f64;
                        let b_sample = b.sample as f64;
                        // Lerp, given x as a time scale and y as volume.
                        a.volume
                            + (sample - a_sample) * (b.volume - a.volume) / (b_sample - a_sample)
                    }
                }
            }
        };
        for output in outputs {
            *output *= volume;
        }
        Ok(())
    }
}
