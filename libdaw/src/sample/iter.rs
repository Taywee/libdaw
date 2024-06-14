use std::iter::FusedIterator;

#[derive(Debug, Clone, Default)]
pub struct IntoIter(pub(super) std::vec::IntoIter<f64>);

impl AsRef<[f64]> for IntoIter {
    fn as_ref(&self) -> &[f64] {
        self.0.as_ref()
    }
}

impl Iterator for IntoIter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for IntoIter {}

#[derive(Debug, Clone, Default)]
pub struct Iter<'a>(pub(super) std::slice::Iter<'a, f64>);

impl AsRef<[f64]> for Iter<'_> {
    fn as_ref(&self) -> &[f64] {
        self.0.as_ref()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for Iter<'_> {}

#[derive(Debug, Default)]
pub struct IterMut<'a>(pub(super) std::slice::IterMut<'a, f64>);

impl AsRef<[f64]> for IterMut<'_> {
    fn as_ref(&self) -> &[f64] {
        self.0.as_ref()
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }
}

impl DoubleEndedIterator for IterMut<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}

impl ExactSizeIterator for IterMut<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for IterMut<'_> {}
