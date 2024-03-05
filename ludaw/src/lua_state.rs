use mlua::{AnyUserData, FromLua, Lua, Result, UserData, Value};
use std::{cell::Ref, rc::Weak};

/// A state wrapper for keeping the lua state in the registry, so it can be used
/// internally for callback functions during processing that need access to a
/// lua state.
#[derive(Debug, Clone)]
pub struct LuaState {
    pub state: Weak<Lua>,
}

impl UserData for LuaState {}

impl<'lua> FromLua<'lua> for LuaState {
    fn from_lua(_value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        let ud: AnyUserData = lua.named_registry_value("daw.lua_state")?;
        let weak: Ref<'_, LuaState> = ud.borrow()?;
        Ok((*weak).clone())
    }
}
