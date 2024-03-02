mod callable;
pub mod error;
mod indexable;
mod nodes;
mod track;

pub use track::{Track, TrackSource};

use lua::{AnyUserDataExt as _, FromLua, Lua, UserData};
use mlua as lua;
use std::cell::Ref;

use std::rc::Rc;

pub trait ContainsNode {
    fn node(&self) -> Rc<dyn libdaw::Node>;
}

#[derive(Debug, Clone)]
struct Node {
    node: Rc<dyn libdaw::Node>,
}

impl From<Rc<dyn libdaw::Node>> for Node {
    fn from(node: Rc<dyn libdaw::Node>) -> Self {
        Self { node }
    }
}

impl ContainsNode for Node {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

// Get the sample rate if it exists, or set it to the default of 48000
pub fn get_sample_rate(lua: &Lua) -> lua::Result<u32> {
    if let Some(sample_rate) = lua.named_registry_value("daw.sample_rate")? {
        Ok(sample_rate)
    } else {
        lua.set_named_registry_value("daw.sample_rate", 48000u32)?;
        Ok(48000)
    }
}

// Get the channel count if it exists, or set it to the default of 2
pub fn get_channels(lua: &Lua) -> lua::Result<u16> {
    if let Some(channels) = lua.named_registry_value("daw.channels")? {
        Ok(channels)
    } else {
        lua.set_named_registry_value("daw.channels", 2u16)?;
        Ok(2)
    }
}

impl Node {
    pub fn add_node_fields<'lua, T: UserData + ContainsNode, F: lua::UserDataFields<'lua, T>>(
        _fields: &mut F,
    ) {
    }
    pub fn add_node_methods<'lua, T: UserData + ContainsNode, M: lua::UserDataMethods<'lua, T>>(
        methods: &mut M,
    ) {
        methods.add_method("node", |_, this, ()| Ok(Node::from(this.node())));
    }
}

impl UserData for Node {
    fn add_fields<'lua, F>(fields: &mut F)
    where
        F: lua::UserDataFields<'lua, Self>,
    {
        Node::add_node_fields(fields);
    }
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

/// Check if the data is a Node, otherwise repeatedly call the "node" method
/// until we get one.
impl<'lua> FromLua<'lua> for Node {
    fn from_lua(value: lua::Value<'lua>, lua: &'lua Lua) -> lua::Result<Self> {
        match value {
            lua::Value::Table(table) => {
                let node_func: lua::Function = table.get("node")?;
                Node::from_lua(node_func.call(table)?, lua)
            }
            lua::Value::UserData(ud) => {
                if ud.is::<Self>() {
                    let node: Ref<Self> = ud.borrow()?;
                    Ok((*node).clone())
                } else {
                    let node_func: lua::Function = ud.get("node")?;
                    Node::from_lua(node_func.call(ud)?, lua)
                }
            }
            _ => Err(lua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Node",
                message: Some("A node function must be receieved from a table or userdata".into()),
            }),
        }
    }
}
