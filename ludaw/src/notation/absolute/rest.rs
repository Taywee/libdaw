use crate::indexable::Indexable;

use libdaw::metronome::Beat;
use mlua::FromLua;
use mlua::IntoLua;
use mlua::Lua;

#[derive(Debug)]
pub struct Rest(pub libdaw::notation::absolute::Rest);

impl<'lua> IntoLua<'lua> for Rest {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let table = lua.create_table()?;
        table.set("length", self.0.length.get())?;
        Ok(mlua::Value::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Rest {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(string) = value {
            let string = string.to_str()?;
            string.parse().map_err(mlua::Error::external).map(Self)
        } else {
            let indexable = Indexable::from_lua(value, lua)?;
            let length = indexable.get("length")?;
            let length = Beat::new(length).ok_or_else(move || {
                mlua::Error::external(format!("illegal length value: {length}"))
            })?;
            Ok(Self(libdaw::notation::absolute::Rest { length }))
        }
    }
}
