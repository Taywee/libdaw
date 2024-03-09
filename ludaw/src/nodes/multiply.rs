use crate::node::{ContainsNode, Node};
use mlua::Lua;
use mlua::UserData;
use std::rc::Rc;

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
    pub fn new(_lua: &Lua, _: ()) -> mlua::Result<Self> {
        let node: Rc<libdaw::nodes::Multiply> = Default::default();
        Ok(Self { node })
    }
}

impl UserData for Multiply {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}
