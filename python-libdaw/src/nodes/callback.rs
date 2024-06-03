use crate::{time::Timestamp, Node};
use libdaw::{time::Timestamp as DawTimestamp, Node as DawNode};
use nohash_hasher::IntSet;
use pyo3::{
    pyclass, pymethods, types::PyAnyMethods as _, Bound, Py, PyAny, PyClassInitializer, PyResult,
    PyTraverseError, PyVisit, Python,
};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Slot {
    callable: Py<PyAny>,
    start: DawTimestamp,
    end: DawTimestamp,
    handle: usize,
}

impl PartialOrd for Slot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Slot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.start, self.end, self.handle).cmp(&(other.start, other.end, other.handle))
    }
}
impl PartialEq for Slot {
    fn eq(&self, other: &Self) -> bool {
        (self.start, self.end, self.handle) == (other.start, other.end, other.handle)
    }
}

impl Eq for Slot {}

#[derive(Debug, Default)]
struct Callbacks {
    slots: Vec<Slot>,
}

impl Callbacks {
    fn run(&mut self, now: DawTimestamp) -> PyResult<()> {
        if self.slots.is_empty() {
            return Ok(());
        }
        let removals: PyResult<_> = Python::with_gil(|py| {
            let py_now = Timestamp(now);
            let mut remove = IntSet::default();
            for slot in &self.slots {
                if slot.end <= now {
                    remove.insert(slot.handle);
                    continue;
                }
                if slot.start > now {
                    break;
                }
                let bound = slot.callable.bind(py);
                let result: Option<bool> = bound.call1((py_now,))?.extract()?;
                if result.unwrap_or(false) {
                    remove.insert(slot.handle);
                }
            }
            Ok(remove)
        });
        let removals = removals?;
        if !removals.is_empty() {
            self.slots.retain(|slot| !removals.contains(&slot.handle));
        }

        Ok(())
    }

    fn add(&mut self, callable: Py<PyAny>, start: Timestamp, end: Timestamp) {
        let handles: IntSet<_> = self.slots.iter().map(|slot| slot.handle).collect();
        let handle = (0usize..)
            .find(|handle| !handles.contains(handle))
            .expect("All integer handles used");
        let slot = Slot {
            callable,
            start: start.0,
            end: end.0,
            handle,
        };

        let index = self
            .slots
            .binary_search(&slot)
            .expect_err("Should never find an existing handle in callbacks");

        self.slots.insert(index, slot);
    }

    fn traverse(&self, visit: &PyVisit<'_>) -> std::result::Result<(), PyTraverseError> {
        for slot in &self.slots {
            visit.call(&slot.callable)?;
        }
        Ok(())
    }

    fn clear(&mut self) {
        self.slots.clear()
    }
}

#[derive(Debug)]
struct Inner {
    node: Arc<Mutex<dyn DawNode>>,
    pre: Callbacks,
    post: Callbacks,
    sample_rate: u32,
    sample: u64,
}

impl Inner {
    fn new(node: Arc<Mutex<dyn DawNode>>, sample_rate: u32) -> Self {
        Self {
            node,
            pre: Default::default(),
            post: Default::default(),
            sample_rate,
            sample: 0,
        }
    }
    fn traverse(&self, visit: &PyVisit<'_>) -> std::result::Result<(), PyTraverseError> {
        self.pre.traverse(visit)?;
        self.post.traverse(visit)?;
        Ok(())
    }

    fn clear(&mut self) {
        self.pre.clear();
        self.post.clear();
    }
}

impl DawNode for Inner {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [libdaw::Sample],
        outputs: &'c mut Vec<libdaw::Sample>,
    ) -> libdaw::Result<()> {
        let seconds = self.sample as f64 / self.sample_rate as f64;
        let timestamp = DawTimestamp::from_seconds(seconds).expect("Impossible");
        self.pre.run(timestamp)?;
        self.node
            .lock()
            .expect("poisoned")
            .process(inputs, outputs)?;
        self.post.run(timestamp)?;
        self.sample += 1;
        Ok(())
    }
}

#[pyclass(extends = Node, subclass, module = "libdaw.nodes")]
#[derive(Debug, Clone)]
pub struct Callback {
    inner: Arc<Mutex<Inner>>,
    node: Option<Py<Node>>,
}

#[pymethods]
impl Callback {
    #[new]
    #[pyo3(signature = (node, sample_rate = 48000))]
    pub fn new(node: Bound<'_, Node>, sample_rate: u32) -> PyClassInitializer<Self> {
        let inner = Arc::new(Mutex::new(Inner::new(node.borrow().0.clone(), sample_rate)));
        PyClassInitializer::from(Node(inner.clone())).add_subclass(Self {
            inner,
            node: Some(node.unbind()),
        })
    }
    #[pyo3(signature = (
        callable,
        start = Timestamp::MIN,
        end = Timestamp::MAX,
        post = false,
    ))]
    fn add(&self, callable: Py<PyAny>, start: Timestamp, end: Timestamp, post: bool) {
        let mut inner = self.inner.lock().expect("poisoned");
        let callbacks = if post {
            &mut inner.post
        } else {
            &mut inner.pre
        };

        callbacks.add(callable, start, end);
    }
    #[getter]
    fn get_node(&self) -> Py<Node> {
        self.node.clone().expect("cleared").clone()
    }
    #[setter]
    fn set_node(&mut self, node: Py<Node>) {
        self.node = Some(node);
    }
    fn __traverse__(&self, visit: PyVisit<'_>) -> std::result::Result<(), PyTraverseError> {
        if let Some(node) = &self.node {
            visit.call(node)?;
        }
        self.inner.lock().expect("poisoned").traverse(&visit)?;
        Ok(())
    }

    fn __clear__(&mut self) {
        self.node = None;
        self.inner.lock().expect("poisoned").clear();
    }
}
