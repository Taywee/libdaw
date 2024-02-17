use std::fmt;

#[derive(Debug)]
pub enum Error {
    Lua(mlua::Error),
}

impl From<mlua::Error> for Error {
    fn from(v: mlua::Error) -> Self {
        Self::Lua(v)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lua(e) => write!(f, "lua error: {e}"),
        }
    }
}

impl std::error::Error for Error {}
