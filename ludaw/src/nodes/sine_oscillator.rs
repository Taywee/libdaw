use crate::get_channels;
use crate::get_sample_rate;
use crate::node::{ContainsFrequencyNode, ContainsNode, FrequencyNode};

use mlua::Lua;
use mlua::UserData;
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
    pub fn new(lua: &Lua, _: ()) -> mlua::Result<Self> {
        let node =
            libdaw::nodes::SineOscillator::new(get_sample_rate(lua)?, get_channels(lua)?).into();
        Ok(Self { node })
    }
}

impl UserData for SineOscillator {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        FrequencyNode::add_node_fields(fields);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        FrequencyNode::add_node_methods(methods);
    }
}
