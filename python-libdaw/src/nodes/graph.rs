use crate::{Node, Result};
use libdaw::nodes::graph::Index as DawIndex;
use nohash_hasher::IntMap;
use pyo3::{
    pyclass,
    pyclass::CompareOp,
    pymethods,
    types::{PyModule, PyModuleMethods as _},
    Bound, Py, PyClassInitializer, PyResult, PyTraverseError, PyVisit,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash as _, Hasher as _},
    sync::Arc,
};

#[pyclass(module = "libdaw.nodes")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Index(DawIndex);

#[pymethods]
impl Index {
    pub fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn __richcmp__(&self, other: Bound<'_, Self>, op: CompareOp) -> bool {
        op.matches(self.0.cmp(&other.borrow().0))
    }
}

impl From<DawIndex> for Index {
    fn from(value: DawIndex) -> Self {
        Self(value)
    }
}
impl From<Index> for DawIndex {
    fn from(value: Index) -> Self {
        value.0
    }
}

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Graph {
    inner: Arc<::libdaw::nodes::Graph>,
    nodes: IntMap<DawIndex, Py<Node>>,
}

#[pymethods]
impl Graph {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        let inner = Arc::new(::libdaw::nodes::Graph::default());
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self {
            inner,
            nodes: Default::default(),
        })
    }

    pub fn add(&mut self, node: Bound<'_, Node>) -> Index {
        let index = self.inner.add(node.borrow().0.clone());
        self.nodes.insert(index, node.unbind());
        index.into()
    }

    pub fn remove(&mut self, index: Index) -> Result<Option<Py<Node>>> {
        self.inner.remove(index.0)?;
        Ok(self.nodes.remove(&index.into()))
    }

    /// Connect the given output of the source to the destination.  The same
    /// output may be attached  multiple times. `None` will attach all outputs.
    pub fn connect(&self, source: Index, destination: Index, stream: Option<usize>) -> Result<()> {
        self.inner
            .connect(source.0, destination.0, stream)
            .map_err(Into::into)
    }

    /// Disconnect the last-added matching connection, returning a boolean
    /// indicating if anything was disconnected.
    pub fn disconnect(
        &self,
        source: Index,
        destination: Index,
        stream: Option<usize>,
    ) -> Result<()> {
        self.inner
            .disconnect(source.0, destination.0, stream)
            .map_err(Into::into)
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn input(&self, source: Index, stream: Option<usize>) -> Result<()> {
        self.inner.input(source.0, stream).map_err(Into::into)
    }

    /// Disconnect the last-added matching connection from the destination.0,
    /// returning a boolean indicating if anything was disconnected.
    pub fn remove_input(&self, source: Index, stream: Option<usize>) -> Result<()> {
        self.inner
            .remove_input(source.0, stream)
            .map_err(Into::into)
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn output(&self, source: Index, stream: Option<usize>) -> Result<()> {
        self.inner.output(source.0, stream).map_err(Into::into)
    }

    /// Disconnect the last-added matching connection from the destination.0,
    /// returning a boolean indicating if anything was disconnected.
    pub fn remove_output(&self, source: Index, stream: Option<usize>) -> Result<()> {
        self.inner
            .remove_output(source.0, stream)
            .map_err(Into::into)
    }
    fn __traverse__(&self, visit: PyVisit<'_>) -> std::result::Result<(), PyTraverseError> {
        for node in self.nodes.values() {
            visit.call(node)?
        }
        Ok(())
    }

    pub fn __clear__(&mut self) {
        for index in self.nodes.keys().copied() {
            self.inner
                .remove(index)
                .expect("illegal index")
                .expect("unfilled index");
        }
        self.nodes.clear();
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Index>()?;
    Ok(())
}
