use crate::callable::Callable;
use crate::get_sample_rate;
use crate::indexable::Indexable;
use crate::lua_state::LuaState;
use crate::node::FrequencyNode;
use crate::node::{ContainsNode, Node};
use libdaw::nodes::instrument;
use mlua::Lua;

use mlua::UserData;
use mlua::{FromLua};

use std::rc::Rc;
use std::time::Duration;

use super::envelope::EnvelopePoint;

#[derive(Debug)]
pub struct Note(instrument::Note);

impl std::ops::DerefMut for Note {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for Note {
    type Target = instrument::Note;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'lua> FromLua<'lua> for Note {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let indexable = Indexable::from_lua(value, lua)?;
        let start = Duration::from_secs_f64(indexable.get("start")?);
        let length = Duration::from_secs_f64(indexable.get("length")?);
        let frequency = indexable.get("frequency")?;
        Ok(Note(instrument::Note {
            start,
            length,
            frequency,
        }))
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
        methods.add_method("add_note", |_, this, note: Note| {
            this.node.add_note(*note);
            Ok(())
        });
    }

    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_set("detune", |_, this, detune| {
            this.node.set_detune(detune);
            Ok(())
        });
    }
}
