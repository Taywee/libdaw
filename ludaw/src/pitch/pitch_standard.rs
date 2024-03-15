use super::pitch::Pitch;
use mlua::UserData;

#[derive(Debug)]
pub struct PitchStandard(pub Box<dyn libdaw::pitch::PitchStandard>);

impl UserData for PitchStandard {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("resolve", |_, this, note: Pitch| Ok(this.0.resolve(note.0)))
    }
}
