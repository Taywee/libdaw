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

pub trait Output: Node {}
pub trait Input: Node {
    fn sample(&self) -> SmallVec<[f64; 2]>;
}

#[derive(Debug)]
enum GraphSlot {
    Input(Rc<RefCell<dyn Input>>),
    Output(Rc<RefCell<dyn Output>>),
}

impl GraphSlot {
    fn as_ptr(&self) -> *const () {
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
        self.as_ptr().hash(state);
    }
}

impl PartialEq for GraphSlot {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Eq for GraphSlot {}

#[derive(Debug)]
struct Edge {
    source: Weak<RefCell<dyn Input>>,
    destination: Weak<RefCell<dyn Output>>,
}

impl Hash for Edge {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        (self.source.as_ptr(), self.destination.as_ptr()).hash(state);
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.source.as_ptr(), self.destination.as_ptr())
            == (other.source.as_ptr(), other.destination.as_ptr())
    }
}

impl Eq for Edge {}

#[derive(Default, Debug)]
pub struct Graph {
    nodes: HashSet<GraphSlot>,
    edges: HashSet<Edge>,
}

impl Graph {
    pub fn connect(
        &mut self,
        source: Rc<RefCell<dyn Input>>,
        destination: Rc<RefCell<dyn Output>>,
    ) {
        self.edges.insert(Edge {
            source: Rc::downgrade(&source),
            destination: Rc::downgrade(&destination),
        });
        self.nodes.insert(GraphSlot::Input(source));
        self.nodes.insert(GraphSlot::Output(destination));
        todo!()
    }
}

#[derive(Default, Debug)]
pub struct SquareOscillator {
    frequency: f64,
    sample_number: u64,
    sample: f64,
}
