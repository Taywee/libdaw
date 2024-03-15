use libdaw::pitch::PitchClass;
use mlua::FromLua;
use mlua::Lua;

use crate::indexable::Indexable;

#[derive(Debug)]
pub struct Pitch(pub libdaw::pitch::Pitch);

impl<'lua> FromLua<'lua> for Pitch {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(string) = value {
            let string = string.to_str()?;
            string.parse().map_err(mlua::Error::external).map(Self)
        } else {
            let indexable = Indexable::from_lua(value, lua)?;
            let class: String = indexable.get("class")?;
            let octave: i8 = indexable.get("octave")?;
            let adjustment: Option<f64> = indexable.get("adjustment")?;

            let class: PitchClass = class.parse().map_err(mlua::Error::external)?;

            Ok(Self(libdaw::pitch::Pitch {
                octave,
                class,
                adjustment: adjustment.unwrap_or(0.0),
            }))
        }
    }
}
