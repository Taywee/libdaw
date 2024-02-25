use crate::callable::Callable;
use crate::{error::Error, nodes};
use crate::{get_node, ConcreteNode as _};
use libdaw::stream::{IntoIter, Stream};
use lua::{IntoLua, Lua, Table};
use mlua as lua;
use nohash_hasher::IntSet;
use rodio::source::Source;
use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::time::Duration;

#[derive(Debug)]
struct Message {
    sample: Stream,
}

#[derive(Debug)]
pub struct Track {
    lua: Lua,
    sender: SyncSender<Message>,
    sample_number: u64,
    node: Rc<RefCell<dyn libdaw::Node>>,
    outputs: Vec<Stream>,
    before_sample_indexes: Rc<RefCell<IntSet<i64>>>,
}

#[derive(Debug)]
pub struct TrackSource {
    receiver: Receiver<Message>,
    sample: IntoIter,
}

impl Track {
    pub fn new<S, A1, A2>(source: S, args: A1) -> Result<(Track, TrackSource), Error>
    where
        S: AsRef<str>,
        A1: IntoIterator<Item = A2>,
        A2: AsRef<str>,
    {
        let before_sample_indexes: Rc<RefCell<IntSet<i64>>> = Default::default();
        let source = source.as_ref();
        let lua = Lua::new();
        lua.set_named_registry_value("daw.before_sample", lua.create_table()?)?;
        {
            let package: Table = lua.globals().get("package")?;
            let preload: Table = package.get("preload")?;
            preload.set(
                "daw.nodes",
                lua.create_function(|lua, ()| {
                    let module = lua.create_table()?;
                    module.set("Graph", lua.create_function(nodes::Graph::new)?)?;
                    module.set(
                        "SquareOscillator",
                        lua.create_function(nodes::SquareOscillator::new)?,
                    )?;
                    module.set(
                        "SawtoothOscillator",
                        lua.create_function(nodes::SawtoothOscillator::new)?,
                    )?;
                    module.set(
                        "ConstantValue",
                        lua.create_function(nodes::ConstantValue::new)?,
                    )?;
                    module.set("Add", lua.create_function(nodes::Add::new)?)?;
                    module.set("Multiply", lua.create_function(nodes::Multiply::new)?)?;
                    module.set("Delay", lua.create_function(nodes::Delay::new)?)?;
                    Ok(module)
                })?,
            )?;
            let before_sample_indexes = before_sample_indexes.clone();
            preload.set(
                "daw",
                lua.create_function(move |lua, ()| {
                    let module = lua.create_table()?;

                    let before_sample_indexes = before_sample_indexes.clone();
                    module.set(
                        "before_sample",
                        lua.create_function(move |lua, callable: Callable| {
                            let table: lua::Table =
                                lua.named_registry_value("daw.before_sample")?;
                            let index = table.len()? + 1;
                            table.set(index, callable)?;
                            before_sample_indexes.borrow_mut().insert(index);
                            Ok(index)
                        })?,
                    )?;

                    module.set(
                        "cancel_before_sample",
                        lua.create_function(move |lua, handle: i64| {
                            let table: lua::Table =
                                lua.named_registry_value("daw.before_sample")?;
                            table.set(handle, lua::Value::Nil)?;
                            Ok(())
                        })?,
                    )?;

                    Ok(module)
                })?,
            )?;
        }
        let chunk = lua.load(source);
        let mut arg_vec: Vec<_> = Vec::new();
        for arg in args {
            arg_vec.push(arg.as_ref().into_lua(&lua)?);
        }
        let node = get_node(chunk.call(lua::MultiValue::from_vec(arg_vec))?)?.node();
        {
            let mut node = node.borrow_mut();
            node.set_sample_rate(48000);
            node.set_channels(2);
        }
        let (sender, receiver) = sync_channel(48000);
        let track = Track {
            lua,
            sender,
            sample_number: 0,
            node,
            outputs: Default::default(),
            before_sample_indexes,
        };
        let track_source = TrackSource {
            receiver,
            // The initial sample is empty.
            sample: Stream::default().into_iter(),
        };
        Ok((track, track_source))
    }

    pub fn process(&mut self) -> Result<bool, Error> {
        let before_sample_table: lua::Table = self.lua.named_registry_value("daw.before_sample")?;
        let mut cancelled = IntSet::default();
        for i in self.before_sample_indexes.borrow().iter().copied() {
            let callable: Option<Callable> = before_sample_table.get(i)?;
            if let Some(callable) = callable {
                let () = callable.call(self.sample_number)?;
            } else {
                cancelled.insert(i);
            }
        }

        if !cancelled.is_empty() {
            let mut before_sample_indexes = self.before_sample_indexes.borrow_mut();
            for i in cancelled {
                before_sample_indexes.remove(&i);
            }
        }
        self.outputs.clear();
        self.node.borrow_mut().process(&[], &mut self.outputs);
        let sample = self
            .outputs
            .iter()
            .copied()
            .reduce(Add::add)
            .expect("No empty outputs are allowed yet");
        self.sender.send(Message { sample })?;
        self.sample_number += 1;
        Ok(true)
    }
}

impl TrackSource {
    fn refresh(&mut self) {
        if self.sample.len() == 0 {
            if let Ok(message) = self.receiver.recv() {
                self.sample = message.sample.into_iter();
            }
        }
    }
}

impl Source for TrackSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        // TODO: make configurable
        2
    }

    fn sample_rate(&self) -> u32 {
        // TODO: make configurable
        48000
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for TrackSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.refresh();
        self.sample.next().map(|sample| sample as f32)
    }
}
