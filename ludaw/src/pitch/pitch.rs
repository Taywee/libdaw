use crate::indexable::Indexable;
use mlua::FromLua;
use mlua::IntoLua;
use mlua::Lua;

#[derive(Debug)]
pub struct PitchClass(pub libdaw::pitch::PitchClass);

impl<'lua> FromLua<'lua> for PitchClass {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let string = String::from_lua(value, lua)?;
        let class: libdaw::pitch::PitchClass = string.parse().map_err(mlua::Error::external)?;
        Ok(Self(class))
    }
}

impl<'lua> IntoLua<'lua> for PitchClass {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let mut buf = 0u8;
        let name = self.0.name().encode_utf8(std::slice::from_mut(&mut buf));

        lua.create_string(name).map(mlua::Value::String)
    }
}

#[derive(Debug)]
pub struct Pitch(pub libdaw::pitch::Pitch);

impl<'lua> FromLua<'lua> for Pitch {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(string) = value {
            let string = string.to_str()?;
            string.parse().map_err(mlua::Error::external).map(Self)
        } else {
            let indexable = Indexable::from_lua(value, lua)?;
            let class: PitchClass = indexable.get("class")?;
            let octave: i8 = indexable.get("octave")?;
            let adjustment: Option<f64> = indexable.get("adjustment")?;

            Ok(Self(libdaw::pitch::Pitch {
                octave,
                class: class.0,
                adjustment: adjustment.unwrap_or(0.0),
            }))
        }
    }
}

impl<'lua> IntoLua<'lua> for Pitch {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let table = lua.create_table()?;
        table.set("class", PitchClass(self.0.class))?;
        table.set("octave", self.0.octave)?;
        table.set("adjustment", self.0.adjustment)?;
        Ok(mlua::Value::Table(table))
    }
}
