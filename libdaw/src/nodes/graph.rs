use crate::nodes::Passthrough;
use crate::stream::Stream;
use crate::Node;
use nohash_hasher::{IntSet, IsEnabled};
use std::cell::RefCell;

use std::hash::Hash;
use std::rc::Rc;

/// A strong node wrapper, allowing hashing and comparison on a pointer basis.
type Strong = Rc<dyn Node>;

/// The node index.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Index(pub usize);

impl Hash for Index {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.0);
    }
}

impl IsEnabled for Index {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Input {
    source: Index,
    stream: Option<usize>,
}

#[derive(Debug, Clone)]
struct Slot {
    node: Strong,
    output: RefCell<Vec<Stream>>,
    input_buffer: RefCell<Vec<Stream>>,
    inputs: Vec<Input>,
}

/// The processing order list, keeping the list in memory so that we only have
/// to rebuild it if the graph has changed.
#[derive(Debug, Default)]
struct ProcessList {
    list: Vec<Index>,
    memo: IntSet<Index>,
    reprocess: bool,
}

#[derive(Debug)]
struct InnerGraph {
    nodes: Vec<Option<Slot>>,
    empty_nodes: IntSet<Index>,
    set_nodes: IntSet<Index>,

    process_list: RefCell<ProcessList>,
}

impl Default for InnerGraph {
    fn default() -> Self {
        let mut graph = Self {
            nodes: Default::default(),
            empty_nodes: Default::default(),
            set_nodes: Default::default(),
            process_list: Default::default(),
        };
        // input
        graph.add(Rc::new(Passthrough::default()));
        // output
        graph.add(Rc::new(Passthrough::default()));
        graph
    }
}

impl InnerGraph {
    pub fn add(&mut self, node: Strong) -> Index {
        self.process_list.borrow_mut().reprocess = true;
        let slot = Some(Slot {
            node,
            output: Default::default(),
            input_buffer: Default::default(),
            inputs: Default::default(),
        });
        if let Some(index) = self.empty_nodes.iter().next().copied() {
            self.empty_nodes.remove(&index);
            self.set_nodes.insert(index);
            self.nodes[index.0] = slot;
            index
        } else {
            let index = Index(self.nodes.len());
            self.nodes.push(slot);
            self.set_nodes.insert(index);
            index
        }
    }

    pub fn remove(&mut self, index: Index) -> Option<Strong> {
        assert_ne!(index, Index(0), "Can not remove the input");
        assert_ne!(index, Index(1), "Can not remove the output");
        self.process_list.borrow_mut().reprocess = true;

        if let Some(slot) = self.nodes[index.0].take() {
            self.empty_nodes.insert(index);
            self.set_nodes.remove(&index);
            for set_index in self.set_nodes.iter().copied() {
                let slot = self.nodes[set_index.0]
                    .as_mut()
                    .expect("set slot not existing");
                slot.inputs.retain(|input| input.source != index);
            }
            Some(slot.node)
        } else {
            None
        }
    }

    /// Connect the given output of the source to the destination.  The same
    /// output may be attached  multiple times. `None` will attach all outputs.
    fn inner_connect(&mut self, source: Index, destination: Index, stream: Option<usize>) {
        assert!(
            self.nodes[source.0].is_some(),
            "source must be a valid index",
        );
        self.process_list.borrow_mut().reprocess = true;
        let destination = self.nodes[destination.0]
            .as_mut()
            .expect("destination must be a valid index");
        destination.inputs.push(Input { source, stream });
    }

    /// Connect the given output of the source to the destination.  The same
    /// output may be attached  multiple times. `None` will attach all outputs.
    pub fn connect(&mut self, source: Index, destination: Index, stream: Option<usize>) {
        assert_ne!(destination, Index(0), "use input instead");
        assert_ne!(source, Index(0), "can not use the input as a source");
        assert_ne!(destination, Index(1), "use output instead");
        assert_ne!(source, Index(1), "can not use the output as a source");
        self.inner_connect(source, destination, stream);
    }

    /// Disconnect the last-added matching connection, returning a boolean
    /// indicating if anything was disconnected.
    fn inner_disconnect(&mut self, source: Index, destination: Index, stream: Option<usize>) {
        let destination = self.nodes[destination.0]
            .as_mut()
            .expect("destination must be a valid index");
        self.process_list.borrow_mut().reprocess = true;
        let source = Input { source, stream };
        let (index, _) = destination
            .inputs
            .iter()
            .enumerate()
            .rev()
            .find(|(_, input)| **input == source)
            .expect("could not find connection");
        destination.inputs.remove(index);
    }

    /// Disconnect the last-added matching connection, returning a boolean
    /// indicating if anything was disconnected.
    pub fn disconnect(&mut self, source: Index, destination: Index, stream: Option<usize>) {
        assert_ne!(source, Index(0), "cannot connect or disconnect sink");
        assert_ne!(destination, Index(0), "use unsink instead");
        self.disconnect(source, destination, stream);
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn input(&mut self, source: Index, stream: Option<usize>) {
        assert_ne!(source, Index(0), "cannot input input");
        assert_ne!(source, Index(1), "cannot input output");
        self.inner_connect(Index(0), source, stream);
    }

    /// Disconnect the last-added matching connection from the destination,
    /// returning a boolean indicating if anything was disconnected.
    pub fn remove_input(&mut self, source: Index, stream: Option<usize>) {
        assert_ne!(source, Index(0), "cannot remove input");
        assert_ne!(source, Index(1), "cannot remove output");
        self.inner_disconnect(Index(0), source, stream);
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn output(&mut self, source: Index, stream: Option<usize>) {
        assert_ne!(source, Index(0), "cannot output input");
        assert_ne!(source, Index(1), "cannot output output");
        self.inner_connect(source, Index(1), stream);
    }

    /// Disconnect the last-added matching connection from the destination,
    /// returning a boolean indicating if anything was disconnected.
    pub fn remove_output(&mut self, source: Index, stream: Option<usize>) {
        assert_ne!(source, Index(0), "cannot remove input");
        assert_ne!(source, Index(1), "cannot remove output");
        self.inner_disconnect(source, Index(1), stream);
    }

    fn walk_node(&self, node: Index, process_list: &mut ProcessList) {
        if process_list.memo.insert(node) {
            process_list.list.push(node);
            let slot = self
                .nodes
                .get(node.0)
                .map(Option::as_ref)
                .flatten()
                .expect("walk_node found node that doesn't exist");
            for input in &slot.inputs {
                self.walk_node(input.source, process_list);
            }
        }
    }

    /// Get the processing list, in order from sink to roots.
    fn build_process_list(&self) {
        let mut process_list = self.process_list.borrow_mut();
        if process_list.reprocess {
            process_list.list.clear();
            process_list.memo.clear();
            // Special case the input node to ensure it's always at the end of
            // the list.
            process_list.memo.insert(Index(0));
            self.walk_node(Index(1), &mut process_list);
            if process_list.list.len() < self.nodes.len() {
                for index in self.set_nodes.iter().copied() {
                    self.walk_node(index, &mut process_list);
                }
            }
            process_list.list.push(Index(0));
            process_list.reprocess = false;
        }
    }

    /// Process all inputs from roots down to the sink.
    /// All sinks are added together to turn this into a single output.
    fn process<'a, 'b, 'c>(&'a mut self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        self.build_process_list();
        // First process all process-needing nodes in reverse order.
        for node in self.process_list.borrow().list.iter().rev().copied() {
            let slot = self.nodes[node.0].as_ref().expect("node needs to be set");
            let mut input_buffer = slot.input_buffer.borrow_mut();
            input_buffer.clear();
            if node == Index(0) {
                // The input node, 0, just gets the inputs from the outside world.
                input_buffer.extend_from_slice(inputs);
            } else if !slot.inputs.is_empty() {
                for input in slot.inputs.iter().copied() {
                    let input_slot = self.nodes[input.source.0]
                        .as_ref()
                        .expect("process node not in input values");
                    if let Some(output) = input.stream {
                        if let Some(stream) = input_slot.output.borrow().get(output).copied() {
                            input_buffer.push(stream);
                        }
                    } else {
                        input_buffer.extend_from_slice(&input_slot.output.borrow());
                    }
                }
            }
            let mut output = slot.output.borrow_mut();
            output.clear();
            slot.node.process(&input_buffer, &mut output);
        }
        outputs.extend_from_slice(
            &self.nodes[1]
                .as_ref()
                .expect("Sink does not exist")
                .output
                .borrow(),
        );
    }
}

#[derive(Debug, Default)]
pub struct Graph {
    inner: RefCell<InnerGraph>,
}

impl Graph {
    pub fn add(&self, node: Strong) -> Index {
        self.inner.borrow_mut().add(node)
    }

    pub fn remove(&self, index: Index) -> Option<Strong> {
        self.inner.borrow_mut().remove(index)
    }

    /// Connect the given output of the source to the destination.  The same
    /// output may be attached  multiple times. `None` will attach all outputs.
    pub fn connect(&self, source: Index, destination: Index, stream: Option<usize>) {
        self.inner.borrow_mut().connect(source, destination, stream);
    }

    /// Disconnect the last-added matching connection, returning a boolean
    /// indicating if anything was disconnected.
    pub fn disconnect(&self, source: Index, destination: Index, stream: Option<usize>) {
        self.inner
            .borrow_mut()
            .disconnect(source, destination, stream);
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn input(&self, source: Index, stream: Option<usize>) {
        self.inner.borrow_mut().input(source, stream);
    }

    /// Disconnect the last-added matching connection from the destination,
    /// returning a boolean indicating if anything was disconnected.
    pub fn remove_input(&self, source: Index, stream: Option<usize>) {
        self.inner.borrow_mut().remove_input(source, stream);
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn output(&self, source: Index, stream: Option<usize>) {
        self.inner.borrow_mut().output(source, stream);
    }

    /// Disconnect the last-added matching connection from the destination,
    /// returning a boolean indicating if anything was disconnected.
    pub fn remove_output(&self, source: Index, stream: Option<usize>) {
        self.inner.borrow_mut().remove_output(source, stream);
    }
}

impl Node for Graph {
    fn process<'a, 'b, 'c>(&'a self, inputs: &'b [Stream], outputs: &'c mut Vec<Stream>) {
        self.inner.borrow_mut().process(inputs, outputs)
    }

    fn node(self: Rc<Self>) -> Rc<dyn Node> {
        self
    }
}
