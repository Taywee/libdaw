use crate::indexable::Indexable;
use crate::pitch::Pitch;
use libdaw::metronome::Beat;
use mlua::FromLua;
use mlua::IntoLua;
use mlua::Lua;

#[derive(Debug)]
pub struct Note(pub libdaw::notation::absolute::Note);

impl<'lua> IntoLua<'lua> for Note {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let table = lua.create_table()?;
        table.set("pitch", Pitch(self.0.pitch))?;
        table.set("length", self.0.length.get())?;
        table.set("duration", self.0.duration.map(|beat| beat.get()))?;
        Ok(mlua::Value::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Note {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(string) = value {
            let string = string.to_str()?;
            string.parse().map_err(mlua::Error::external).map(Self)
        } else {
            let indexable = Indexable::from_lua(value, lua)?;
            let pitch: Pitch = indexable.get("pitch")?;
            let length = indexable.get("length")?;
            let length = Beat::new(length).ok_or_else(move || {
                mlua::Error::external(format!("illegal length value: {length}"))
            })?;
            let duration: Option<f64> = indexable.get("duration")?;
            let duration = duration
                .map(|duration| {
                    Beat::new(duration).ok_or_else(move || {
                        mlua::Error::external(format!("illegal duration value: {duration}"))
                    })
                })
                .transpose()?;
            Ok(Self(libdaw::notation::absolute::Note {
                pitch: pitch.0,
                length,
                duration,
            }))
        }
    }
}
