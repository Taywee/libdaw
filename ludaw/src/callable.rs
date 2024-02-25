use mlua::{
    AnyUserData, AnyUserDataExt as _, Error, FromLua, FromLuaMulti, Function, IntoLua,
    IntoLuaMulti, Lua, Result, Table, TableExt as _, Value,
};

/// A wrapper type for any callable.
#[derive(Debug, Clone)]
pub enum Callable<'lua> {
    UserData(AnyUserData<'lua>),
    Function(Function<'lua>),
    Table(Table<'lua>),
}

impl<'lua> Callable<'lua> {
    pub fn call<A: IntoLuaMulti<'lua>, R: FromLuaMulti<'lua>>(&self, args: A) -> Result<R> {
        match self {
            Callable::UserData(user_data) => user_data.call(args),
            Callable::Function(function) => function.call(args),
            Callable::Table(table) => table.call(args),
        }
    }
}

impl<'lua> From<Table<'lua>> for Callable<'lua> {
    fn from(v: Table<'lua>) -> Self {
        Self::Table(v)
    }
}

impl<'lua> From<Function<'lua>> for Callable<'lua> {
    fn from(v: Function<'lua>) -> Self {
        Self::Function(v)
    }
}

impl<'lua> From<AnyUserData<'lua>> for Callable<'lua> {
    fn from(v: AnyUserData<'lua>) -> Self {
        Self::UserData(v)
    }
}

impl<'lua> TryFrom<Value<'lua>> for Callable<'lua> {
    type Error = Error;

    fn try_from(value: Value<'lua>) -> Result<Self> {
        Ok(match value {
            Value::Table(table) => table.into(),
            Value::Function(function) => function.into(),
            Value::UserData(user_data) => user_data.into(),
            _ => {
                return Err(Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "Callable",
                    message: None,
                })
            }
        })
    }
}

impl<'lua> From<Callable<'lua>> for Value<'lua> {
    fn from(value: Callable<'lua>) -> Self {
        match value {
            Callable::UserData(user_data) => Value::UserData(user_data),
            Callable::Function(function) => Value::Function(function),
            Callable::Table(table) => Value::Table(table),
        }
    }
}

impl<'lua> FromLua<'lua> for Callable<'lua> {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> Result<Self> {
        value.try_into()
    }
}

impl<'lua> IntoLua<'lua> for Callable<'lua> {
    fn into_lua(self, _lua: &'lua Lua) -> Result<Value<'lua>> {
        Ok(self.into())
    }
}
