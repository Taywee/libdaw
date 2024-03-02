use crate::get_channels;
use crate::get_sample_rate;
use crate::ContainsNode;
use crate::Node;
use libdaw::nodes::graph::Index;
use libdaw::FrequencyNode as _;

use lua::Table;
use lua::{Lua, UserData};
use mlua as lua;
use std::rc::Rc;
use std::time::Duration;

pub fn setup_module<'a>(lua: &'a Lua, _: ()) -> lua::Result<Table<'a>> {
    let module = lua.create_table()?;
    module.set("Graph", lua.create_function(Graph::new)?)?;
    module.set(
        "SquareOscillator",
        lua.create_function(SquareOscillator::new)?,
    )?;
    module.set(
        "SawtoothOscillator",
        lua.create_function(SawtoothOscillator::new)?,
    )?;
    module.set("ConstantValue", lua.create_function(ConstantValue::new)?)?;
    module.set("Add", lua.create_function(Add::new)?)?;
    module.set("Multiply", lua.create_function(Multiply::new)?)?;
    module.set("Delay", lua.create_function(Delay::new)?)?;
    module.set("Gain", lua.create_function(Gain::new)?)?;
    Ok(module)
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub node: Rc<libdaw::nodes::Graph>,
}

impl ContainsNode for Graph {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Graph {
    pub fn new(_lua: &Lua, _: ()) -> lua::Result<Self> {
        let node = libdaw::nodes::Graph::default().into();
        Ok(Self { node })
    }
}

impl UserData for Graph {
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
        methods.add_method_mut("add", |_, this, node: Node| Ok(this.node.add(node.node).0));
        methods.add_method_mut("remove", |_, this, index| {
            Ok(this.node.remove(Index(index)).map(Node::from))
        });
        methods.add_method_mut(
            "connect",
            |_, this, (source, destination, stream): (usize, usize, Option<usize>)| {
                this.node.connect(Index(source), Index(destination), stream);
                Ok(())
            },
        );
        methods.add_method_mut(
            "disconnect",
            |_, this, (source, destination, stream): (usize, usize, Option<usize>)| {
                this.node
                    .disconnect(Index(source), Index(destination), stream);
                Ok(())
            },
        );
        methods.add_method_mut(
            "output",
            |_, this, (source, stream): (usize, Option<usize>)| {
                this.node.output(Index(source), stream);
                Ok(())
            },
        );
        methods.add_method_mut(
            "remove_output",
            |_, this, (source, stream): (usize, Option<usize>)| {
                this.node.remove_output(Index(source), stream);
                Ok(())
            },
        );
        methods.add_method_mut(
            "input",
            |_, this, (destination, stream): (usize, Option<usize>)| {
                this.node.input(Index(destination), stream);
                Ok(())
            },
        );
        methods.add_method_mut(
            "remove_input",
            |_, this, (destination, stream): (usize, Option<usize>)| {
                this.node.remove_input(Index(destination), stream);
                Ok(())
            },
        );
    }

    fn add_fields<'lua, F: lua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }
}

#[derive(Debug, Clone)]
pub struct SquareOscillator {
    node: Rc<libdaw::nodes::SquareOscillator>,
}

impl ContainsNode for SquareOscillator {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl SquareOscillator {
    pub fn new(lua: &Lua, _: ()) -> lua::Result<Self> {
        let node =
            libdaw::nodes::SquareOscillator::new(get_sample_rate(lua)?, get_channels(lua)?).into();
        Ok(Self { node })
    }
}

impl UserData for SquareOscillator {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_get("frequency", |_, this| Ok(this.node.get_frequency()));
        fields.add_field_method_set("frequency", |_, this, frequency| {
            this.node.set_frequency(frequency);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

#[derive(Debug, Clone)]
pub struct SawtoothOscillator {
    node: Rc<libdaw::nodes::SawtoothOscillator>,
}

impl ContainsNode for SawtoothOscillator {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl SawtoothOscillator {
    pub fn new(lua: &Lua, _: ()) -> lua::Result<Self> {
        let node =
            libdaw::nodes::SawtoothOscillator::new(get_sample_rate(lua)?, get_channels(lua)?)
                .into();
        Ok(Self { node })
    }
}

impl UserData for SawtoothOscillator {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_get("frequency", |_, this| Ok(this.node.get_frequency()));
        fields.add_field_method_set("frequency", |_, this, frequency| {
            this.node.set_frequency(frequency);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

#[derive(Debug, Clone)]
pub struct ConstantValue {
    node: Rc<libdaw::nodes::ConstantValue>,
}

impl ContainsNode for ConstantValue {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl ConstantValue {
    pub fn new(lua: &Lua, value: f64) -> lua::Result<Self> {
        let node = libdaw::nodes::ConstantValue::new(get_channels(lua)?, value).into();
        Ok(Self { node })
    }
}

impl UserData for ConstantValue {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_get("value", |_, this| Ok(this.node.get_value()));
        fields.add_field_method_set("value", |_, this, value| {
            this.node.set_value(value);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

#[derive(Debug, Clone)]
pub struct Multiply {
    node: Rc<libdaw::nodes::Multiply>,
}

impl ContainsNode for Multiply {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Multiply {
    pub fn new(_lua: &Lua, _: ()) -> lua::Result<Self> {
        let node: Rc<libdaw::nodes::Multiply> = Default::default();
        Ok(Self { node })
    }
}

impl UserData for Multiply {
    fn add_fields<'lua, F: lua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

#[derive(Debug, Clone)]
pub struct Add {
    node: Rc<libdaw::nodes::Add>,
}

impl ContainsNode for Add {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Add {
    pub fn new(lua: &Lua, _: ()) -> lua::Result<Self> {
        let node = libdaw::nodes::Add::new(get_channels(lua)?).into();
        Ok(Self { node })
    }
}

impl UserData for Add {
    fn add_fields<'lua, F: lua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

#[derive(Debug, Clone)]
pub struct Delay {
    node: Rc<libdaw::nodes::Delay>,
}

impl ContainsNode for Delay {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Delay {
    pub fn new(lua: &Lua, value: f64) -> lua::Result<Self> {
        let node =
            libdaw::nodes::Delay::new(get_sample_rate(lua)?, Duration::from_secs_f64(value)).into();
        Ok(Self { node })
    }
}

impl UserData for Delay {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}
#[derive(Debug, Clone)]
pub struct Gain {
    node: Rc<libdaw::nodes::Gain>,
}

impl ContainsNode for Gain {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Gain {
    pub fn new(_lua: &Lua, gain: f64) -> lua::Result<Self> {
        let node = libdaw::nodes::Gain::new(gain).into();
        Ok(Self { node })
    }
}

impl UserData for Gain {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_get("gain", |_, this| Ok(this.node.get_gain()));
        fields.add_field_method_set("gain", |_, this, gain| {
            this.node.set_gain(gain);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}
