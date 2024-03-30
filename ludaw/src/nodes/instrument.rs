use super::envelope::EnvelopePoint;
use crate::{
    callable::Callable,
    get_sample_rate,
    indexable::Indexable,
    lua_state::LuaState,
    node::{ContainsNode, FrequencyNode, Node},
};
use libdaw::{
    nodes::instrument,
    time::{Duration, Timestamp},
};
use mlua::{FromLua, IntoLua, Lua, UserData};
use std::rc::Rc;

#[derive(Debug)]
pub struct Tone(pub instrument::Tone);

impl std::ops::DerefMut for Tone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for Tone {
    type Target = instrument::Tone;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'lua> FromLua<'lua> for Tone {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let indexable = Indexable::from_lua(value, lua)?;
        let start =
            Timestamp::from_seconds(indexable.get("start")?).map_err(mlua::Error::external)?;
        let length =
            Duration::from_seconds(indexable.get("length")?).map_err(mlua::Error::external)?;
        let frequency = indexable.get("frequency")?;
        Ok(Tone(instrument::Tone {
            start,
            length,
            frequency,
        }))
    }
}

impl<'lua> IntoLua<'lua> for Tone {
    fn into_lua(self, lua: &'lua Lua) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("start", self.0.start.seconds())?;
        table.set("length", self.0.length.seconds())?;
        table.set("frequency", self.0.frequency)?;
        IntoLua::into_lua(table, lua)
    }
}

#[derive(Debug, Clone)]
pub struct Instrument {
    node: Rc<libdaw::nodes::Instrument>,
}

impl ContainsNode for Instrument {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Instrument {
    pub fn new(
        lua: &Lua,
        (frequency_node_creator, envelope): (Callable, Vec<EnvelopePoint>),
    ) -> mlua::Result<Self> {
        let callable_key = lua.create_registry_value(frequency_node_creator)?;
        let lua_state: LuaState = lua.named_registry_value("daw.lua_state")?;
        let function = move || {
            let Some(lua) = lua_state.state.upgrade() else {
                unreachable!("The graph should not be callable after Lua has been destructed");
            };
            let callable: Callable = lua
                .registry_value(&callable_key)
                .expect("set key should be a function");
            let node: FrequencyNode = callable
                .call(())
                .expect("function should return a frequencynode");
            node.node
        };
        let node = libdaw::nodes::Instrument::new(
            get_sample_rate(lua)?,
            function,
            envelope.into_iter().map(|point| *point),
        )
        .into();
        Ok(Self { node })
    }
}

impl UserData for Instrument {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
        methods.add_method("add_tone", |_, this, tone: Tone| {
            this.node.add_tone(*tone);
            Ok(())
        });
    }

    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_set("detune", |_, this, detune| {
            this.node.set_detune(detune).map_err(mlua::Error::external)
        });
    }
}
