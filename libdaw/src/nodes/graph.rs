use crate::nodes::Add;
use crate::streams::Streams;
use crate::Node;
use smallvec::smallvec;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ptr::addr_eq;
use std::rc::Rc;

/// A strong node wrapper, allowing hashing and comparison on a pointer basis.
#[derive(Debug, Clone)]
struct Slot(Rc<RefCell<dyn Node>>);

impl Hash for Slot {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        Rc::as_ptr(&self.0).hash(state);
    }
}

impl PartialEq for Slot {
    fn eq(&self, other: &Self) -> bool {
        addr_eq(Rc::as_ptr(&self.0), Rc::as_ptr(&other.0))
    }
}

impl Eq for Slot {}

impl PartialOrd for Slot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (Rc::as_ptr(&self.0) as *const ()).partial_cmp(&(Rc::as_ptr(&other.0) as *const ()))
    }
}

impl Ord for Slot {
    fn cmp(&self, other: &Self) -> Ordering {
        (Rc::as_ptr(&self.0) as *const ()).cmp(&(Rc::as_ptr(&other.0) as *const ()))
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Input {
    output: Option<usize>,
    source: Slot,
}

#[derive(Debug, Default)]
struct Outputs {
    streams: Streams,
    count: usize,
}

type InputValues = HashMap<Slot, Outputs>;
type Inputs = HashMap<Slot, Vec<Input>>;

#[derive(Debug)]
pub struct Graph {
    /// Stored values output from an input node. Every input node has this from
    /// the beginning, even if they have not produced any streams yet.  In that
    /// case, they will all be empty streams.
    input_values: InputValues,

    /// Connects a node to a particular input.
    inputs: Inputs,

    /// An Add node that represents the actual sink.
    sink: Slot,

    sample_rate: u32,
}

impl Default for Graph {
    fn default() -> Self {
        let mut input_values: InputValues = Default::default();
        let sink = Slot(Rc::new(RefCell::new(Add::default())));
        input_values.entry(sink.clone()).or_default().count += 1;
        Self {
            input_values,
            sink,
            inputs: Default::default(),
            sample_rate: Default::default(),
        }
    }
}

impl Graph {
    /// Connect the given output of the source to the destination.  The same
    /// output may be attached  multiple times. `None` will attach all outputs.
    pub fn connect(
        &mut self,
        source: Rc<RefCell<dyn Node>>,
        destination: Rc<RefCell<dyn Node>>,
        output: Option<usize>,
    ) {
        self.input_values
            .entry(Slot(source.clone()))
            .or_default()
            .count += 1;

        self.inputs
            .entry(Slot(destination))
            .or_default()
            .push(Input {
                source: Slot(source),
                output,
            });
    }
    /// Disconnect the last-added matching connection, returning a boolean
    /// indicating if anything was disconnected.
    pub fn disconnect(
        &mut self,
        source: Rc<RefCell<dyn Node>>,
        destination: Rc<RefCell<dyn Node>>,
        output: Option<usize>,
    ) -> bool {
        if let Some(inputs) = self.inputs.get_mut(&Slot(destination)) {
            let source = Input {
                source: Slot(source),
                output,
            };
            if let Some((index, _)) = inputs
                .iter()
                .enumerate()
                .rev()
                .find(|(i, input)| **input == source)
            {
                inputs.remove(index);
                let Entry::Occupied(mut input_entry) = self.input_values.entry(source.source)
                else {
                    unreachable!()
                };

                input_entry.get_mut().count -= 1;
                if input_entry.get_mut().count == 0 {
                    input_entry.remove();
                }
            }
        }
        false
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached  multiple times. `None` will attach all
    /// outputs.
    pub fn sink(&mut self, source: Rc<RefCell<dyn Node>>, output: Option<usize>) {
        self.connect(source, self.sink.0.clone(), output);
    }

    /// Disconnect the last-added matching connection from the destination,
    /// returning a boolean indicating if anything was disconnected.
    pub fn unsink(&mut self, source: Rc<RefCell<dyn Node>>, output: Option<usize>) -> bool {
        self.disconnect(source, self.sink.0.clone(), output)
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn iter_nodes(&self) -> impl Iterator<Item = &Slot> {
        let mut seen = HashSet::with_capacity(self.input_values.len());
        self.inputs
            .iter()
            .flat_map(|(node, inputs)| {
                std::iter::once(node).chain(inputs.iter().map(|input| &input.source))
            })
            .chain(std::iter::once(&self.sink))
            .filter(move |node| seen.insert(*node))
    }

    fn walk_node(&self, node: &Slot, output: &mut Vec<Slot>, memo: &mut HashSet<Slot>) {
        if memo.insert(node.clone()) {
            let inputs = self.inputs.get(&node);
            output.push(node.clone());
            for input in inputs.into_iter().flatten() {
                self.walk_node(&input.source, output, memo);
            }
        }
    }

    /// Get the processing list, in order from sink to roots.
    fn get_process_list(&self) -> Vec<Slot> {
        let mut output = Vec::with_capacity(self.input_values.len());
        let mut memo = HashSet::with_capacity(self.input_values.len());
        self.walk_node(&self.sink, &mut output, &mut memo);
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
            node.0.borrow_mut().set_sample_rate(sample_rate);
        }
    }

    /// Process all inputs in reverse order, from roots down to the sink.
    /// Nodes not connected to any sinks are processed in an undefined order.
    /// All sinks are added together to turn this into a single output.
    fn process(&mut self, _inputs: Streams) -> Streams {
        // First process all process-needing nodes in reverse order.
        for node in self.get_process_list().into_iter().rev() {
            let inputs = self.inputs.get(&node);
            let mut input_streams = Streams::default();
            for input in inputs.into_iter().flatten() {
                let value_streams = self
                    .input_values
                    .get(&input.source)
                    .expect("process node not in input_values");
                if let Some(output) = input.output {
                    if let Some(channels) = value_streams.streams.0.get(output).cloned() {
                        input_streams.0.push(channels);
                    }
                } else {
                    input_streams
                        .0
                        .extend(value_streams.streams.0.iter().cloned());
                }
            }
            let output = node.0.borrow_mut().process(input_streams);
            self.input_values.get_mut(&node).unwrap().streams = output;
        }
        self.input_values.get(&self.sink).unwrap().streams.clone()
    }
}
