use crate::get_node;
use crate::ConcreteNode;
use crate::Node;
use libdaw::nodes::graph::Index;
use libdaw::Node as _;
use lua::{Lua, UserData};
use mlua as lua;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Graph {
    pub node: Rc<RefCell<libdaw::nodes::Graph>>,
}

impl ConcreteNode for Graph {
    fn node(&self) -> Rc<RefCell<dyn libdaw::Node>> {
        self.node.clone()
    }
}

impl Graph {
    pub fn new(_lua: &Lua, _: ()) -> lua::Result<Self> {
        let mut graph = libdaw::nodes::Graph::default();
        graph.set_channels(2);
        graph.set_sample_rate(48000);
        Ok(Self {
            node: Rc::new(RefCell::new(graph)),
        })
    }
}

impl UserData for Graph {
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
        methods.add_method_mut("add", |_, this, node| {
            let node = get_node(node)?.node();
            let mut this = this.node.borrow_mut();
            Ok(this.add(node).0)
        });
        methods.add_method_mut("remove", |_, this, index| {
            let mut this = this.node.borrow_mut();
            Ok(this.remove(Index(index)).map(Node::from))
        });
        methods.add_method_mut(
            "connect",
            |_, this, (source, destination, stream): (usize, usize, Option<usize>)| {
                let mut this = this.node.borrow_mut();
                this.connect(Index(source), Index(destination), stream);
                Ok(())
            },
        );
        methods.add_method_mut(
            "disconnect",
            |_, this, (source, destination, stream): (usize, usize, Option<usize>)| {
                let mut this = this.node.borrow_mut();
                this.disconnect(Index(source), Index(destination), stream);
                Ok(())
            },
        );
        methods.add_method_mut(
            "output",
            |_, this, (source, stream): (usize, Option<usize>)| {
                let mut this = this.node.borrow_mut();
                this.output(Index(source), stream);
                Ok(())
            },
        );
        methods.add_method_mut(
            "remove_output",
            |_, this, (source, stream): (usize, Option<usize>)| {
                let mut this = this.node.borrow_mut();
                this.remove_output(Index(source), stream);
                Ok(())
            },
        );
        methods.add_method_mut(
            "input",
            |_, this, (destination, stream): (usize, Option<usize>)| {
                let mut this = this.node.borrow_mut();
                this.input(Index(destination), stream);
                Ok(())
            },
        );
        methods.add_method_mut(
            "remove_input",
            |_, this, (destination, stream): (usize, Option<usize>)| {
                let mut this = this.node.borrow_mut();
                this.remove_input(Index(destination), stream);
                Ok(())
            },
        );
    }
}

#[derive(Debug, Clone)]
pub struct SquareOscillator {
    node: Rc<RefCell<libdaw::nodes::SquareOscillator>>,
}

impl ConcreteNode for SquareOscillator {
    fn node(&self) -> Rc<RefCell<dyn libdaw::Node>> {
        self.node.clone()
    }
}

impl SquareOscillator {
    pub fn new(_lua: &Lua, _: ()) -> lua::Result<Self> {
        let node: Rc<RefCell<libdaw::nodes::SquareOscillator>> = Default::default();
        Ok(Self { node })
    }
}

impl UserData for SquareOscillator {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("frequency", |_, this| {
            Ok(this.node.borrow_mut().get_frequency())
        });
        fields.add_field_method_set("frequency", |_, this, frequency| {
            this.node.borrow_mut().set_frequency(frequency);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

#[derive(Debug, Clone)]
pub struct SawtoothOscillator {
    node: Rc<RefCell<libdaw::nodes::SawtoothOscillator>>,
}

impl ConcreteNode for SawtoothOscillator {
    fn node(&self) -> Rc<RefCell<dyn libdaw::Node>> {
        self.node.clone()
    }
}

impl SawtoothOscillator {
    pub fn new(_lua: &Lua, _: ()) -> lua::Result<Self> {
        let node: Rc<RefCell<libdaw::nodes::SawtoothOscillator>> = Default::default();
        Ok(Self { node })
    }
}

impl UserData for SawtoothOscillator {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("frequency", |_, this| {
            Ok(this.node.borrow_mut().get_frequency())
        });
        fields.add_field_method_set("frequency", |_, this, frequency| {
            this.node.borrow_mut().set_frequency(frequency);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

#[derive(Debug, Clone)]
pub struct ConstantValue {
    node: Rc<RefCell<libdaw::nodes::ConstantValue>>,
}

impl ConcreteNode for ConstantValue {
    fn node(&self) -> Rc<RefCell<dyn libdaw::Node>> {
        self.node.clone()
    }
}

impl ConstantValue {
    pub fn new(_lua: &Lua, value: f64) -> lua::Result<Self> {
        let node = Rc::new(RefCell::new(libdaw::nodes::ConstantValue::new(value)));
        Ok(Self { node })
    }
}

impl UserData for ConstantValue {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| Ok(this.node.borrow_mut().get_value()));
        fields.add_field_method_set("value", |_, this, value| {
            this.node.borrow_mut().set_value(value);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

#[derive(Debug, Clone)]
pub struct Multiply {
    node: Rc<RefCell<libdaw::nodes::Multiply>>,
}

impl ConcreteNode for Multiply {
    fn node(&self) -> Rc<RefCell<dyn libdaw::Node>> {
        self.node.clone()
    }
}

impl Multiply {
    pub fn new(_lua: &Lua, _: ()) -> lua::Result<Self> {
        let node: Rc<RefCell<libdaw::nodes::Multiply>> = Default::default();
        Ok(Self { node })
    }
}

impl UserData for Multiply {
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

#[derive(Debug, Clone)]
pub struct Add {
    node: Rc<RefCell<libdaw::nodes::Add>>,
}

impl ConcreteNode for Add {
    fn node(&self) -> Rc<RefCell<dyn libdaw::Node>> {
        self.node.clone()
    }
}

impl Add {
    pub fn new(_lua: &Lua, _: ()) -> lua::Result<Self> {
        let node: Rc<RefCell<libdaw::nodes::Add>> = Default::default();
        Ok(Self { node })
    }
}

impl UserData for Add {
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

#[derive(Debug, Clone)]
pub struct Delay {
    node: Rc<RefCell<libdaw::nodes::Delay>>,
}

impl ConcreteNode for Delay {
    fn node(&self) -> Rc<RefCell<dyn libdaw::Node>> {
        self.node.clone()
    }
}

impl Delay {
    pub fn new(_lua: &Lua, value: f64) -> lua::Result<Self> {
        let node = Rc::new(RefCell::new(libdaw::nodes::Delay::new(
            Duration::from_secs_f64(value),
        )));
        Ok(Self { node })
    }
}

impl UserData for Delay {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("delay", |_, this| {
            Ok(this.node.borrow_mut().get_delay().as_secs_f64())
        });
        fields.add_field_method_set("delay", |_, this, delay| {
            this.node
                .borrow_mut()
                .set_delay(Duration::from_secs_f64(delay));
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}
