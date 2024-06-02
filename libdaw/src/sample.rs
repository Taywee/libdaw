mod iter;

pub use iter::{IntoIter, Iter};

use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign},
};

#[derive(Debug, Clone, Default)]
pub struct Sample {
    pub channels: Vec<f64>,
}

impl Sample {
    pub fn zeroed(len: usize) -> Self {
        Self {
            channels: vec![0.0; len],
        }
    }
    pub fn iter(&self) -> iter::Iter<'_> {
        iter::Iter(self.channels.iter())
    }
}

impl IntoIterator for Sample {
    type Item = f64;

    type IntoIter = iter::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIter(self.channels.into_iter())
    }
}
impl<'a> IntoIterator for &'a Sample {
    type Item = &'a f64;

    type IntoIter = iter::Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<Vec<f64>> for Sample {
    fn from(samples: Vec<f64>) -> Self {
        Self { channels: samples }
    }
}
impl From<Sample> for Vec<f64> {
    fn from(value: Sample) -> Self {
        value.channels
    }
}

impl Deref for Sample {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        &self.channels
    }
}

impl DerefMut for Sample {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.channels
    }
}

impl AddAssign<&Sample> for Sample {
    fn add_assign(&mut self, rhs: &Sample) {
        if self.len() < rhs.len() {
            self.channels.resize(rhs.len(), 0.0);
        }
        for (l, &r) in self.channels.iter_mut().zip(&rhs.channels) {
            *l += r;
        }
    }
}

impl AddAssign for Sample {
    fn add_assign(&mut self, rhs: Self) {
        if self.len() < rhs.len() {
            self.channels.resize(rhs.len(), 0.0);
        }
        for (l, r) in self.channels.iter_mut().zip(rhs.channels) {
            *l += r;
        }
    }
}
impl Add for &Sample {
    type Output = Sample;

    fn add(self, rhs: &Sample) -> Self::Output {
        let mut output = self.clone();
        output += rhs;
        output
    }
}

impl Add<Sample> for &Sample {
    type Output = Sample;

    fn add(self, rhs: Sample) -> Self::Output {
        let mut output = self.clone();
        output += rhs;
        output
    }
}
impl Add<&Sample> for Sample {
    type Output = Sample;

    fn add(mut self, rhs: &Sample) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add for Sample {
    type Output = Sample;

    fn add(mut self, rhs: Sample) -> Self::Output {
        self += rhs;
        self
    }
}

impl MulAssign<&Sample> for Sample {
    fn mul_assign(&mut self, rhs: &Sample) {
        if self.len() < rhs.len() {
            self.channels.resize(rhs.len(), 1.0);
        }
        for (l, &r) in self.channels.iter_mut().zip(&rhs.channels) {
            *l *= r;
        }
    }
}

impl MulAssign for Sample {
    fn mul_assign(&mut self, rhs: Self) {
        if self.len() < rhs.len() {
            self.channels.resize(rhs.len(), 1.0);
        }
        for (l, r) in self.channels.iter_mut().zip(rhs.channels) {
            *l *= r;
        }
    }
}
impl Mul<&Sample> for &Sample {
    type Output = Sample;

    fn mul(self, rhs: &Sample) -> Self::Output {
        let mut output = self.clone();
        output *= rhs;
        output
    }
}

impl Mul<Sample> for &Sample {
    type Output = Sample;

    fn mul(self, rhs: Sample) -> Self::Output {
        let mut output = self.clone();
        output *= rhs;
        output
    }
}
impl Mul<&Sample> for Sample {
    type Output = Sample;

    fn mul(mut self, rhs: &Sample) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul for Sample {
    type Output = Sample;

    fn mul(mut self, rhs: Sample) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f64> for Sample {
    fn mul_assign(&mut self, rhs: f64) {
        let rhs = rhs;
        for l in self.channels.iter_mut() {
            *l *= rhs;
        }
    }
}

impl Mul<f64> for &Sample {
    type Output = Sample;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut output = self.clone();
        output *= rhs;
        output
    }
}

impl Mul<f64> for Sample {
    type Output = Sample;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<Sample> for f64 {
    type Output = Sample;

    fn mul(self, rhs: Sample) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Sample> for f64 {
    type Output = Sample;

    fn mul(self, rhs: &Sample) -> Self::Output {
        rhs * self
    }
}

impl Sum for Sample {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut output = Sample::zeroed(0);
        for item in iter {
            output += item;
        }
        output
    }
}

impl<'a> Sum<&'a Sample> for Sample {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Sample>,
    {
        let mut output = Sample::zeroed(0);
        for item in iter {
            output += item;
        }
        output
    }
}
impl Product for Sample {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut output = Sample::zeroed(0);
        for item in iter {
            output *= item;
        }
        output
    }
}

impl<'a> Product<&'a Sample> for Sample {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Sample>,
    {
        let mut output = Sample::zeroed(0);
        for item in iter {
            output *= item;
        }
        output
    }
}
