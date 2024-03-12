use crate::indexable::Indexable;
use mlua::FromLua;
use mlua::Lua;
use mlua::UserData;

#[derive(Debug)]
struct TempoInstruction(libdaw::metronome::TempoInstruction);

impl<'lua> FromLua<'lua> for TempoInstruction {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let indexable = Indexable::from_lua(value, lua)?;
        let beat: f64 = indexable.get("beat")?;
        let beats_per_minute: f64 = indexable.get("beats_per_minute")?;
        Ok(TempoInstruction(libdaw::metronome::TempoInstruction {
            beat,
            beats_per_minute,
        }))
    }
}

#[derive(Debug)]
pub struct Metronome(libdaw::metronome::Metronome);

impl Metronome {
    pub fn new(_lua: &Lua, _: ()) -> mlua::Result<Self> {
        Ok(Metronome(Default::default()))
    }
}

impl UserData for Metronome {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "add_tempo_instruction",
            |_, this, tempo_instruction: TempoInstruction| {
                this.0
                    .add_tempo_instruction(tempo_instruction.0)
                    .map_err(mlua::Error::external)
            },
        );
        methods.add_method("beat_to_time", |_, this, beat: f64| {
            this.0
                .beat_to_time(beat)
                .map_err(mlua::Error::external)
                .map(|time| time.as_secs_f64())
        });
    }
}
