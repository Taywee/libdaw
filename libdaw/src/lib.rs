use smallvec::{smallvec, SmallVec};
use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};

pub trait Node: Debug {
    fn reset(&mut self);
    fn set_sample_rate(&mut self, sample_rate: f64);
    fn update(&mut self, inputs: &[&[f64]]);
    fn sample(&self) -> &[&[f64]];
}

/// A strong node wrapper, allowing hashing and comparison on a pointer basis.
#[derive(Debug)]
struct StrongNode(Rc<RefCell<dyn Node>>);

impl Hash for StrongNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        Rc::as_ptr(&self.0).hash(state);
    }
}

impl PartialEq for StrongNode {
    fn eq(&self, other: &Self) -> bool {
        Rc::as_ptr(&self.0) == Rc::as_ptr(&other.0)
    }
}

impl Eq for StrongNode {}

impl PartialOrd for StrongNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Rc::as_ptr(&self.0).partial_cmp(&Rc::as_ptr(&other.0))
    }
}

impl Ord for StrongNode {
    fn cmp(&self, other: &Self) -> Ordering {
        Rc::as_ptr(&self.0).cmp(&Rc::as_ptr(&other.0))
    }
}

#[derive(Debug)]
struct WeakNode(Weak<RefCell<dyn Node>>);

impl Hash for WeakNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        Weak::as_ptr(&self.0).hash(state);
    }
}

impl PartialEq for WeakNode {
    fn eq(&self, other: &Self) -> bool {
        Weak::as_ptr(&self.0) == Weak::as_ptr(&other.0)
    }
}

impl Eq for WeakNode {}
impl PartialOrd for WeakNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Weak::as_ptr(&self.0).partial_cmp(&Weak::as_ptr(&other.0))
    }
}

impl Ord for WeakNode {
    fn cmp(&self, other: &Self) -> Ordering {
        Weak::as_ptr(&self.0).cmp(&Weak::as_ptr(&other.0))
    }
}

#[derive(Default, Debug)]
pub struct Graph {
    nodes: HashSet<StrongNode>,
    // Connect a node to its inputs.
    inputs: HashMap<StrongNode, Vec<StrongNode>>,
    // Connect a node to its outputs
    outputs: HashMap<StrongNode, Vec<StrongNode>>,
}

impl Graph {
    pub fn connect(&mut self, source: Rc<RefCell<dyn Node>>, destination: Rc<RefCell<dyn Node>>) {
        self.edges.insert(Edge {
            source: WeakNode(Rc::downgrade(&source)),
            destination: WeakNode(Rc::downgrade(&destination)),
        });
        self.nodes.insert(StrongNode(source));
        self.nodes.insert(StrongNode(destination));
    }
}

#[derive(Debug, Default)]
pub struct SquareOscillator {
    frequency: f64,
    samples_per_switch: f64,
    samples_since_switch: f64,
    sample: f64,
}

impl Default for SquareOscillator {
    fn default() -> Self {
        SquareOscillator {
            frequency: 256.0,
            samples_since_switch: 0.0,
            sample: 1.0,
            samples_per_switch: 100000.0,
        }
    }
}

impl Node for SquareOscillator {
    fn reset(&mut self) {
        self.frequency = 256.0;
        self.samples_since_switch = 0.0;
        self.sample = 1.0;
    }

    fn set_sample_rate(&mut self, sample_rate: f64) {
        let switches_per_second = self.frequency * 2.0;
        let samples_per_second: f64 = sample_rate.into();
        self.samples_per_switch = samples_per_second / switches_per_second;
    }

    fn update(&mut self, _inputs: &[SmallVec<[f64; 2]>]) {
        while self.samples_since_switch >= self.samples_per_switch {
            self.samples_since_switch -= self.samples_per_switch;
            self.sample *= -1.0;
        }
        self.samples_since_switch += 1.0;
    }

    fn sample(&self) -> SmallVec<[f64; 2]> {
        smallvec![self.sample]
    }
}
