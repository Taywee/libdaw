mod parse;

use crate::{parse::IResult, time::Timestamp};
use ordered_float::OrderedFloat;
use std::{
    fmt,
    hash::{Hash, Hasher},
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IllegalBeat {
    NaN,
    Negative,
}

impl fmt::Display for IllegalBeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IllegalBeat::NaN => write!(f, "Beat may not be NaN"),
            IllegalBeat::Negative => {
                write!(f, "Beat may not be Negative")
            }
        }
    }
}

impl std::error::Error for IllegalBeat {}

/// A representation of a beat, a non-negative floating point number.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Beat(f64);

impl Eq for Beat {}

impl Hash for Beat {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u64(self.0.to_bits())
    }
}

impl Ord for Beat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Beat may not be NaN")
    }
}

impl Beat {
    pub const ZERO: Beat = Beat(0.0);
    pub const ONE: Beat = Beat(1.0);

    pub fn new(beat: f64) -> Result<Self, IllegalBeat> {
        if beat >= 0.0 {
            Ok(Self(beat))
        } else if beat.is_nan() {
            Err(IllegalBeat::NaN)
        } else {
            Err(IllegalBeat::Negative)
        }
    }
    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    pub fn checked_add(self, other: Self) -> Result<Self, IllegalBeat> {
        Self::new(self.0 + other.0)
    }

    pub fn checked_sub(self, other: Self) -> Result<Self, IllegalBeat> {
        Self::new(self.0 - other.0)
    }

    pub fn checked_mul(self, other: f64) -> Result<Self, IllegalBeat> {
        Self::new(self.0 * other)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::beat(input)
    }
}

impl Add for Beat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).expect("added to illegal beat")
    }
}

impl AddAssign for Beat {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.checked_add(rhs).expect("added to illegal beat").0;
    }
}

impl Sub for Beat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).expect("subtracted to illegal beat")
    }
}

impl SubAssign for Beat {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.checked_sub(rhs).expect("subtracted to illegal beat").0;
    }
}

impl Sum for Beat {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::new(iter.map(|each| each.0).sum()).expect("summed to illegal beat")
    }
}

impl Mul<f64> for Beat {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.checked_mul(rhs)
            .expect("multiplied with illegal value")
    }
}

impl MulAssign<f64> for Beat {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 = self
            .checked_mul(rhs)
            .expect("multiplied with illegal value")
            .0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IllegalBeatsPerMinute {
    NaN,
    NonPositive,
}

impl fmt::Display for IllegalBeatsPerMinute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IllegalBeatsPerMinute::NaN => write!(f, "BeatsPerMinute may not be NaN"),
            IllegalBeatsPerMinute::NonPositive => {
                write!(f, "BeatsPerMinute must be positive")
            }
        }
    }
}

impl std::error::Error for IllegalBeatsPerMinute {}

/// A representation of a tempo, a positive floating point number.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct BeatsPerMinute(f64);

impl Eq for BeatsPerMinute {}

impl Ord for BeatsPerMinute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("BeatsPerMinute may not be NaN")
    }
}

impl Hash for BeatsPerMinute {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u64(self.0.to_bits())
    }
}

impl BeatsPerMinute {
    pub fn new(beats_per_minute: f64) -> Result<Self, IllegalBeatsPerMinute> {
        if beats_per_minute > 0.0 {
            Ok(Self(beats_per_minute))
        } else if beats_per_minute.is_nan() {
            Err(IllegalBeatsPerMinute::NaN)
        } else {
            Err(IllegalBeatsPerMinute::NonPositive)
        }
    }
    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    pub fn checked_add(self, other: Self) -> Result<Self, IllegalBeatsPerMinute> {
        Self::new(self.0 + other.0)
    }
}
impl Add for BeatsPerMinute {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).expect("added to illegal beat")
    }
}

impl AddAssign for BeatsPerMinute {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.checked_add(rhs).expect("added to illegal beat").0;
    }
}

impl Sum for BeatsPerMinute {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::new(iter.map(|each| each.0).sum()).expect("summed to illegal beat")
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TempoInstruction {
    /// The beat to apply this instruction at.
    /// Must be non-negative.
    pub beat: Beat,

    /// The beats per minute for this instruction.
    /// Must be positive.
    pub tempo: BeatsPerMinute,
}

/// Internal tempo instruction, with each beat correlated to a concrete time.
/// This allows us to calculate the beat-to-time intersection.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
struct CalculatedTempoInstruction {
    beat: f64,
    time: f64,
    seconds_per_beat: f64,
}

/// Calculates a specific time for each specific beat, and vice-versa.
/// Fractional beats and times work too. Instructions are linearly interpolated,
/// and order is significant.  Two instructions can share the same beat, which
/// is the only way to suddenly change tempo without any interpolation (each
/// two adjacent instructions are lerped between, so two who share the same beat
/// will be a sudden tempo change).
/// If no instructions are supplied, a default bpm of 128 will be used.
/// All beats before the first instruction use the first instruction's timings,
/// and all instructions after the last instruction use the last instruction's
/// timing.  The times are not extrapolated.
#[derive(Debug, Default)]
pub struct Metronome {
    instructions: Vec<CalculatedTempoInstruction>,
}

impl Metronome {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_tempo_instruction(&mut self, instruction: TempoInstruction) {
        self.instructions.push(CalculatedTempoInstruction {
            beat: instruction.beat.get(),
            time: 0.0f64,
            seconds_per_beat: 60.0 / instruction.tempo.get(),
        });

        // Sort must be stable.
        self.instructions
            .sort_by_key(|instruction| OrderedFloat(instruction.beat));

        // It's inefficient to run a full recalculation every time we add a new
        // instruction, but it should be infrequent enough to make very little
        // difference.
        let mut last = CalculatedTempoInstruction {
            beat: 0.0,
            time: 0.0,
            seconds_per_beat: self.instructions[0].seconds_per_beat,
        };

        for instruction in &mut self.instructions {
            let time = if instruction.beat == last.beat {
                last.time
            } else {
                Self::integrate_beat(last, *instruction, instruction.beat)
            };
            instruction.time = time;
            last = *instruction;
        }
    }

    pub fn beat_to_time(&self, beat: Beat) -> Timestamp {
        let instructions_len = self.instructions.len();
        let beat = beat.get();

        Timestamp::from_seconds(match instructions_len {
            // Default to 128 beats per second unless otherwise specified.
            0 => (60.0 / 128.0) * beat,
            1 => self.instructions[0].seconds_per_beat * beat,
            _ => {
                match self
                    .instructions
                    .binary_search_by_key(&OrderedFloat(beat), |instruction| {
                        OrderedFloat(instruction.beat)
                    }) {
                    // Exact match, just use the time value given
                    Ok(index) => self.instructions[index].time,

                    // Before first element, just use first beat markings.
                    Err(0) => self.instructions[0].seconds_per_beat * beat,

                    // After last element, just use last beat markings.
                    Err(index) if index == instructions_len => {
                        let last = self.instructions[instructions_len - 1];
                        let beats_left = beat - last.beat;
                        let time_since_last = beats_left * last.seconds_per_beat;
                        last.time + time_since_last
                    }

                    // Between two elements, need to use the definite integral.
                    Err(index) => {
                        let a = self.instructions[index - 1];
                        let b = self.instructions[index];
                        Self::integrate_beat(a, b, beat)
                    }
                }
            }
        })
        .expect("Time ended up negative or NaN")
    }

    // Integrate a beat between two endcap instructions to find its time.
    // b.beat must be > a.beat
    fn integrate_beat(
        a: CalculatedTempoInstruction,
        b: CalculatedTempoInstruction,
        beat: f64,
    ) -> f64 {
        // spb = seconds per beat
        // b = beat
        // spb = seconds per beat
        // s = time in seconds
        // m = (spb2 - spb1) / (b2 - b1)
        // spb = m * (b - b1) + spb1
        //     = m * b - m * b1 + spb1
        // ∫spb db =  m * b ^ 2 / 2 + (-m * b1 + spb1) * b + C
        // s = m * b ^ 2 / 2 + (-m * b1 + spb1) * b - (m * b1 ^ 2 / 2 + (-m * b1 + spb1) * b1)
        // s = m * b ^ 2 / 2 + (-m * b1 + spb1) * b - m * b1 ^ 2 / 2 - (-m * b1 + spb1) * b1
        // s = m * (b^2 - b1^2) / 2 + (b - b1) * (-m * b1 + spb1)
        let b1 = a.beat;
        let spb1 = a.seconds_per_beat;
        let b2 = b.beat;
        let spb2 = b.seconds_per_beat;
        let m = (spb2 - spb1) / (b2 - b1);
        let time = m * (beat.powi(2) - b1.powi(2)) / 2.0 + (beat - b1) * (-m * b1 + spb1);

        a.time + time
    }
}
