use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};

// TODO: Apply these changes to existing nodes

/// An audio node trait, allowing a node to be set, updated from inputs, and to
/// generate samples.
pub trait Node: Debug + Iterator<Item = f64> {
    fn set_sample_rate(&mut self, sample_rate: u32);
    fn update(
        &mut self,
        input_channels: &mut dyn Iterator<Item = u16>,
        inputs: &mut dyn Iterator<Item = f64>,
    );

    fn channels(&self) -> u16 {
        1
    }
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

#[derive(Debug)]
struct Input {
    output: usize,
    source: StrongNode,
}

#[derive(Default, Debug)]
pub struct Graph {
    // Connect a node to its inputs.
    inputs: HashMap<StrongNode, Vec<Input>>,
}

impl Graph {
    pub fn connect(
        &mut self,
        source: Rc<RefCell<dyn Node>>,
        output: usize,
        destination: Rc<RefCell<dyn Node>>,
    ) {
        self.inputs
            .entry(StrongNode(destination))
            .or_default()
            .push(Input {
                source: StrongNode(source),
                output,
            });
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
    fn reset(&mut self) {
        self.frequency = 256.0;
        self.samples_since_switch = 0.0;
        self.sample = 1.0;
        self.calculate_samples_per_switch();
    }

    fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
        self.calculate_samples_per_switch();
    }

    fn update(&mut self, _inputs: &[&[f64]], outputs: &mut Vec<Vec<f64>>) {
        let sample = self.sample;
        while self.samples_since_switch >= self.samples_per_switch {
            self.samples_since_switch -= self.samples_per_switch;
            self.sample *= -1.0;
        }
        self.samples_since_switch += 1.0;
        outputs.push(vec![sample]);
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
    fn reset(&mut self) {
        self.frequency = 256.0;
        self.sample = 1.0;
        self.calculate_delta();
    }

    fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
        self.calculate_delta();
    }

    fn update(&mut self, _inputs: &[&[f64]], outputs: &mut Vec<Vec<f64>>) {
        let sample = self.sample;
        self.sample += self.delta;
        while self.sample > 1.0 {
            self.sample -= 1.0;
        }

        outputs.push(vec![sample]);
    }
}
