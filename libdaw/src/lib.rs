use smallvec::{smallvec, SmallVec};
use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::AddAssign,
    ptr::addr_eq,
    rc::{Rc, Weak},
};

#[derive(Debug, Default, Clone)]
pub struct Channels(pub SmallVec<[f64; 2]>);

impl AddAssign<&Channels> for Channels {
    fn add_assign(&mut self, rhs: &Channels) {
        if self.0.len() < rhs.0.len() {
            self.0.resize(rhs.0.len(), 0.0);
        }
        for (l, &r) in self.0.iter_mut().zip(&rhs.0) {
            *l += r;
        }
    }
}
impl AddAssign for Channels {
    fn add_assign(&mut self, rhs: Self) {
        if self.0.len() < rhs.0.len() {
            self.0.resize(rhs.0.len(), 0.0);
        }
        for (l, r) in self.0.iter_mut().zip(rhs.0) {
            *l += r;
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Streams(pub SmallVec<[Channels; 1]>);

/// An audio node trait, allowing a sample_rate to be set and processing to
/// be performed.
pub trait Node: Debug {
    fn set_sample_rate(&mut self, sample_rate: f64);
    fn process(&mut self, inputs: Streams) -> Streams;
}

/// A strong node wrapper, allowing hashing and comparison on a pointer basis.
#[derive(Debug, Clone)]
struct StrongNode(Rc<RefCell<dyn Node>>);

impl StrongNode {
    pub fn downgrade(this: &Self) -> WeakNode {
        WeakNode(Rc::downgrade(&this.0))
    }
}

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
        addr_eq(Rc::as_ptr(&self.0), Rc::as_ptr(&other.0))
    }
}

impl Eq for StrongNode {}

impl PartialOrd for StrongNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (Rc::as_ptr(&self.0) as *const ()).partial_cmp(&(Rc::as_ptr(&other.0) as *const ()))
    }
}

impl Ord for StrongNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (Rc::as_ptr(&self.0) as *const ()).cmp(&(Rc::as_ptr(&other.0) as *const ()))
    }
}

#[derive(Debug, Clone)]
struct WeakNode(Weak<RefCell<dyn Node>>);

impl WeakNode {
    pub fn upgrade(&self) -> Option<StrongNode> {
        self.0.upgrade().map(StrongNode)
    }
}

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
        addr_eq(Weak::as_ptr(&self.0), Weak::as_ptr(&other.0))
    }
}

impl Eq for WeakNode {}
impl PartialOrd for WeakNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (Weak::as_ptr(&self.0) as *const ()).partial_cmp(&(Weak::as_ptr(&other.0) as *const ()))
    }
}

impl Ord for WeakNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (Weak::as_ptr(&self.0) as *const ()).cmp(&(Weak::as_ptr(&other.0) as *const ()))
    }
}

#[derive(Debug)]
struct Input {
    output: usize,
    source: StrongNode,
}

#[derive(Default, Debug)]
pub struct Graph {
    /// Stored values output from an input node.
    input_values: HashMap<StrongNode, Streams>,

    /// Connects a node to a particular input.
    inputs: HashMap<StrongNode, Vec<Input>>,

    /// Sinks that go directly to the outgoing sound.
    /// All outputs are added together channel-wise.
    sinks: Vec<Input>,
}

impl Graph {
    pub fn connect(
        &mut self,
        source: Rc<RefCell<dyn Node>>,
        output: usize,
        destination: Rc<RefCell<dyn Node>>,
    ) {
        self.input_values
            .entry(StrongNode(source.clone()))
            .or_default();
        self.inputs
            .entry(StrongNode(destination))
            .or_default()
            .push(Input {
                source: StrongNode(source),
                output,
            });
    }
    pub fn sink(&mut self, source: Rc<RefCell<dyn Node>>, output: usize) {
        self.input_values
            .entry(StrongNode(source.clone()))
            .or_default();

        self.sinks.push(Input {
            source: StrongNode(source),
            output,
        });
    }

    fn iter_nodes(&self) -> impl Iterator<Item = &StrongNode> {
        let mut seen = HashSet::with_capacity(self.input_values.len());
        self.inputs
            .iter()
            .flat_map(|(node, inputs)| {
                std::iter::once(node).chain(inputs.iter().map(|input| &input.source))
            })
            .chain(self.sinks.iter().map(|input| &input.source))
            .filter(move |node| seen.insert(*node))
    }

    fn walk_node(
        &self,
        node: &StrongNode,
        output: &mut Vec<StrongNode>,
        memo: &mut HashSet<StrongNode>,
    ) {
        if memo.insert(node.clone()) {
            let inputs = self.inputs.get(&node);
            output.push(node.clone());
            for input in inputs.into_iter().flatten() {
                self.walk_node(&input.source, output, memo);
            }
        }
    }

    /// Get the processing list, in reverse order to be processed.
    fn get_process_list(&self) -> Vec<StrongNode> {
        let mut output = Vec::with_capacity(self.input_values.len());
        let mut memo = HashSet::with_capacity(self.input_values.len());
        for sink in &self.sinks {
            self.walk_node(&sink.source, &mut output, &mut memo);
        }
        for input in self.input_values.keys() {
            self.walk_node(input, &mut output, &mut memo);
        }
        output
    }
}

impl Node for Graph {
    fn set_sample_rate(&mut self, sample_rate: f64) {
        for node in self.iter_nodes() {
            node.0.borrow_mut().set_sample_rate(sample_rate);
        }
    }

    /// Process all inputs in reverse order, from roots down to sinks.
    /// Nodes not connected to any sinks are processed in an unpredictable, but
    /// consistent, order.
    /// All sinks are added together to turn this into a single output.
    fn process(&mut self, _inputs: Streams) -> Streams {
        // First process all process-needing nodes in reverse order.
        for node in self.get_process_list().into_iter().rev() {
            let inputs = self.inputs.get(&node);
            let mut input_streams = Streams::default();
            for input in inputs.into_iter().flatten() {
                let value_streams = self.input_values.get(&input.source);
                let channels = value_streams
                    .map(|streams| streams.0.get(input.output).cloned())
                    .flatten()
                    .unwrap_or_default();
                input_streams.0.push(channels);
            }
            let output = node.0.borrow_mut().process(input_streams);
            self.input_values.insert(node, output);
        }
        let mut output = Channels::default();
        for sink in &self.sinks {
            let value_streams = self.input_values.get(&sink.source);
            let channels = value_streams
                .map(|streams| streams.0.get(sink.output).cloned())
                .flatten()
                .unwrap_or_default();
            output += channels;
        }
        Streams(smallvec![output])
    }
}

#[derive(Debug, Default)]
pub struct ConstantValue(f64);

impl ConstantValue {
    fn set_value(&mut self, value: f64) {
        self.0 = value;
    }
}

impl Node for ConstantValue {
    fn set_sample_rate(&mut self, _: f64) {}

    fn process(&mut self, _inputs: Streams) -> Streams {
        Streams(smallvec![Channels(smallvec![self.0])])
    }
}

#[derive(Debug)]
pub struct SquareOscillator {
    frequency: f64,
    samples_per_switch: f64,
    samples_since_switch: f64,
    sample_rate: f64,
    sample: f64,
}

impl SquareOscillator {
    fn calculate_samples_per_switch(&mut self) {
        let switches_per_second = self.frequency * 2.0;
        self.samples_per_switch = self.sample_rate / switches_per_second;
    }
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
        self.calculate_samples_per_switch();
    }
}

impl Default for SquareOscillator {
    fn default() -> Self {
        let mut node = SquareOscillator {
            frequency: 256.0,
            samples_since_switch: 0.0,
            sample: 1.0,
            samples_per_switch: 100000.0,
            sample_rate: 44100.0,
        };
        node.calculate_samples_per_switch();
        node
    }
}

impl Node for SquareOscillator {
    fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
        self.calculate_samples_per_switch();
    }

    fn process(&mut self, _: Streams) -> Streams {
        let output = Streams(smallvec![Channels(smallvec![self.sample])]);
        while self.samples_since_switch >= self.samples_per_switch {
            self.samples_since_switch -= self.samples_per_switch;
            self.sample *= -1.0;
        }
        self.samples_since_switch += 1.0;
        output
    }
}

#[derive(Debug)]
pub struct SawtoothOscillator {
    frequency: f64,
    sample_rate: f64,
    sample: f64,
    delta: f64,
}

impl SawtoothOscillator {
    fn calculate_delta(&mut self) {
        self.delta = self.frequency * 2.0 / self.sample_rate;
    }
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
        self.calculate_delta();
    }
}

impl Default for SawtoothOscillator {
    fn default() -> Self {
        let mut node = SawtoothOscillator {
            frequency: 256.0,
            sample: -1.0,
            sample_rate: 44100.0,
            delta: 0.01,
        };
        node.calculate_delta();
        node
    }
}

impl Node for SawtoothOscillator {
    fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
        self.calculate_delta();
    }

    fn process(&mut self, _: Streams) -> Streams {
        let output = Streams(smallvec![Channels(smallvec![self.sample])]);
        self.sample += self.delta;
        while self.sample > 1.0 {
            self.sample -= 1.0;
        }

        output
    }
}
