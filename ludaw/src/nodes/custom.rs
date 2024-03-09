use std::rc::Rc;

use crate::{
    callable::Callable,
    lua_state::LuaState,
    node::{ContainsNode, Node, Stream},
};
use mlua::{Lua, RegistryKey, UserData, UserDataFields, UserDataMethods};

#[derive(Debug)]
struct LuaCallbackNode {
    callable_key: RegistryKey,
    lua_state: LuaState,
}

impl LuaCallbackNode {
    pub fn new(lua: &Lua, callable: Callable) -> mlua::Result<Self> {
        let callable_key = lua.create_registry_value(callable)?;
        let lua_state: LuaState = lua.named_registry_value("daw.lua_state")?;
        Ok(Self {
            callable_key,
            lua_state,
        })
    }
}

impl libdaw::Node for LuaCallbackNode {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [libdaw::Stream],
        outputs: &'c mut Vec<libdaw::Stream>,
    ) {
        let Some(lua) = self.lua_state.state.upgrade() else {
            unreachable!("The graph should not be callable after Lua has been destructed");
        };
        let input: Vec<_> = inputs.iter().copied().map(Stream).collect();

        let callable: Callable = lua
            .registry_value(&self.callable_key)
            .expect("set key should be a function");
        let output: Vec<Stream> = callable
            .call(input)
            .expect("process nodes can not throw errors properly yet");
        outputs.extend(output.into_iter().map(|stream| stream.0));
    }
}

#[derive(Debug, Clone)]
pub struct Custom {
    node: Rc<LuaCallbackNode>,
}

impl Custom {
    pub fn new(lua: &Lua, callable: Callable) -> mlua::Result<Self> {
        Ok(Self {
            node: Rc::new(LuaCallbackNode::new(lua, callable)?),
        })
    }
}

impl UserData for Custom {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }

    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }
}

impl ContainsNode for Custom {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}
