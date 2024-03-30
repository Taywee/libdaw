use mlua::FromLua;
use mlua::IntoLua;
use mlua::Lua;

use super::Item;

#[derive(Debug)]
pub struct Section(pub libdaw::notation::absolute::Section);

impl<'lua> IntoLua<'lua> for Section {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let items: Vec<_> = self.0 .0.into_iter().map(Item).collect();
        items.into_lua(lua)
    }
}

impl<'lua> FromLua<'lua> for Section {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(string) = value {
            let string = string.to_str()?;
            string.parse().map_err(mlua::Error::external).map(Self)
        } else {
            let items = <Vec<Item>>::from_lua(value, lua)?;
            Ok(Self(libdaw::notation::absolute::Section(
                items.into_iter().map(|item| item.0).collect(),
            )))
        }
    }
}
