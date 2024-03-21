use crate::{
    get_sample_rate,
    node::{ContainsNode, Node},
};
use libdaw::time::Duration;
use mlua::{Lua, UserData};
use std::rc::Rc;

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
    pub fn new(lua: &Lua, seconds: f64) -> mlua::Result<Self> {
        let delay = Duration::from_seconds(seconds).map_err(mlua::Error::external)?;
        let node = libdaw::nodes::Delay::new(get_sample_rate(lua)?, delay).into();
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
