use std::{
    fmt,
    hash::{Hash, Hasher},
    time::{Duration, TryFromFloatSecsError},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IllegalTime {
    NaN,
    Infinite,
    NegativeInfinite,
}

impl fmt::Display for IllegalTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IllegalTime::NaN => write!(f, "Time may not be NaN"),
            IllegalTime::Infinite => write!(f, "Time may not be Infinite"),
            IllegalTime::NegativeInfinite => write!(f, "Time may not be NegativeInfinite"),
        }
    }
}

impl std::error::Error for IllegalTime {}

/// A time value representing a finite number of seconds, which may be positive
/// or negative.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Time {
    seconds: f64,
}

impl Time {
    pub const ZERO: Time = Time { seconds: 0.0 };
    pub const MAX: Time = Time { seconds: f64::MAX };
    pub const MIN: Time = Time { seconds: f64::MIN };

    pub fn from_seconds(seconds: f64) -> Result<Self, IllegalTime> {
        if seconds.is_nan() {
            Err(IllegalTime::NaN)
        } else if seconds == f64::INFINITY {
            Err(IllegalTime::Infinite)
        } else if seconds == f64::NEG_INFINITY {
            Err(IllegalTime::NegativeInfinite)
        } else {
            Ok(Self { seconds })
        }
    }
    pub fn seconds(self) -> f64 {
        self.seconds
    }
}

impl From<Duration> for Time {
    fn from(value: Duration) -> Self {
        Self {
            seconds: value.as_secs_f64(),
        }
    }
}

impl TryFrom<Time> for Duration {
    type Error = TryFromFloatSecsError;

    fn try_from(value: Time) -> Result<Self, Self::Error> {
        Duration::try_from_secs_f64(value.seconds)
    }
}

impl Eq for Time {}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("One of the time values was invalid")
    }
}

impl Hash for Time {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u64(self.seconds.to_bits())
    }
}
