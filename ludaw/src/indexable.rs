use mlua::{
    AnyUserData, AnyUserDataExt as _, Error, FromLua, IntoLua, Lua, Result, Table, Value,
};

/// A wrapper type for any indexable.
#[derive(Debug, Clone)]
pub enum Indexable<'lua> {
    UserData(AnyUserData<'lua>),
    Table(Table<'lua>),
}

impl<'lua> Indexable<'lua> {
    pub fn get<K: IntoLua<'lua>, V: FromLua<'lua>>(&self, key: K) -> Result<V> {
        match self {
            Indexable::UserData(user_data) => user_data.get(key),
            Indexable::Table(table) => table.get(key),
        }
    }
}

impl<'lua> From<Table<'lua>> for Indexable<'lua> {
    fn from(v: Table<'lua>) -> Self {
        Self::Table(v)
    }
}

impl<'lua> From<AnyUserData<'lua>> for Indexable<'lua> {
    fn from(v: AnyUserData<'lua>) -> Self {
        Self::UserData(v)
    }
}

impl<'lua> TryFrom<Value<'lua>> for Indexable<'lua> {
    type Error = Error;

    fn try_from(value: Value<'lua>) -> Result<Self> {
        Ok(match value {
            Value::Table(table) => table.into(),
            Value::UserData(user_data) => user_data.into(),
            _ => {
                return Err(Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "Indexable",
                    message: None,
                })
            }
        })
    }
}

impl<'lua> From<Indexable<'lua>> for Value<'lua> {
    fn from(value: Indexable<'lua>) -> Self {
        match value {
            Indexable::UserData(user_data) => Value::UserData(user_data),
            Indexable::Table(table) => Value::Table(table),
        }
    }
}

impl<'lua> FromLua<'lua> for Indexable<'lua> {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> Result<Self> {
        value.try_into()
    }
}

impl<'lua> IntoLua<'lua> for Indexable<'lua> {
    fn into_lua(self, _lua: &'lua Lua) -> Result<Value<'lua>> {
        Ok(self.into())
    }
}
