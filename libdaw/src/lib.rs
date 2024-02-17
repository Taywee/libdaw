pub mod streams;

use smallvec::smallvec;
use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Mul,
    ptr::addr_eq,
    rc::Rc,
    sync::{Arc, Mutex, Weak},
};
use streams::{Channels, Streams};

/// An audio node trait, allowing a sample_rate to be set and processing to
/// be performed.
pub trait Node: Debug {
    fn set_sample_rate(&mut self, sample_rate: u32);
    fn process(&mut self, inputs: Streams) -> Streams;
}

/// A strong node wrapper, allowing hashing and comparison on a pointer basis.
#[derive(Debug, Clone)]
struct StrongNode(Arc<Mutex<dyn Node + Send>>);

impl Hash for StrongNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        Arc::as_ptr(&self.0).hash(state);
    }
}

impl PartialEq for StrongNode {
    fn eq(&self, other: &Self) -> bool {
        addr_eq(Arc::as_ptr(&self.0), Arc::as_ptr(&other.0))
    }
}

impl Eq for StrongNode {}

impl PartialOrd for StrongNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (Arc::as_ptr(&self.0) as *const ()).partial_cmp(&(Arc::as_ptr(&other.0) as *const ()))
    }
}

impl Ord for StrongNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (Arc::as_ptr(&self.0) as *const ()).cmp(&(Arc::as_ptr(&other.0) as *const ()))
    }
}

#[derive(Debug, Clone)]
struct WeakNode(Weak<Mutex<dyn Node + Send>>);

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

    sample_rate: u32,
}

impl Graph {
    pub fn connect(
        &mut self,
        source: Arc<Mutex<dyn Node + Send>>,
        output: usize,
        destination: Arc<Mutex<dyn Node + Send>>,
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
    pub fn sink(&mut self, source: Arc<Mutex<dyn Node + Send>>, output: usize) {
        self.input_values
            .entry(StrongNode(source.clone()))
            .or_default();

        self.sinks.push(Input {
            source: StrongNode(source),
            output,
        });
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
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
    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
        for node in self.iter_nodes() {
            node.0
                .lock()
                .expect("Could not lock mutex")
                .set_sample_rate(sample_rate);
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
            let output = node
                .0
                .lock()
                .expect("Could not lock mutex")
                .process(input_streams);
            self.input_values.insert(node, output);
        }
        let mut output = streams::Channels::default();
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
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl ConstantValue {
    pub fn set_value(&mut self, value: f64) {
        self.0 = value;
    }
}

impl Node for ConstantValue {
    fn set_sample_rate(&mut self, _: u32) {}

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
    pub fn get_frequency(&self) -> f64 {
        self.frequency
    }
}

impl Default for SquareOscillator {
    fn default() -> Self {
        let mut node = SquareOscillator {
            frequency: 256.0,
            samples_since_switch: 0.0,
            sample: 1.0,
            samples_per_switch: 100000.0,
            sample_rate: 48000.0,
        };
        node.calculate_samples_per_switch();
        node
    }
}

impl Node for SquareOscillator {
    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate.into();
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
            sample_rate: 48000.0,
            delta: 0.01,
        };
        node.calculate_delta();
        node
    }
}

impl Node for SawtoothOscillator {
    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate.into();
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

#[derive(Debug, Default)]
pub struct Multiply;

impl Node for Multiply {
    fn set_sample_rate(&mut self, _: u32) {}

    fn process(&mut self, input: Streams) -> Streams {
        Streams(smallvec![input
            .0
            .into_iter()
            .reduce(Channels::mul)
            .unwrap_or_default()])
    }
}
