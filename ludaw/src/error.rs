use std::{
    fmt,
    sync::mpsc::{RecvError, SendError},
};

#[derive(Debug)]
pub enum Error {
    Lua(mlua::Error),
    AudioDisconnect,
    LibDAW(libdaw::Error),
}

impl From<libdaw::Error> for Error {
    fn from(v: libdaw::Error) -> Self {
        Self::LibDAW(v)
    }
}

impl From<mlua::Error> for Error {
    fn from(v: mlua::Error) -> Self {
        Self::Lua(v)
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Error::AudioDisconnect
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Error::AudioDisconnect
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lua(e) => write!(f, "lua error: {e}"),
            Error::AudioDisconnect => write!(f, "audio disconnected"),
            Error::LibDAW(e) => write!(f, "libdaw error: {e}"),
        }
    }
}

impl std::error::Error for Error {}
