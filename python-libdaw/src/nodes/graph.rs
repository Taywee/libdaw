use crate::{Node, Result};
use pyo3::{
    pyclass,
    pyclass::CompareOp,
    pymethods,
    types::{PyModule, PyModuleMethods as _},
    Bound, PyClassInitializer, PyResult,
};
use std::{
    hash::{DefaultHasher, Hash as _, Hasher as _},
    sync::Arc,
};

#[pyclass(module = "libdaw.nodes")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct GraphIndex(libdaw::nodes::graph::Index);

#[pymethods]
impl GraphIndex {
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

impl From<libdaw::nodes::graph::Index> for GraphIndex {
    fn from(value: libdaw::nodes::graph::Index) -> Self {
        Self(value)
    }
}
impl From<GraphIndex> for libdaw::nodes::graph::Index {
    fn from(value: GraphIndex) -> Self {
        value.0
    }
}

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Graph(pub Arc<::libdaw::nodes::Graph>);

#[pymethods]
impl Graph {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        let inner = Arc::new(::libdaw::nodes::Graph::default());
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self(inner))
    }

    pub fn add(&self, node: Bound<'_, Node>) -> GraphIndex {
        self.0.add(node.borrow().0.clone()).into()
    }

    pub fn remove(&self, index: GraphIndex) -> Result<Option<Node>> {
        Ok(self.0.remove(index.0)?.map(Node))
    }

    /// Connect the given output of the source to the destination.  The same
    /// output may be attached  multiple times. `None` will attach all outputs.
    pub fn connect(
        &self,
        source: GraphIndex,
        destination: GraphIndex,
        stream: Option<usize>,
    ) -> Result<()> {
        self.0
            .connect(source.0, destination.0, stream)
            .map_err(Into::into)
    }

    /// Disconnect the last-added matching connection, returning a boolean
    /// indicating if anything was disconnected.
    pub fn disconnect(
        &self,
        source: GraphIndex,
        destination: GraphIndex,
        stream: Option<usize>,
    ) -> Result<()> {
        self.0
            .disconnect(source.0, destination.0, stream)
            .map_err(Into::into)
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn input(&self, source: GraphIndex, stream: Option<usize>) -> Result<()> {
        self.0.input(source.0, stream).map_err(Into::into)
    }

    /// Disconnect the last-added matching connection from the destination.0,
    /// returning a boolean indicating if anything was disconnected.
    pub fn remove_input(&self, source: GraphIndex, stream: Option<usize>) -> Result<()> {
        self.0.remove_input(source.0, stream).map_err(Into::into)
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn output(&self, source: GraphIndex, stream: Option<usize>) -> Result<()> {
        self.0.output(source.0, stream).map_err(Into::into)
    }

    /// Disconnect the last-added matching connection from the destination.0,
    /// returning a boolean indicating if anything was disconnected.
    pub fn remove_output(&self, source: GraphIndex, stream: Option<usize>) -> Result<()> {
        self.0.remove_output(source.0, stream).map_err(Into::into)
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<GraphIndex>()?;
    Ok(())
}
