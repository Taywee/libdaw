use super::{Duration, IllegalDuration};
use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, Sub},
    time,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IllegalTimestamp {
    NaN,
    Infinite,
    Negative,
}

impl fmt::Display for IllegalTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IllegalTimestamp::NaN => write!(f, "Timestamp may not be NaN"),
            IllegalTimestamp::Infinite => write!(f, "Timestamp may not be Infinite"),
            IllegalTimestamp::Negative => {
                write!(f, "Timestamp may not be Negative")
            }
        }
    }
}

impl std::error::Error for IllegalTimestamp {}

/// A timestamp value representing a finite number of seconds, which may be positive
/// or negative.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Timestamp {
    seconds: f64,
}

impl Timestamp {
    pub const ZERO: Timestamp = Timestamp { seconds: 0.0 };
    pub const MAX: Timestamp = Timestamp { seconds: f64::MAX };
    pub const MIN: Timestamp = Timestamp { seconds: 0.0 };

    pub fn from_seconds(seconds: f64) -> Result<Self, IllegalTimestamp> {
        if seconds.is_nan() {
            Err(IllegalTimestamp::NaN)
        } else if seconds < 0.0 {
            Err(IllegalTimestamp::Negative)
        } else if seconds == f64::INFINITY {
            Err(IllegalTimestamp::Infinite)
        } else {
            Ok(Self { seconds })
        }
    }
    pub fn seconds(self) -> f64 {
        self.seconds
    }
    pub fn try_sub(self, other: Self) -> Result<Duration, IllegalDuration> {
        Duration::from_seconds(self.seconds - other.seconds)
    }
    pub fn try_sub_duration(self, other: Duration) -> Result<Timestamp, IllegalTimestamp> {
        Timestamp::from_seconds(self.seconds - other.seconds())
    }
    pub fn try_add_duration(self, other: Duration) -> Result<Timestamp, IllegalTimestamp> {
        Timestamp::from_seconds(self.seconds + other.seconds())
    }
}

impl Sub for Timestamp {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.try_sub(rhs).expect("Invalid timestamp subtraction")
    }
}
impl Sub<Duration> for Timestamp {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.try_sub_duration(rhs)
            .expect("Invalid timestamp subtraction")
    }
}
impl Add<Duration> for Timestamp {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        self.try_add_duration(rhs)
            .expect("Invalid timestamp addition")
    }
}

impl TryFrom<Timestamp> for time::Duration {
    type Error = time::TryFromFloatSecsError;

    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        time::Duration::try_from_secs_f64(value.seconds)
    }
}

impl Eq for Timestamp {}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("One of the timestamp values was invalid")
    }
}

impl Hash for Timestamp {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u64(self.seconds.to_bits())
    }
}
