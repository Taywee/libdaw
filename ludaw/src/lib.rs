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

#[derive(Debug, Clone)]
struct Node(Rc<RefCell<dyn libdaw::Node>>);

impl<T> From<Rc<RefCell<T>>> for Node
where
    T: libdaw::Node + 'static,
{
    fn from(value: Rc<RefCell<T>>) -> Self {
        Node(value.clone())
    }
}

impl UserData for Node {
    fn add_fields<'lua, F>(fields: &mut F)
    where
        F: lua::UserDataFields<'lua, Self>,
    {
        fields.add_field_method_set("sample_rate", |_, this, sample_rate| {
            this.0.borrow_mut().set_sample_rate(sample_rate);
            Ok(())
        });
    }
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // node:node() clones itself for convenience.
        methods.add_method("node", |_, this, ()| Ok(this.clone()));
    }
}

impl<'lua> FromLua<'lua> for Node {
    fn from_lua(value: lua::Value<'lua>, _lua: &'lua Lua) -> lua::Result<Self> {
        let type_name = value.type_name();
        let lua::Value::UserData(ud) = value else {
            return Err(lua::Error::FromLuaConversionError {
                from: type_name,
                to: "Node",
                message: None,
            });
        };
        if !ud.is::<Node>() {
            return Err(lua::Error::FromLuaConversionError {
                from: type_name,
                to: "Node",
                message: None,
            });
        }
        let node: Ref<Node> = ud.borrow()?;
        Ok(Node(node.0.clone()))
    }
}
