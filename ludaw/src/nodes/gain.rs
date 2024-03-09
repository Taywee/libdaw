use crate::node::{ContainsNode, Node};

use mlua::Lua;
use mlua::UserData;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Gain {
    node: Rc<libdaw::nodes::Gain>,
}

impl ContainsNode for Gain {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Gain {
    pub fn new(_lua: &Lua, gain: f64) -> mlua::Result<Self> {
        let node = libdaw::nodes::Gain::new(gain).into();
        Ok(Self { node })
    }
}

impl UserData for Gain {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_get("gain", |_, this| Ok(this.node.get_gain()));
        fields.add_field_method_set("gain", |_, this, gain| {
            this.node.set_gain(gain);
            Ok(())
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}
