use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign};

pub const MAX_CHANNELS: usize = 6;

pub type IntoIter = std::iter::Take<std::array::IntoIter<f64, MAX_CHANNELS>>;

#[derive(Debug, Clone, Copy, Default)]
pub struct Stream {
    channels: usize,

    samples: [f64; MAX_CHANNELS],
}

impl IntoIterator for Stream {
    type Item = f64;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.samples.into_iter().take(self.channels)
    }
}
impl<'a> IntoIterator for &'a Stream {
    type Item = &'a f64;

    type IntoIter = std::slice::Iter<'a, f64>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.samples[..self.channels]).into_iter()
    }
}

impl Stream {
    pub fn new(channels: usize) -> Self {
        assert!(channels <= MAX_CHANNELS);
        Self {
            channels,
            samples: [0.0; MAX_CHANNELS],
        }
    }

    pub fn from_raw_parts(samples: [f64; MAX_CHANNELS], channels: usize) -> Self {
        assert!(channels <= MAX_CHANNELS);
        Self { channels, samples }
    }

    pub fn channels(&self) -> usize {
        self.channels
    }
}

impl Deref for Stream {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        &self.samples[..self.channels]
    }
}

impl DerefMut for Stream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.samples[..self.channels]
    }
}

impl AddAssign<&Stream> for Stream {
    fn add_assign(&mut self, rhs: &Stream) {
        assert_eq!(self.channels, rhs.channels);
        for (l, &r) in self.samples.iter_mut().zip(&rhs.samples) {
            *l += r;
        }
    }
}

impl AddAssign for Stream {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.channels, rhs.channels);
        for (l, r) in self.samples.iter_mut().zip(rhs.samples) {
            *l += r;
        }
    }
}
impl Add for &Stream {
    type Output = Stream;

    fn add(self, rhs: &Stream) -> Self::Output {
        let mut output = self.clone();
        output += rhs;
        output
    }
}

impl Add<Stream> for &Stream {
    type Output = Stream;

    fn add(self, rhs: Stream) -> Self::Output {
        let mut output = self.clone();
        output += rhs;
        output
    }
}
impl Add<&Stream> for Stream {
    type Output = Stream;

    fn add(mut self, rhs: &Stream) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add for Stream {
    type Output = Stream;

    fn add(mut self, rhs: Stream) -> Self::Output {
        self += rhs;
        self
    }
}

impl MulAssign<&Stream> for Stream {
    fn mul_assign(&mut self, rhs: &Stream) {
        assert_eq!(self.channels, rhs.channels);
        for (l, &r) in self.samples.iter_mut().zip(&rhs.samples) {
            *l *= r;
        }
    }
}

impl MulAssign for Stream {
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(self.channels, rhs.channels);
        for (l, r) in self.samples.iter_mut().zip(rhs.samples) {
            *l *= r;
        }
    }
}
impl Mul<&Stream> for &Stream {
    type Output = Stream;

    fn mul(self, rhs: &Stream) -> Self::Output {
        let mut output = self.clone();
        output *= rhs;
        output
    }
}

impl Mul<Stream> for &Stream {
    type Output = Stream;

    fn mul(self, rhs: Stream) -> Self::Output {
        let mut output = self.clone();
        output *= rhs;
        output
    }
}
impl Mul<&Stream> for Stream {
    type Output = Stream;

    fn mul(mut self, rhs: &Stream) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul for Stream {
    type Output = Stream;

    fn mul(mut self, rhs: Stream) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f64> for Stream {
    fn mul_assign(&mut self, rhs: f64) {
        let rhs = rhs;
        for l in self.samples.iter_mut() {
            *l *= rhs;
        }
    }
}

impl Mul<f64> for &Stream {
    type Output = Stream;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut output = self.clone();
        output *= rhs;
        output
    }
}

impl Mul<f64> for Stream {
    type Output = Stream;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<Stream> for f64 {
    type Output = Stream;

    fn mul(self, rhs: Stream) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Stream> for f64 {
    type Output = Stream;

    fn mul(self, rhs: &Stream) -> Self::Output {
        rhs * self
    }
}
