use libdaw::streams::Channels;
use libdaw::{ConstantValue, Graph, Multiply, Node, SquareOscillator};
use ludaw::Track;
use mlua::prelude::*;
use rodio::source::Source;
use rodio::{OutputStream, Sink};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.append(Track::new()?);
    sink.sleep_until_end();

    Ok(())
}
