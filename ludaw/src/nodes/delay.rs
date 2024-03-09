use crate::get_sample_rate;
use crate::node::{ContainsNode, Node};

use mlua::Lua;
use mlua::UserData;
use std::rc::Rc;
use std::time::Duration;

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
    pub fn new(lua: &Lua, value: f64) -> mlua::Result<Self> {
        let node =
            libdaw::nodes::Delay::new(get_sample_rate(lua)?, Duration::from_secs_f64(value)).into();
        Ok(Self { node })
    }
}

impl UserData for Delay {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}
