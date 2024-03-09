use crate::get_channels;
use crate::node::{ContainsNode, Node};
use mlua::Lua;
use mlua::UserData;
use std::rc::Rc;

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
    pub fn new(lua: &Lua, _: ()) -> mlua::Result<Self> {
        let node = libdaw::nodes::Add::new(get_channels(lua)?).into();
        Ok(Self { node })
    }
}

impl UserData for Add {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}
