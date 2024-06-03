mod strong;

use crate::{nodes::Passthrough, sample::Sample, Node};
use nohash_hasher::{IntMap, IntSet};
use std::sync::{Arc, Mutex};
use strong::Strong;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Input {
    source: Strong,
    stream: Option<usize>,
}

#[derive(Debug, Default)]
struct Slot {
    output: Vec<Sample>,
    inputs: Vec<Input>,
}

#[derive(Debug, Default)]
pub struct ProcessList {
    list: Vec<Strong>,
    memo: IntSet<Strong>,
    needs_rebuild: bool,
}

#[derive(Debug)]
pub struct Graph {
    nodes: IntMap<Strong, Slot>,
    input: Strong,
    output: Strong,
    process_list: ProcessList,
}

impl Default for Graph {
    fn default() -> Self {
        let input = Strong {
            inner: Arc::new(Mutex::new(Passthrough::default())),
        };
        let output = Strong {
            inner: Arc::new(Mutex::new(Passthrough::default())),
        };
        let graph = Self {
            nodes: Default::default(),
            input,
            output,
            process_list: Default::default(),
        };
        graph
    }
}

impl Graph {
    fn add_if_needed(&mut self, node: Strong) -> &mut Slot {
        self.nodes.entry(node).or_default()
    }

    pub fn remove(&mut self, node: Arc<Mutex<dyn Node>>) -> bool {
        let node = Strong { inner: node };
        if let Some((node, _)) = self.nodes.remove_entry(&node) {
            for slot in self.nodes.values_mut() {
                slot.inputs.retain(|input| input.source != node);
            }
            self.process_list.needs_rebuild = true;
            true
        } else {
            false
        }
    }

    pub fn connect(
        &mut self,
        source: Arc<Mutex<dyn Node>>,
        destination: Arc<Mutex<dyn Node>>,
        stream: Option<usize>,
    ) {
        self.process_list.needs_rebuild = true;
        let source = Strong { inner: source };
        let destination = Strong { inner: destination };
        self.add_if_needed(source.clone());
        self.add_if_needed(destination)
            .inputs
            .push(Input { source, stream });
    }

    fn remove_if_unneeded(&mut self, node: Strong) {
        // Check if this needs another node
        if self
            .nodes
            .get(&node)
            .iter()
            .flat_map(|&slot| slot.inputs.iter())
            .next()
            .is_some()
        {
            return;
        }
        // Check if this is needed by another node
        if self
            .nodes
            .values()
            .flat_map(|slot| slot.inputs.iter().map(|input| &input.source))
            .find(|&strong| node == *strong)
            .is_some()
        {
            return;
        }
        self.nodes.remove(&node);
    }

    pub fn disconnect(
        &mut self,
        source: Arc<Mutex<dyn Node>>,
        destination: Arc<Mutex<dyn Node>>,
        stream: Option<usize>,
    ) -> bool {
        let source = Strong { inner: source };
        let destination = Strong { inner: destination };
        let Some(destination_slot) = self.nodes.get_mut(&destination) else {
            return false;
        };
        let Some((index, _)) = destination_slot
            .inputs
            .iter()
            .enumerate()
            .rfind(|&(_, input)| input.source == source && input.stream == stream)
        else {
            return false;
        };
        self.process_list.needs_rebuild = true;
        destination_slot.inputs.remove(index);

        self.remove_if_unneeded(source);
        self.remove_if_unneeded(destination);

        true
    }

    /// Connect the given output of the initial input to the destination.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn input(&mut self, destination: Arc<Mutex<dyn Node>>, stream: Option<usize>) {
        self.connect(self.input.clone().inner, destination, stream);
    }

    /// Disconnect the last-added matching connection from the destination,
    /// returning a boolean indicating if anything was disconnected.
    pub fn remove_input(
        &mut self,
        destination: Arc<Mutex<dyn Node>>,
        stream: Option<usize>,
    ) -> bool {
        self.disconnect(self.input.clone().inner, destination, stream)
    }

    /// Connect the given output of the source to the final destinaton.  The
    /// same output may be attached multiple times. `None` will attach all
    /// outputs.
    pub fn output(&mut self, source: Arc<Mutex<dyn Node>>, stream: Option<usize>) {
        self.connect(source, self.output.clone().inner, stream);
    }

    /// Disconnect the last-added matching connection from the source, returning
    /// a boolean indicating if anything was disconnected.
    pub fn remove_output(&mut self, source: Arc<Mutex<dyn Node>>, stream: Option<usize>) -> bool {
        self.disconnect(source, self.output.clone().inner, stream)
    }

    fn walk_node(&mut self, node: Strong) {
        if self.process_list.memo.insert(node.clone()) {
            self.process_list.list.push(node.clone());
            let sources: Vec<_> = self
                .nodes
                .get(&node)
                .iter()
                .flat_map(|&slot| &slot.inputs)
                .map(|input| input.source.clone())
                .collect();
            for source in sources {
                self.walk_node(source);
            }
        }
    }

    fn check_process_list(&mut self) {
        if self.process_list.needs_rebuild {
            self.process_list.needs_rebuild = false;
            self.process_list.memo.clear();
            self.process_list.list.clear();
            self.walk_node(self.output.clone());
        }
    }
}

impl Node for Graph {
    /// Process all inputs from roots down to the sink.
    /// All sinks are added together to turn this into a single output.
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> crate::Result<()> {
        self.check_process_list();
        let mut input_buffer = Vec::new();
        // First process all process-needing nodes in reverse order.
        for node in self.process_list.list.iter().rev() {
            input_buffer.clear();
            if *node == self.input {
                // The input node just gets the inputs from the outside world.
                input_buffer.extend_from_slice(inputs);
            } else {
                for input in self
                    .nodes
                    .get(node)
                    .expect("node needs to be set")
                    .inputs
                    .iter()
                {
                    let input_slot = self
                        .nodes
                        .get(&input.source)
                        .expect("process node not in input values");
                    if let Some(output) = input.stream {
                        if let Some(stream) = input_slot.output.get(output).cloned() {
                            input_buffer.push(stream);
                        }
                    } else {
                        input_buffer.extend_from_slice(&input_slot.output);
                    }
                }
            }
            let slot = self.nodes.get_mut(node).expect("node needs to be set");
            slot.output.clear();
            node.lock()
                .expect("poisoned")
                .process(&input_buffer, &mut slot.output)?;
        }
        if let Some(slot) = self.nodes.get_mut(&self.output) {
            outputs.extend_from_slice(&slot.output);
        }
        Ok(())
    }
}
