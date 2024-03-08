use crate::get_channels;
use crate::get_sample_rate;
use crate::node::{ContainsFrequencyNode, ContainsNode, FrequencyNode};
use libdaw::FrequencyNode as _;
use lua::Lua;
use lua::UserData;
use mlua as lua;
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
    pub fn new(lua: &Lua, frequency_nodes: Vec<FrequencyNode>) -> lua::Result<Self> {
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
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        FrequencyNode::add_node_fields(fields);
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        FrequencyNode::add_node_methods(methods);
    }
}
