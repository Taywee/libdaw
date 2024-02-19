use crate::callable::Callable;
use crate::{error::Error, get_node, nodes, Node};
use lua::{IntoLua, Lua, Table};
use mlua as lua;
use rodio::source::Source;
use smallvec::{smallvec, IntoIter, SmallVec};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::time::Duration;

#[derive(Debug)]
struct Message {
    sample: SmallVec<[f64; 2]>,
}

#[derive(Debug)]
pub struct Track {
    lua: Lua,
    node: Node,
    sender: SyncSender<Message>,
    sample_number: u64,
}

#[derive(Debug)]
pub struct TrackSource {
    receiver: Receiver<Message>,
    channels: u16,
    sample: IntoIter<[f64; 2]>,
}

impl Track {
    pub fn new<S, A1, A2>(source: S, args: A1) -> Result<(Track, TrackSource), Error>
    where
        S: AsRef<str>,
        A1: IntoIterator<Item = A2>,
        A2: AsRef<str>,
    {
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
                    module.set(
                        "SquareOscillator",
                        lua.create_function(|_, ()| Ok(nodes::SquareOscillator::default()))?,
                    )?;
                    module.set(
                        "SawtoothOscillator",
                        lua.create_function(|_, ()| Ok(nodes::SawtoothOscillator::default()))?,
                    )?;
                    module.set(
                        "Graph",
                        lua.create_function(|_, ()| Ok(nodes::Graph::default()))?,
                    )?;
                    module.set(
                        "ConstantValue",
                        lua.create_function(|_, value| Ok(nodes::ConstantValue::new(value)))?,
                    )?;
                    module.set(
                        "Add",
                        lua.create_function(|_, ()| Ok(nodes::Add::default()))?,
                    )?;
                    module.set(
                        "Multiply",
                        lua.create_function(|_, ()| Ok(nodes::Multiply::default()))?,
                    )?;
                    Ok(module)
                })?,
            )?;
            preload.set(
                "daw",
                lua.create_function(|lua, ()| {
                    let module = lua.create_table()?;
                    module.set(
                        "before_sample",
                        lua.create_function(|lua, callable: Callable| {
                            let table: lua::Table =
                                lua.named_registry_value("daw.before_sample")?;
                            table.set(table.len()? + 1, callable)?;
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
        let node: Node = get_node(chunk.call(lua::MultiValue::from_vec(arg_vec))?)?;
        let (sender, receiver) = sync_channel(1024);
        let mut track = Track {
            lua,
            sender,
            node,
            sample_number: 0,
        };
        let mut track_source = TrackSource {
            receiver,
            channels: 0,
            sample: smallvec![].into_iter(),
        };
        track.process()?;
        track_source.refresh();
        Ok((track, track_source))
    }

    pub fn process(&mut self) -> Result<(), Error> {
        let before_sample_table: lua::Table = self.lua.named_registry_value("daw.before_sample")?;
        let len = before_sample_table.len()?;
        for i in 1..=len {
            let callable: Callable = before_sample_table.get(i)?;
            let () = callable.call(self.sample_number)?;
        }
        let sample = self
            .node
            .0
            .borrow_mut()
            .process(Default::default())
            .0
            .into_iter()
            .next()
            .unwrap()
            .0;
        self.sender.send(Message { sample })?;
        self.sample_number += 1;
        Ok(())
    }
}

impl TrackSource {
    fn refresh(&mut self) {
        if self.sample.len() == 0 {
            self.sample = self.receiver.recv().unwrap().sample.into_iter();
            self.channels = self.sample.len().try_into().expect("out of range");
        }
    }
}

impl Source for TrackSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
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
