use mlua::FromLua;
use mlua::IntoLua;
use mlua::Lua;

use super::Section;

#[derive(Debug)]
pub struct Overlapped(pub libdaw::notation::absolute::Overlapped);

impl<'lua> IntoLua<'lua> for Overlapped {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let items: Vec<_> = self.0 .0.into_iter().map(Section).collect();
        items.into_lua(lua)
    }
}

impl<'lua> FromLua<'lua> for Overlapped {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(string) = value {
            let string = string.to_str()?;
            string.parse().map_err(mlua::Error::external).map(Self)
        } else {
            let items = <Vec<Section>>::from_lua(value, lua)?;
            Ok(Self(libdaw::notation::absolute::Overlapped(
                items.into_iter().map(|item| item.0).collect(),
            )))
        }
    }
}
