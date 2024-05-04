use crate::{
    metronome::{Beat, MaybeMetronome},
    nodes::instrument::Tone,
    notation::absolute::Item,
    pitch::MaybePitchStandard,
    Result,
};
use libdaw::metronome::Beat as DawBeat;
use pyo3::{exceptions::PyIndexError, pyclass, pymethods, Bound, PyResult};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Section(pub Arc<Mutex<libdaw::notation::absolute::Section>>);

impl Section {
    fn resolve_index(&self, index: isize) -> PyResult<isize> {
        let len = isize::try_from(self.__len__())
            .map_err(|error| PyIndexError::new_err(error.to_string()))?;
        Ok(if index < 0 { len + index } else { index })
    }
}

#[pymethods]
impl Section {
    #[new]
    pub fn new() -> Self {
        Self(Default::default())
    }

    #[staticmethod]
    pub fn parse(source: String) -> Result<Self> {
        Ok(Self(Arc::new(Mutex::new(source.parse()?))))
    }

    #[pyo3(
        signature = (
            *,
            offset=Beat(DawBeat::ZERO),
            metronome=MaybeMetronome::default(),
            pitch_standard=MaybePitchStandard::default(),
            previous_length=Beat(DawBeat::ONE),
        )
    )]
    pub fn resolve(
        &self,
        offset: Beat,
        metronome: MaybeMetronome,
        pitch_standard: MaybePitchStandard,
        previous_length: Beat,
    ) -> Vec<Tone> {
        self.0
            .lock()
            .expect("poisoned")
            .resolve(
                offset.0,
                &metronome,
                pitch_standard.deref(),
                previous_length.0,
            )
            .map(Tone)
            .collect()
    }

    pub fn length(&self, previous_length: Beat) -> Beat {
        Beat(self.0.lock().expect("poisoned").length(previous_length.0))
    }

    pub fn duration(&self, previous_length: Beat) -> Beat {
        Beat(self.0.lock().expect("poisoned").duration(previous_length.0))
    }

    pub fn __len__(&self) -> usize {
        self.0.lock().expect("poisoned").0.len()
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0.lock().expect("poisoned"))
    }

    pub fn __getitem__(&self, index: isize) -> PyResult<Item> {
        usize::try_from(self.resolve_index(index)?)
            .ok()
            .and_then(|index| self.0.lock().expect("poisoned").0.get(index).cloned())
            .map(Into::into)
            .ok_or_else(|| PyIndexError::new_err("Index out of range"))
    }

    pub fn __setitem__(&mut self, index: isize, value: Item) -> PyResult<()> {
        let mut lock = self.0.lock().expect("poisoned");
        let slot = usize::try_from(self.resolve_index(index)?)
            .ok()
            .and_then(|index| lock.0.get_mut(index))
            .ok_or_else(|| PyIndexError::new_err("Index out of range"))?;
        *slot = value.into();
        Ok(())
    }
    pub fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        self.pop(Some(index)).map(|_| ())
    }

    pub fn __iter__(&self) -> SectionIterator {
        SectionIterator(self.0.lock().expect("poisoned").0.clone().into_iter())
    }

    pub fn append(&mut self, value: Item) -> PyResult<()> {
        self.0.lock().expect("poisoned").0.push(value.into());
        Ok(())
    }

    pub fn insert(&mut self, index: isize, value: Item) -> PyResult<()> {
        let index = usize::try_from(self.resolve_index(index)?).unwrap_or(0);
        if index >= self.0.lock().expect("poisoned").0.len() {
            self.0.lock().expect("poisoned").0.push(value.into());
        } else {
            self.0
                .lock()
                .expect("poisoned")
                .0
                .insert(index, value.into());
        }
        Ok(())
    }

    pub fn pop(&mut self, index: Option<isize>) -> PyResult<Item> {
        let len = self.0.lock().expect("poisoned").0.len();
        if len == 0 {
            return Err(PyIndexError::new_err("Pop from empty"));
        }
        let index = match index {
            Some(index) => usize::try_from(self.resolve_index(index)?)
                .ok()
                .filter(move |index| *index < len)
                .ok_or_else(|| PyIndexError::new_err("Index out of range"))?,
            None => len - 1,
        };

        Ok(self.0.lock().expect("poisoned").0.remove(index).into())
    }
}

#[derive(Debug, Clone)]
#[pyclass(sequence, module = "libdaw.notation.absolute")]
pub struct SectionIterator(pub std::vec::IntoIter<libdaw::notation::absolute::Item>);

#[pymethods]
impl SectionIterator {
    pub fn __iter__(self_: Bound<'_, Self>) -> Bound<'_, Self> {
        self_
    }
    pub fn __repr__(&self) -> String {
        format!("SectionIterator<{:?}>", self.0)
    }
    pub fn __next__(&mut self) -> Option<Item> {
        self.0.next().map(Into::into)
    }
}
