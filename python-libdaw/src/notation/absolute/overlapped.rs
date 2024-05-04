use crate::{
    metronome::{Beat, MaybeMetronome},
    nodes::instrument::Tone,
    notation::absolute::Section,
    pitch::MaybePitchStandard,
    Result,
};
use libdaw::metronome::Beat as DawBeat;
use pyo3::{exceptions::PyIndexError, pyclass, pymethods, Bound, PyResult};
use std::{
    ops::Deref as _,
    sync::{Arc, Mutex},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Overlapped(pub Arc<Mutex<libdaw::notation::absolute::Overlapped>>);

impl Overlapped {
    fn resolve_index(&self, index: isize) -> PyResult<isize> {
        let len = isize::try_from(self.__len__())
            .map_err(|error| PyIndexError::new_err(error.to_string()))?;
        Ok(if index < 0 { len + index } else { index })
    }
}

#[pymethods]
impl Overlapped {
    #[new]
    pub fn new(sections: Vec<Section>) -> Self {
        Self(Arc::new(Mutex::new(
            libdaw::notation::absolute::Overlapped(
                sections.into_iter().map(|section| section.0).collect(),
            ),
        )))
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
        let metronome = MaybeMetronome::from(metronome);
        let pitch_standard = MaybePitchStandard::from(pitch_standard);
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

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0.lock().expect("poisoned"))
    }

    pub fn __len__(&self) -> usize {
        self.0.lock().expect("poisoned").0.len()
    }

    pub fn __getitem__(&self, index: isize) -> PyResult<Section> {
        usize::try_from(self.resolve_index(index)?)
            .ok()
            .and_then(|index| {
                self.0
                    .lock()
                    .expect("poisoned")
                    .0
                    .get(index)
                    .cloned()
                    .map(Section)
            })
            .ok_or_else(|| PyIndexError::new_err("Index out of range"))
    }
    pub fn __setitem__(&mut self, index: isize, value: Section) -> PyResult<()> {
        let mut lock = self.0.lock().expect("poisoned");
        let slot = usize::try_from(self.resolve_index(index)?)
            .ok()
            .and_then(|index| lock.0.get_mut(index))
            .ok_or_else(|| PyIndexError::new_err("Index out of range"))?;
        *slot = value.0;
        Ok(())
    }
    pub fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        self.pop(Some(index)).map(|_| ())
    }

    pub fn __iter__(&self) -> OverlappedIterator {
        OverlappedIterator(self.0.lock().expect("poisoned").0.clone().into_iter())
    }

    pub fn append(&mut self, value: Section) -> PyResult<()> {
        self.0.lock().expect("poisoned").0.push(value.0);
        Ok(())
    }

    pub fn insert(&mut self, index: isize, value: Section) -> PyResult<()> {
        let index = usize::try_from(self.resolve_index(index)?).unwrap_or(0);
        if index >= self.0.lock().expect("poisoned").0.len() {
            self.0.lock().expect("poisoned").0.push(value.0);
        } else {
            self.0.lock().expect("poisoned").0.insert(index, value.0);
        }
        Ok(())
    }

    pub fn pop(&mut self, index: Option<isize>) -> PyResult<Section> {
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

        Ok(Section(self.0.lock().expect("poisoned").0.remove(index)))
    }
}

#[derive(Debug, Clone)]
#[pyclass(sequence, module = "libdaw.notation.absolute")]
pub struct OverlappedIterator(
    pub std::vec::IntoIter<Arc<Mutex<libdaw::notation::absolute::Section>>>,
);

#[pymethods]
impl OverlappedIterator {
    pub fn __iter__(self_: Bound<'_, Self>) -> Bound<'_, Self> {
        self_
    }
    pub fn __repr__(&self) -> String {
        format!("OverlappedIterator<{:?}>", self.0)
    }
    pub fn __next__(&mut self) -> Option<Section> {
        self.0.next().map(Section)
    }
}
