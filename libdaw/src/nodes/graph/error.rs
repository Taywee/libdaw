use super::Index;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    IllegalIndex {
        index: Index,
        message: &'static str,
    },
    NoSuchConnection {
        source: Index,
        destination: Index,
        stream: Option<usize>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IllegalIndex { index, message } => {
                write!(f, "illegal index {index:?}: {message}")
            }
            Error::NoSuchConnection {
                source,
                destination,
                stream,
            } => {
                write!(
                    f,
                    "Connection does not exist between {source:?} and {destination:?}"
                )?;
                match stream {
                    Some(stream) => write!(f, " for output {stream}"),
                    None => write!(f, " for all outputs"),
                }
            }
        }
    }
}

impl std::error::Error for Error {}
