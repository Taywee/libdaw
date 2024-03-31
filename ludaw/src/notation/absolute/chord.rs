use crate::indexable::Indexable;
use crate::pitch::Pitch;
use libdaw::metronome::Beat;
use mlua::FromLua;
use mlua::IntoLua;
use mlua::Lua;

#[derive(Debug)]
pub struct Chord(pub libdaw::notation::absolute::Chord);

impl<'lua> IntoLua<'lua> for Chord {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let table = lua.create_table()?;
        let pitches: Vec<_> = self.0.pitches.into_iter().map(Pitch).collect();
        table.set("pitches", pitches)?;
        table.set("length", self.0.length.as_ref().map(Beat::get))?;
        table.set("duration", self.0.duration.map(|beat| beat.get()))?;
        Ok(mlua::Value::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Chord {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(string) = value {
            let string = string.to_str()?;
            string.parse().map_err(mlua::Error::external).map(Self)
        } else {
            let indexable = Indexable::from_lua(value, lua)?;
            let pitches: Vec<Pitch> = indexable.get("pitches")?;
            let length: Option<f64> = indexable.get("length")?;
            let length = length
                .map(|length| {
                    Beat::new(length).ok_or_else(move || {
                        mlua::Error::external(format!("illegal length value: {length}"))
                    })
                })
                .transpose()?;
            let duration: Option<f64> = indexable.get("duration")?;
            let duration = duration
                .map(|duration| {
                    Beat::new(duration).ok_or_else(move || {
                        mlua::Error::external(format!("illegal duration value: {duration}"))
                    })
                })
                .transpose()?;
            Ok(Self(libdaw::notation::absolute::Chord {
                pitches: pitches.into_iter().map(|pitch| pitch.0).collect(),
                length,
                duration,
            }))
        }
    }
}
