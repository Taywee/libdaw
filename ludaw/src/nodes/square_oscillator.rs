use crate::get_channels;
use crate::get_sample_rate;
use crate::node::{ContainsFrequencyNode, ContainsNode, FrequencyNode};
use libdaw::FrequencyNode as _;
use lua::Lua;
use lua::UserData;
use mlua as lua;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SquareOscillator {
    node: Rc<libdaw::nodes::SquareOscillator>,
}

impl ContainsNode for SquareOscillator {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl ContainsFrequencyNode for SquareOscillator {
    fn frequency_node(&self) -> Rc<dyn libdaw::FrequencyNode> {
        self.node.clone()
    }
}

impl SquareOscillator {
    pub fn new(lua: &Lua, _: ()) -> lua::Result<Self> {
        let node =
            libdaw::nodes::SquareOscillator::new(get_sample_rate(lua)?, get_channels(lua)?).into();
        Ok(Self { node })
    }
}

impl UserData for SquareOscillator {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        FrequencyNode::add_node_fields(fields);
        fields.add_field_method_get("frequency", |_, this| Ok(this.node.get_frequency()));
        fields.add_field_method_set("frequency", |_, this, frequency| {
            this.node.set_frequency(frequency);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        FrequencyNode::add_node_methods(methods);
    }
}
