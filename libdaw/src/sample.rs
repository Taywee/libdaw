mod iter;

pub use iter::{IntoIter, Iter, IterMut};

use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
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
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.channels.iter())
    }
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut(self.channels.iter_mut())
    }
}

impl IntoIterator for Sample {
    type Item = f64;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.channels.into_iter())
    }
}
impl<'a> IntoIterator for &'a Sample {
    type Item = &'a f64;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a> IntoIterator for &'a mut Sample {
    type Item = &'a mut f64;

    type IntoIter = IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl FromIterator<f64> for Sample {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = f64>,
    {
        Self {
            channels: iter.into_iter().collect(),
        }
    }
}

/// Expected to always be a single-channel sample.
impl From<f64> for Sample {
    fn from(sample: f64) -> Self {
        let mut channels = Vec::new();
        channels.reserve_exact(1);
        channels.push(sample);
        Self { channels }
    }
}
impl From<Vec<f64>> for Sample {
    fn from(sample: Vec<f64>) -> Self {
        Self { channels: sample }
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

impl SubAssign<&Sample> for Sample {
    fn sub_assign(&mut self, rhs: &Sample) {
        if self.len() < rhs.len() {
            self.channels.resize(rhs.len(), 0.0);
        }
        for (l, &r) in self.channels.iter_mut().zip(&rhs.channels) {
            *l -= r;
        }
    }
}

impl SubAssign for Sample {
    fn sub_assign(&mut self, rhs: Self) {
        if self.len() < rhs.len() {
            self.channels.resize(rhs.len(), 0.0);
        }
        for (l, r) in self.channels.iter_mut().zip(rhs.channels) {
            *l -= r;
        }
    }
}
impl Sub for &Sample {
    type Output = Sample;

    fn sub(self, rhs: &Sample) -> Self::Output {
        let mut output = self.clone();
        output -= rhs;
        output
    }
}

impl Sub<Sample> for &Sample {
    type Output = Sample;

    fn sub(self, rhs: Sample) -> Self::Output {
        let mut output = self.clone();
        output -= rhs;
        output
    }
}
impl Sub<&Sample> for Sample {
    type Output = Sample;

    fn sub(mut self, rhs: &Sample) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Sub for Sample {
    type Output = Sample;

    fn sub(mut self, rhs: Sample) -> Self::Output {
        self -= rhs;
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
impl DivAssign<f64> for Sample {
    fn div_assign(&mut self, rhs: f64) {
        let rhs = rhs;
        for l in self.channels.iter_mut() {
            *l /= rhs;
        }
    }
}

impl Div<f64> for &Sample {
    type Output = Sample;

    fn div(self, rhs: f64) -> Self::Output {
        let mut output = self.clone();
        output /= rhs;
        output
    }
}

impl Div<f64> for Sample {
    type Output = Sample;

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl Div<Sample> for f64 {
    type Output = Sample;

    fn div(self, rhs: Sample) -> Self::Output {
        rhs / self
    }
}

impl Div<&Sample> for f64 {
    type Output = Sample;

    fn div(self, rhs: &Sample) -> Self::Output {
        rhs / self
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
