use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::indexable::Indexable;
use libdaw::metronome::{Beat, BeatsPerMinute};
use mlua::{AnyUserData, FromLua, Lua, UserData};

#[derive(Debug)]
struct TempoInstruction(libdaw::metronome::TempoInstruction);

impl<'lua> FromLua<'lua> for TempoInstruction {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let indexable = Indexable::from_lua(value, lua)?;
        let beat = indexable.get("beat")?;
        let beat = Beat::new(beat)
            .ok_or_else(move || mlua::Error::external(format!("illegal beat value: {beat}")))?;
        let tempo: f64 = indexable.get("tempo")?;
        let tempo = BeatsPerMinute::new(tempo)
            .ok_or_else(move || mlua::Error::external(format!("illegal tempo value: {tempo}")))?;
        Ok(TempoInstruction(libdaw::metronome::TempoInstruction {
            beat,
            tempo,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct Metronome(pub Rc<RefCell<libdaw::metronome::Metronome>>);

impl Metronome {
    pub fn new(_lua: &Lua, _: ()) -> mlua::Result<Self> {
        Ok(Metronome(Default::default()))
    }
}

impl UserData for Metronome {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(
            "add_tempo_instruction",
            |_, this, tempo_instruction: TempoInstruction| {
                this.0
                    .borrow_mut()
                    .add_tempo_instruction(tempo_instruction.0);
                Ok(())
            },
        );
        methods.add_method("beat_to_time", |_, this, beat| {
            let beat = Beat::new(beat)
                .ok_or_else(move || mlua::Error::external(format!("illegal beat value: {beat}")))?;
            Ok(this.0.borrow().beat_to_time(beat).seconds())
        });
    }
}

impl<'lua> FromLua<'lua> for Metronome {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let userdata = AnyUserData::from_lua(value, lua)?;
        let userdata: Ref<Self> = userdata.borrow()?;
        Ok((*userdata).clone())
    }
}
