use super::{Chord, Note, Overlapped, Rest};
use crate::indexable::Indexable;
use libdaw::notation::absolute;
use mlua::{FromLua, IntoLua, Lua};

#[derive(Debug)]
pub struct Item(pub absolute::Item);

impl<'lua> IntoLua<'lua> for Item {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let table = lua.create_table()?;
        match self.0 {
            absolute::Item::Note(note) => {
                table.set("note", Note(note))?;
            }
            absolute::Item::Chord(chord) => {
                table.set("chord", Chord(chord))?;
            }
            absolute::Item::Rest(rest) => {
                table.set("rest", Rest(rest))?;
            }
            absolute::Item::Overlapped(overlapped) => {
                table.set("overlapped", Overlapped(overlapped))?;
            }
        }
        Ok(mlua::Value::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Item {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(string) = value {
            let string = string.to_str()?;
            string.parse().map_err(mlua::Error::external).map(Self)
        } else {
            let value_type_name = value.type_name();
            let indexable = Indexable::from_lua(value, lua)?;
            let note: Option<Note> = indexable.get("note")?;
            if let Some(note) = note {
                return Ok(Self(absolute::Item::Note(note.0)));
            }
            let chord: Option<Chord> = indexable.get("chord")?;
            if let Some(chord) = chord {
                return Ok(Self(absolute::Item::Chord(chord.0)));
            }
            let rest: Option<Rest> = indexable.get("rest")?;
            if let Some(rest) = rest {
                return Ok(Self(absolute::Item::Rest(rest.0)));
            }
            let overlapped: Option<Overlapped> = indexable.get("overlapped")?;
            if let Some(overlapped) = overlapped {
                return Ok(Self(absolute::Item::Overlapped(overlapped.0)));
            }
            Err(mlua::Error::FromLuaConversionError { from: value_type_name, to: "item", message: Some(String::from("Item must be either a string to be parsed or a table with a 'note', 'rest', or 'overlapped' field")) })
        }
    }
}
