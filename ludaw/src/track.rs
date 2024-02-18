use crate::{error::Error, get_node, nodes, Node};
use lua::Lua;
use lua::Table;
use mlua as lua;
use rodio::source::Source;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct Message {
    pub(crate) sample: smallvec::IntoIter<[f64; 2]>,
}

#[derive(Debug)]
pub struct Track {
    pub(crate) _lua: Lua,
    pub(crate) node: Node,
    pub(crate) sender: SyncSender<Message>,
}

#[derive(Debug)]
pub struct TrackSource {
    pub(crate) receiver: Receiver<Message>,
    pub(crate) sample: smallvec::IntoIter<[f64; 2]>,
}

impl Track {
    pub fn new<S>(source: S) -> Result<(Track, TrackSource), Error>
    where
        S: AsRef<str>,
    {
        let source = source.as_ref();
        let lua = Lua::new();
        {
            let package: Table = lua.globals().get("package")?;
            let preload: Table = package.get("preload")?;
            preload.set(
                "daw",
                lua.create_function(|lua, ()| {
                    let daw = lua.create_table()?;
                    daw.set(
                        "SquareOscillator",
                        lua.create_function(|_, ()| Ok(nodes::SquareOscillator::default()))?,
                    )?;
                    daw.set(
                        "SawtoothOscillator",
                        lua.create_function(|_, ()| Ok(nodes::SawtoothOscillator::default()))?,
                    )?;
                    daw.set(
                        "Graph",
                        lua.create_function(|_, ()| Ok(nodes::Graph::default()))?,
                    )?;
                    daw.set(
                        "ConstantValue",
                        lua.create_function(|_, value| Ok(nodes::ConstantValue::new(value)))?,
                    )?;
                    daw.set(
                        "Add",
                        lua.create_function(|_, ()| Ok(nodes::Add::default()))?,
                    )?;
                    daw.set(
                        "Multiply",
                        lua.create_function(|_, ()| Ok(nodes::Multiply::default()))?,
                    )?;
                    Ok(daw)
                })?,
            )?;
        }
        let chunk = lua.load(source);
        let node: Node = get_node(chunk.call(())?)?;
        let sample = node
            .0
            .borrow_mut()
            .process(Default::default())
            .0
            .into_iter()
            .next()
            .unwrap()
            .0
            .into_iter();
        let (sender, receiver) = sync_channel(1024);
        Ok((
            Track {
                _lua: lua,
                sender,
                node,
            },
            TrackSource { receiver, sample },
        ))
    }

    pub fn process(&mut self) -> Result<(), Error> {
        let sample = self
            .node
            .0
            .borrow_mut()
            .process(Default::default())
            .0
            .into_iter()
            .next()
            .unwrap()
            .0
            .into_iter();
        self.sender.send(Message { sample })?;
        Ok(())
    }
}

impl Source for TrackSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.sample.len().try_into().expect("Too many channels")
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
        let next = self.sample.next().map(|sample| sample as f32);
        if self.sample.len() == 0 {
            self.sample = self.receiver.recv().unwrap().sample;
        }
        next
    }
}
