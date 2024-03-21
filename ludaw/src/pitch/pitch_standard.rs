use super::pitch::Pitch;
use mlua::{AnyUserData, FromLua, Lua, UserData};
use std::{cell::Ref, rc::Rc};

#[derive(Debug, Clone)]
pub struct PitchStandard(pub Rc<dyn libdaw::pitch::PitchStandard>);

impl UserData for PitchStandard {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("resolve", |_, this, note: Pitch| Ok(this.0.resolve(note.0)))
    }
}
impl<'lua> FromLua<'lua> for PitchStandard {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let userdata = AnyUserData::from_lua(value, lua)?;
        let userdata: Ref<Self> = userdata.borrow()?;
        Ok((*userdata).clone())
    }
}
