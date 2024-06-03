use crate::Node;
use libdaw::nodes::graph::Graph as Inner;
use pyo3::{pyclass, pymethods, Bound, PyClassInitializer};
use std::sync::{Arc, Mutex};

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Graph {
    inner: Arc<Mutex<Inner>>,
}

#[pymethods]
impl Graph {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::default()));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self { inner })
    }

    pub fn remove(&mut self, node: Bound<'_, Node>) -> bool {
        let node = node.borrow().0.clone();
        self.inner.lock().expect("poisoned").remove(node)
    }

    /// Connect the given output of the source to the destination.  The same
    /// output may be attached  multiple times. `None` will attach all outputs.
    pub fn connect(
        &self,
        source: Bound<'_, Node>,
        destination: Bound<'_, Node>,
        stream: Option<usize>,
    ) {
        let source = source.borrow().0.clone();
        let destination = destination.borrow().0.clone();
        self.inner
            .lock()
            .expect("poisoned")
            .connect(source, destination, stream);
    }

    /// Disconnect the last-added matching connection, returning a boolean
    /// indicating if anything was disconnected.
    pub fn disconnect(
        &self,
        source: Bound<'_, Node>,
        destination: Bound<'_, Node>,
        stream: Option<usize>,
    ) -> bool {
        let source = source.borrow().0.clone();
        let destination = destination.borrow().0.clone();
        self.inner
            .lock()
            .expect("poisoned")
            .disconnect(source, destination, stream)
    }

    /// Connect the given output of the initial input to the destination.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn input(&self, destination: Bound<'_, Node>, stream: Option<usize>) {
        let destination = destination.borrow().0.clone();
        self.inner
            .lock()
            .expect("poisoned")
            .input(destination, stream);
    }

    /// Disconnect the last-added matching connection from the destination,
    /// returning a boolean indicating if anything was disconnected.
    pub fn remove_input(&self, destination: Bound<'_, Node>, stream: Option<usize>) -> bool {
        let destination = destination.borrow().0.clone();
        self.inner
            .lock()
            .expect("poisoned")
            .remove_input(destination, stream)
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn output(&self, source: Bound<'_, Node>, stream: Option<usize>) {
        let source = source.borrow().0.clone();
        self.inner.lock().expect("poisoned").output(source, stream);
    }

    /// Disconnect the last-added matching connection from the source, returning
    /// a boolean indicating if anything was disconnected.
    pub fn remove_output(&self, source: Bound<'_, Node>, stream: Option<usize>) -> bool {
        let source = source.borrow().0.clone();
        self.inner
            .lock()
            .expect("poisoned")
            .remove_output(source, stream)
    }
}
