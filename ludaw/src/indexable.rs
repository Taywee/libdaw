use mlua::{
    AnyUserData, AnyUserDataExt as _, Error, FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, Lua,
    Result, Table, TableExt as _, Value,
};

/// A wrapper type for any indexable, which may also be callable.
#[derive(Debug, Clone)]
pub enum Indexable<'lua> {
    UserData(AnyUserData<'lua>),
    Table(Table<'lua>),
}

impl<'lua> Indexable<'lua> {
    pub fn get<K, V>(&self, key: K) -> Result<V>
    where
        K: IntoLua<'lua>,
        V: FromLua<'lua>,
    {
        match self {
            Indexable::UserData(user_data) => user_data.get(key),
            Indexable::Table(table) => table.get(key),
        }
    }

    pub fn set<K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: IntoLua<'lua>,
        V: IntoLua<'lua>,
    {
        match self {
            Indexable::UserData(user_data) => user_data.set(key, value),
            Indexable::Table(table) => table.set(key, value),
        }
    }

    pub fn call<A, R>(&self, args: A) -> Result<R>
    where
        A: IntoLuaMulti<'lua>,
        R: FromLuaMulti<'lua>,
    {
        match self {
            Indexable::UserData(user_data) => user_data.call(args),
            Indexable::Table(table) => table.call(args),
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
