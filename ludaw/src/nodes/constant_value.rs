use crate::get_channels;
use crate::node::{ContainsNode, Node};
use lua::Lua;
use lua::UserData;
use mlua as lua;
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
    pub fn new(lua: &Lua, value: f64) -> lua::Result<Self> {
        let node = libdaw::nodes::ConstantValue::new(get_channels(lua)?, value).into();
        Ok(Self { node })
    }
}

impl UserData for ConstantValue {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_get("value", |_, this| Ok(this.node.get_value()));
        fields.add_field_method_set("value", |_, this, value| {
            this.node.set_value(value);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}
