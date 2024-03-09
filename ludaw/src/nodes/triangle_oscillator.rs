use crate::get_channels;
use crate::get_sample_rate;
use crate::node::{ContainsFrequencyNode, ContainsNode, FrequencyNode};
use libdaw::FrequencyNode as _;
use lua::Lua;
use lua::UserData;
use mlua as lua;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TriangleOscillator {
    node: Rc<libdaw::nodes::TriangleOscillator>,
}

impl ContainsNode for TriangleOscillator {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl ContainsFrequencyNode for TriangleOscillator {
    fn frequency_node(&self) -> Rc<dyn libdaw::FrequencyNode> {
        self.node.clone()
    }
}

impl TriangleOscillator {
    pub fn new(lua: &Lua, _: ()) -> lua::Result<Self> {
        let node =
            libdaw::nodes::TriangleOscillator::new(get_sample_rate(lua)?, get_channels(lua)?)
                .into();
        Ok(Self { node })
    }
}

impl UserData for TriangleOscillator {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        FrequencyNode::add_node_fields(fields);
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        FrequencyNode::add_node_methods(methods);
    }
}
