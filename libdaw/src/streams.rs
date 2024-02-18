use smallvec::SmallVec;
use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign};

#[derive(Debug, Default, Clone)]
pub struct Channels(pub SmallVec<[f64; 2]>);
impl Deref for Channels {
    type Target = SmallVec<[f64; 2]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Channels {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AddAssign<&Channels> for Channels {
    fn add_assign(&mut self, rhs: &Channels) {
        if self.0.len() < rhs.0.len() {
            self.0.resize(rhs.0.len(), 0.0);
        }
        for (l, &r) in self.0.iter_mut().zip(&rhs.0) {
            *l += r;
        }
    }
}

impl AddAssign for Channels {
    fn add_assign(&mut self, rhs: Self) {
        if self.0.len() < rhs.0.len() {
            self.0.resize(rhs.0.len(), 0.0);
        }
        for (l, r) in self.0.iter_mut().zip(rhs.0) {
            *l += r;
        }
    }
}
impl Add for &Channels {
    type Output = Channels;

    fn add(self, rhs: &Channels) -> Self::Output {
        let mut output = self.clone();
        output += rhs;
        output
    }
}

impl Add<Channels> for &Channels {
    type Output = Channels;

    fn add(self, rhs: Channels) -> Self::Output {
        let mut output = self.clone();
        output += rhs;
        output
    }
}
impl Add<&Channels> for Channels {
    type Output = Channels;

    fn add(mut self, rhs: &Channels) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add for Channels {
    type Output = Channels;

    fn add(mut self, rhs: Channels) -> Self::Output {
        self += rhs;
        self
    }
}

impl MulAssign<&Channels> for Channels {
    fn mul_assign(&mut self, rhs: &Channels) {
        // Need both the same size so 0 multiplication still works.
        match self.0.len().cmp(&rhs.0.len()) {
            std::cmp::Ordering::Less => self.0.resize(rhs.0.len(), 0.0),
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => self.0[rhs.0.len()..].fill(0.0),
        }
        for (l, &r) in self.0.iter_mut().zip(&rhs.0) {
            *l *= r;
        }
    }
}

impl MulAssign for Channels {
    fn mul_assign(&mut self, rhs: Self) {
        // Need both the same size so 0 multiplication still works.
        match self.0.len().cmp(&rhs.0.len()) {
            std::cmp::Ordering::Less => self.0.resize(rhs.0.len(), 0.0),
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => self.0[rhs.0.len()..].fill(0.0),
        }
        for (l, r) in self.0.iter_mut().zip(rhs.0) {
            *l *= r;
        }
    }
}
impl Mul<&Channels> for &Channels {
    type Output = Channels;

    fn mul(self, rhs: &Channels) -> Self::Output {
        let mut output = self.clone();
        output *= rhs;
        output
    }
}

impl Mul<Channels> for &Channels {
    type Output = Channels;

    fn mul(self, rhs: Channels) -> Self::Output {
        let mut output = self.clone();
        output *= rhs;
        output
    }
}
impl Mul<&Channels> for Channels {
    type Output = Channels;

    fn mul(mut self, rhs: &Channels) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul for Channels {
    type Output = Channels;

    fn mul(mut self, rhs: Channels) -> Self::Output {
        self *= rhs;
        self
    }
}

#[derive(Debug, Default, Clone)]
pub struct Streams(pub SmallVec<[Channels; 1]>);

impl Deref for Streams {
    type Target = SmallVec<[Channels; 1]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Streams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
