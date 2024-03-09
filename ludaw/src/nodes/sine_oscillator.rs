use crate::get_channels;
use crate::get_sample_rate;
use crate::node::{ContainsFrequencyNode, ContainsNode, FrequencyNode};
use libdaw::FrequencyNode as _;
use lua::Lua;
use lua::UserData;
use mlua as lua;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SineOscillator {
    node: Rc<libdaw::nodes::SineOscillator>,
}

impl ContainsNode for SineOscillator {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl ContainsFrequencyNode for SineOscillator {
    fn frequency_node(&self) -> Rc<dyn libdaw::FrequencyNode> {
        self.node.clone()
    }
}

impl SineOscillator {
    pub fn new(lua: &Lua, _: ()) -> lua::Result<Self> {
        let node =
            libdaw::nodes::SineOscillator::new(get_sample_rate(lua)?, get_channels(lua)?).into();
        Ok(Self { node })
    }
}

impl UserData for SineOscillator {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        FrequencyNode::add_node_fields(fields);
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        FrequencyNode::add_node_methods(methods);
    }
}
