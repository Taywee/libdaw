use crate::get_sample_rate;
use crate::node::{ContainsNode, Node};

use lua::Lua;
use lua::UserData;
use mlua as lua;
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
