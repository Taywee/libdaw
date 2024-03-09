use crate::get_channels;
use crate::node::{ContainsNode, Node};
use mlua::Lua;
use mlua::UserData;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ConstantValue {
    node: Rc<libdaw::nodes::ConstantValue>,
}

impl ContainsNode for ConstantValue {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl ConstantValue {
    pub fn new(lua: &Lua, value: f64) -> mlua::Result<Self> {
        let node = libdaw::nodes::ConstantValue::new(get_channels(lua)?, value).into();
        Ok(Self { node })
    }
}

impl UserData for ConstantValue {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_get("value", |_, this| Ok(this.node.get_value()));
        fields.add_field_method_set("value", |_, this, value| {
            this.node.set_value(value);
            Ok(())
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}
