mod callable;
pub mod error;
mod nodes;
mod track;

pub use track::{Track, TrackSource};

use lua::{AnyUserDataExt as _, FromLua, Lua, UserData};
use mlua as lua;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

fn get_node<'lua>(value: lua::Value<'lua>) -> lua::Result<Node> {
    match value {
        lua::Value::Table(table) => {
            let node_func: lua::Function = table.get("node")?;
            node_func.call(table)
        }
        lua::Value::UserData(ud) => {
            let node_func: lua::Function = ud.get("node")?;
            node_func.call(ud)
        }
        _ => Err(lua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Node",
            message: Some("A node function must be receieved from a table or userdata".into()),
        }),
    }
}

pub trait ConcreteNode {
    fn node(&self) -> Rc<RefCell<dyn libdaw::Node>>;
}

#[derive(Debug, Clone)]
struct Node {
    node: Rc<RefCell<dyn libdaw::Node>>,
}

impl From<Rc<RefCell<dyn libdaw::Node>>> for Node {
    fn from(node: Rc<RefCell<dyn libdaw::Node>>) -> Self {
        Self { node }
    }
}

impl ConcreteNode for Node {
    fn node(&self) -> Rc<RefCell<dyn libdaw::Node>> {
        self.node.clone()
    }
}

impl Node {
    fn new(node: Rc<RefCell<dyn libdaw::Node>>) -> Self {
        Self { node }
    }

    pub fn add_node_methods<'lua, T: UserData + ConcreteNode, M: lua::UserDataMethods<'lua, T>>(
        methods: &mut M,
    ) {
        methods.add_method("node", |_, this, ()| Ok(Node::new(this.node())));
    }
}

impl UserData for Node {
    fn add_fields<'lua, F>(fields: &mut F)
    where
        F: lua::UserDataFields<'lua, Self>,
    {
        fields.add_field_method_set("sample_rate", |_, this, sample_rate| {
            this.node.borrow_mut().set_sample_rate(sample_rate);
            Ok(())
        });
    }
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

impl<'lua> FromLua<'lua> for Node {
    fn from_lua(value: lua::Value<'lua>, _lua: &'lua Lua) -> lua::Result<Self> {
        let lua::Value::UserData(ud) = value else {
            return Err(lua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Node",
                message: None,
            });
        };
        let node: Ref<Self> = ud.borrow()?;
        Ok((*node).clone())
    }
}
