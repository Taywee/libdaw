use crate::node::ContainsNode;
use crate::node::{ContainsFrequencyNode, FrequencyNode};

use mlua::Lua;

use mlua::UserData;
use std::rc::Rc;

#[derive(Debug, Clone)]

pub struct Detune {
    node: Rc<libdaw::nodes::Detune>,
}

impl ContainsNode for Detune {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl ContainsFrequencyNode for Detune {
    fn frequency_node(&self) -> Rc<dyn libdaw::FrequencyNode> {
        self.node.clone()
    }
}

impl Detune {
    pub fn new(_lua: &Lua, node: FrequencyNode) -> mlua::Result<Self> {
        let node = Rc::new(libdaw::nodes::Detune::new(node.node));
        Ok(Self { node })
    }
}

impl UserData for Detune {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        FrequencyNode::add_node_methods(methods);
    }

    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        FrequencyNode::add_node_fields(fields);
        fields.add_field_method_get("detune", |_, this| Ok(this.node.get_detune()));
        fields.add_field_method_set("detune", |_, this, detune| {
            this.node.set_detune(detune);
            Ok(())
        });
    }
}
