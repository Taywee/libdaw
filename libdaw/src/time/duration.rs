use std::{fmt, ops::Add, time};

use super::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IllegalDuration {
    NaN,
    Infinite,
    Negative,
}

impl fmt::Display for IllegalDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IllegalDuration::NaN => write!(f, "Duration may not be NaN"),
            IllegalDuration::Infinite => write!(f, "Duration may not be Infinite"),
            IllegalDuration::Negative => {
                write!(f, "Duration may not be NegativeInfinite")
            }
        }
    }
}

impl std::error::Error for IllegalDuration {}

/// A time difference value representing a finite number of seconds, which may
/// not be negative.
#[derive(Debug, Clone, Copy)]
pub struct Duration {
    seconds: f64,
}

impl Duration {
    pub const ZERO: Duration = Duration { seconds: 0.0 };
    pub const MAX: Duration = Duration { seconds: f64::MAX };
    pub const MIN: Duration = Duration { seconds: 0.0 };

    pub fn from_seconds(seconds: f64) -> Result<Self, IllegalDuration> {
        if seconds.is_nan() {
            Err(IllegalDuration::NaN)
        } else if seconds < 0.0 {
            Err(IllegalDuration::Negative)
        } else if seconds == f64::INFINITY {
            Err(IllegalDuration::Infinite)
        } else {
            Ok(Self { seconds })
        }
    }
    pub fn seconds(self) -> f64 {
        self.seconds
    }
}

impl Add<Timestamp> for Duration {
    type Output = Timestamp;

    fn add(self, rhs: Timestamp) -> Self::Output {
        rhs + self
    }
}

impl From<time::Duration> for Duration {
    fn from(value: time::Duration) -> Self {
        Self {
            seconds: value.as_secs_f64(),
        }
    }
}

impl TryFrom<Duration> for time::Duration {
    type Error = time::TryFromFloatSecsError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        time::Duration::try_from_secs_f64(value.seconds)
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.seconds.eq(&other.seconds)
    }
}

impl Eq for Duration {}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.seconds.partial_cmp(&other.seconds)
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("One of the timestamp values was invalid")
    }
}
