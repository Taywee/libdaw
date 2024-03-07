use crate::get_sample_rate;
use crate::indexable::Indexable;
use crate::lua_state::LuaState;
use crate::node::FrequencyNode;
use crate::node::{ContainsNode, Node};
use lua::FromLua;
use lua::Lua;
use lua::Table;
use lua::UserData;
use mlua as lua;
use std::rc::Rc;
use std::time::Duration;
#[derive(Debug, Clone)]

pub struct Detune {
    node: Rc<libdaw::nodes::Detune>,
}

impl ContainsNode for Detune {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Detune {
    pub fn new(lua: &Lua, node: FrequencyNode) -> lua::Result<Self> {
        let node = Rc::new(libdaw::nodes::Detune::new(node.node));
        Ok(Self { node })
    }
}

impl UserData for Detune {
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }

    fn add_fields<'lua, F: lua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_get("detune", |_, this| Ok(this.node.get_detune()));
        fields.add_field_method_set("detune", |_, this, detune| {
            this.node.set_detune(detune);
            Ok(())
        });
    }
}
