use super::Node;
use crate::streams::{Channels, Streams};
use smallvec::smallvec;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ptr::addr_eq;
use std::sync::{Arc, Mutex};

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
        destination: Arc<Mutex<dyn Node + Send>>,
        output: usize,
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
