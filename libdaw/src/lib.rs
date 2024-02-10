use smallvec::SmallVec;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};

pub trait Node: Debug {
    fn update(&mut self, sample_rate: u64, inputs: &[SmallVec<[f64; 2]>]);
}

pub trait Output: Node {
    /// Add a node as upstream.  Client applications should use `connect` instead.
    fn add_upstream(&mut self, upstream: Weak<RefCell<dyn Input>>);
}
pub trait Input: Node {
    fn add_downstream(&mut self, downstream: Weak<RefCell<dyn Output>>);
    fn sample(&self) -> SmallVec<[f64; 2]>;
}

#[derive(Debug)]
enum GraphSlot {
    Input(Rc<RefCell<dyn Input>>),
    Output(Rc<RefCell<dyn Output>>),
}

impl GraphSlot {
    fn as_thin_pointer(&self) -> *const () {
        match self {
            GraphSlot::Input(node) => node.as_ptr() as *const (),
            GraphSlot::Output(node) => node.as_ptr() as *const (),
        }
    }
}

impl Hash for GraphSlot {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.as_thin_pointer().hash(state);
    }
}

impl PartialEq for GraphSlot {
    fn eq(&self, other: &Self) -> bool {
        self.as_thin_pointer() == other.as_thin_pointer()
    }
}

impl Eq for GraphSlot {}

#[derive(Default, Debug)]
pub struct Graph {
    nodes: HashSet<GraphSlot>,
}

impl Graph {
    pub fn connect(
        &mut self,
        source: Rc<RefCell<dyn Input>>,
        destination: Rc<RefCell<dyn Output>>,
    ) {
        source
            .borrow_mut()
            .add_downstream(Rc::downgrade(&destination));
        destination
            .borrow_mut()
            .add_upstream(Rc::downgrade(&source));
        self.nodes.insert(GraphSlot::Input(source));
        self.nodes.insert(GraphSlot::Output(destination));
    }
}

#[derive(Default, Debug)]
struct Sockets {
    upstream: Vec<Weak<RefCell<dyn Input>>>,
    downstream: Vec<Weak<RefCell<dyn Output>>>,
}

#[derive(Default, Debug)]
pub struct SquareOscillator {
    sockets: Sockets,
    frequency: f64,
    sample: u64,
}
