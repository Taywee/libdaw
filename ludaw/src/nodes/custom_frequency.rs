use crate::{
    indexable::Indexable,
    lua_state::LuaState,
    node::{ContainsFrequencyNode, ContainsNode, FrequencyNode, Stream},
};
use mlua::{Lua, RegistryKey, UserData, UserDataFields, UserDataMethods};
use std::rc::Rc;

#[derive(Debug)]
struct LuaNode {
    key: RegistryKey,
    lua_state: LuaState,
}

impl LuaNode {
    pub fn new(lua: &Lua, object: Indexable) -> mlua::Result<Self> {
        let key = lua.create_registry_value(object)?;
        let lua_state: LuaState = lua.named_registry_value("daw.lua_state")?;
        Ok(Self { key, lua_state })
    }
}

impl libdaw::Node for LuaNode {
    fn process<'a, 'b, 'c>(
        &'a self,
        inputs: &'b [libdaw::Stream],
        outputs: &'c mut Vec<libdaw::Stream>,
    ) {
        let Some(lua) = self.lua_state.state.upgrade() else {
            unreachable!("The graph should not be callable after Lua has been destructed");
        };
        let input: Vec<_> = inputs.iter().copied().map(Stream).collect();

        let lua_object: Indexable = lua
            .registry_value(&self.key)
            .expect("set key should be an indexable");
        let output: Vec<Stream> = lua_object
            .call(input)
            .expect("process nodes can not throw errors properly yet");
        outputs.extend(output.into_iter().map(|stream| stream.0));
    }
}

impl libdaw::FrequencyNode for LuaNode {
    fn get_frequency(&self) -> f64 {
        let Some(lua) = self.lua_state.state.upgrade() else {
            unreachable!("The graph should not be callable after Lua has been destructed");
        };
        let lua_object: Indexable = lua
            .registry_value(&self.key)
            .expect("set key should be an indexable");
        lua_object
            .get("frequency")
            .expect("frequency should be a number")
    }

    fn set_frequency(&self, frequency: f64) {
        let Some(lua) = self.lua_state.state.upgrade() else {
            unreachable!("The graph should not be callable after Lua has been destructed");
        };
        let lua_object: Indexable = lua
            .registry_value(&self.key)
            .expect("set key should be an indexable");
        lua_object
            .set("frequency", frequency)
            .expect("frequency should be a number")
    }
}

#[derive(Debug, Clone)]
pub struct CustomFrequency {
    node: Rc<LuaNode>,
}

impl CustomFrequency {
    pub fn new(lua: &Lua, callable: Indexable) -> mlua::Result<Self> {
        Ok(Self {
            node: Rc::new(LuaNode::new(lua, callable)?),
        })
    }
}

impl UserData for CustomFrequency {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        FrequencyNode::add_node_methods(methods);
    }

    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        FrequencyNode::add_node_fields(fields);
    }
}

impl ContainsNode for CustomFrequency {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}
impl ContainsFrequencyNode for CustomFrequency {
    fn frequency_node(&self) -> Rc<dyn libdaw::FrequencyNode> {
        self.node.clone()
    }
}
