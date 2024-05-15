mod iter;

pub use iter::{IntoIter, Iter};

use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign},
};

#[derive(Debug, Clone, Default)]
pub struct Stream {
    samples: Vec<f64>,
}

impl Stream {
    pub fn new(channels: usize) -> Self {
        Self {
            samples: vec![0.0; channels],
        }
    }

    pub fn channels(&self) -> usize {
        self.samples.len()
    }

    pub fn resize(&mut self, new_len: usize, value: f64) {
        self.samples.resize(new_len, value)
    }

    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> f64,
    {
        self.samples.resize_with(new_len, f)
    }

    pub fn iter(&self) -> iter::Iter<'_> {
        iter::Iter(self.samples.iter())
    }
}

impl IntoIterator for Stream {
    type Item = f64;

    type IntoIter = iter::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIter(self.samples.into_iter())
    }
}
impl<'a> IntoIterator for &'a Stream {
    type Item = &'a f64;

    type IntoIter = iter::Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<Vec<f64>> for Stream {
    fn from(samples: Vec<f64>) -> Self {
        Self { samples }
    }
}
impl From<Stream> for Vec<f64> {
    fn from(value: Stream) -> Self {
        value.samples
    }
}

impl Deref for Stream {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        &self.samples
    }
}

impl DerefMut for Stream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.samples
    }
}

impl AddAssign<&Stream> for Stream {
    fn add_assign(&mut self, rhs: &Stream) {
        if self.len() < rhs.len() {
            self.resize(rhs.len(), 0.0);
        }
        for (l, &r) in self.samples.iter_mut().zip(&rhs.samples) {
            *l += r;
        }
    }
}

impl AddAssign for Stream {
    fn add_assign(&mut self, rhs: Self) {
        if self.len() < rhs.len() {
            self.resize(rhs.len(), 0.0);
        }
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
        if self.len() < rhs.len() {
            self.resize(rhs.len(), 0.0);
        }
        for (l, &r) in self.samples.iter_mut().zip(&rhs.samples) {
            *l *= r;
        }
    }
}

impl MulAssign for Stream {
    fn mul_assign(&mut self, rhs: Self) {
        if self.len() < rhs.len() {
            self.resize(rhs.len(), 0.0);
        }
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

impl Sum for Stream {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut output = Stream::new(0);
        for item in iter {
            output += item;
        }
        output
    }
}

impl<'a> Sum<&'a Stream> for Stream {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Stream>,
    {
        let mut output = Stream::new(0);
        for item in iter {
            output += item;
        }
        output
    }
}
impl Product for Stream {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut output = Stream::new(0);
        for item in iter {
            output *= item;
        }
        output
    }
}

impl<'a> Product<&'a Stream> for Stream {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Stream>,
    {
        let mut output = Stream::new(0);
        for item in iter {
            output *= item;
        }
        output
    }
}
