use std::{
    fmt,
    sync::{Arc, Mutex},
};

use crate::Node;

#[derive(Debug)]
pub enum Error {
    NoSuchConnection {
        source: Arc<Mutex<dyn Node>>,
        destination: Arc<Mutex<dyn Node>>,
        stream: Option<usize>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoSuchConnection {
                source,
                destination,
                stream,
            } => {
                let source = source.lock().expect("poisoned");
                let destination = destination.lock().expect("poisoned");
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
