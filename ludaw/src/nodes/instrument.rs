use crate::get_sample_rate;
use crate::indexable::Indexable;
use crate::lua_state::LuaState;
use crate::node::FrequencyNode;
use crate::node::{ContainsNode, Node};
use libdaw::nodes::instrument;
use lua::FromLua;
use lua::Lua;
use lua::Table;
use lua::UserData;
use mlua as lua;
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
    fn from_lua(value: lua::Value<'lua>, lua: &'lua Lua) -> lua::Result<Self> {
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
        (frequency_node_creator, envelope): (lua::Function, Vec<EnvelopePoint>),
    ) -> lua::Result<Self> {
        let function_pointer = frequency_node_creator.to_pointer() as i64;
        {
            let function_table: Option<Table> =
                lua.named_registry_value("daw.instrument_functions")?;
            let function_table = match function_table {
                Some(table) => table,
                None => {
                    let table = lua.create_table()?;
                    lua.set_named_registry_value("daw.instrument_functions", table.clone())?;
                    table
                }
            };
            function_table.raw_set(function_pointer, frequency_node_creator)?;
        }
        let lua_state: LuaState = lua.named_registry_value("daw.lua_state")?;
        let function = move || {
            let Some(lua) = lua_state.state.upgrade() else {
                unreachable!("The graph should not be callable after Lua has been destructed");
            };
            let function_table: Table = lua
                .named_registry_value("daw.instrument_functions")
                .expect("instrument functions should exist as a table");
            let function: lua::Function = function_table
                .raw_get(function_pointer)
                .expect("function should be set as a function");
            let node: FrequencyNode = function
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
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
        methods.add_method("add_note", |_, this, note: Note| {
            this.node.add_note(*note);
            Ok(())
        });
    }

    fn add_fields<'lua, F: lua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
        fields.add_field_method_set("detune", |_, this, detune| {
            this.node.set_detune(detune);
            Ok(())
        });
    }
}
