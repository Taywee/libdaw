use crate::node::{ContainsFrequencyNode, ContainsNode, FrequencyNode};

use mlua::Lua;
use mlua::UserData;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct MultiFrequency {
    node: Rc<libdaw::nodes::MultiFrequency>,
}

impl ContainsNode for MultiFrequency {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl ContainsFrequencyNode for MultiFrequency {
    fn frequency_node(&self) -> Rc<dyn libdaw::FrequencyNode> {
        self.node.clone()
    }
}

impl MultiFrequency {
    pub fn new(_lua: &Lua, frequency_nodes: Vec<FrequencyNode>) -> mlua::Result<Self> {
        let node = libdaw::nodes::MultiFrequency::new(
            frequency_nodes
                .into_iter()
                .map(|node| node.frequency_node()),
        )
        .into();
        Ok(Self { node })
    }
}

impl UserData for MultiFrequency {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        FrequencyNode::add_node_fields(fields);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        FrequencyNode::add_node_methods(methods);
    }
}
