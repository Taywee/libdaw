use crate::callable::Callable;
use crate::indexable::Indexable;
use crate::ContainsNode as _;
use crate::Node;
use crate::{error::Error, nodes};
use libdaw::stream::{IntoIter, Stream};
use lua::{IntoLua, Lua, Table};
use mlua as lua;
use nohash_hasher::IntSet;
use ordered_float::OrderedFloat;
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Callback {
    start_time: OrderedFloat<f64>,
    end_time: OrderedFloat<f64>,
    oneshot: bool,
    handle: i64,
}

#[derive(Debug)]
pub struct Track {
    lua: Lua,
    sender: SyncSender<Message>,
    sample_number: u64,
    node: Rc<dyn libdaw::Node>,
    outputs: Vec<Stream>,
    sample_rate: u32,

    /// A sorted set of sample callbacks.  We use a vec instead of something
    /// like BTreeSet because iteration speed is much more important than
    /// insertion and deletion speed.
    callbacks: Rc<RefCell<Vec<Callback>>>,
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
        let callbacks: Rc<RefCell<Vec<Callback>>> = Default::default();
        let source = source.as_ref();
        let lua = Lua::new();
        lua.set_named_registry_value("daw.callbacks", lua.create_table()?)?;
        {
            let package: Table = lua.globals().get("package")?;
            let preload: Table = package.get("preload")?;
            preload.set("daw.nodes", lua.create_function(nodes::setup_module)?)?;
            let callbacks = callbacks.clone();
            preload.set(
                "daw.callbacks",
                lua.create_function(move |lua, ()| {
                    let module = lua.create_table()?;

                    {
                        let callbacks = callbacks.clone();
                        module.set(
                            "register",
                            lua.create_function(move |lua, registration: Indexable| {
                                let table: lua::Table =
                                    lua.named_registry_value("daw.callbacks")?;
                                let handle = table.len()? + 1;
                                let mut callbacks = callbacks.borrow_mut();
                                debug_assert_eq!(
                                    callbacks.iter().find(|callback| callback.handle == handle),
                                    None,
                                    "Should not find a matching handle"
                                );
                                let callback: Callable = registration.get("callback")?;
                                let start_time: Option<f64> = registration.get("start_time")?;
                                let end_time: Option<f64> = registration.get("end_time")?;
                                let oneshot: Option<bool> = registration.get("oneshot")?;

                                table.set(handle, callback)?;
                                let callback = Callback {
                                    start_time: start_time.unwrap_or(f64::NEG_INFINITY).into(),
                                    end_time: end_time.unwrap_or(f64::INFINITY).into(),
                                    handle,
                                    oneshot: oneshot.unwrap_or(false),
                                };
                                let index = callbacks.binary_search(&callback).expect_err(
                                    "Should never find an index that still exists in callbacks",
                                );
                                callbacks.insert(index, callback);
                                Ok(handle)
                            })?,
                        )?;
                    }

                    {
                        let callbacks = callbacks.clone();
                        module.set(
                            "cancel",
                            lua.create_function(move |lua, handle: i64| {
                                let table: lua::Table =
                                    lua.named_registry_value("daw.callbacks")?;
                                table.set(handle, lua::Value::Nil)?;
                                let mut callbacks = callbacks.borrow_mut();
                                callbacks.retain(|e| e.handle != handle);
                                Ok(())
                            })?,
                        )?;
                    }

                    Ok(module)
                })?,
            )?;
        }
        let chunk = lua.load(source);
        let mut arg_vec: Vec<_> = Vec::new();
        for arg in args {
            arg_vec.push(arg.as_ref().into_lua(&lua)?);
        }
        let node: Node = chunk.call(lua::MultiValue::from_vec(arg_vec))?;
        let node = node.node();
        node.set_sample_rate(48000);
        node.set_channels(2);
        let (sender, receiver) = sync_channel(480000);
        let track = Track {
            lua,
            sender,
            sample_number: 0,
            node,
            sample_rate: 48000,
            outputs: Default::default(),
            callbacks,
        };
        let track_source = TrackSource {
            receiver,
            // The initial sample is empty.
            sample: Stream::default().into_iter(),
        };
        Ok((track, track_source))
    }

    pub fn process(&mut self) -> Result<bool, Error> {
        let sample_time = OrderedFloat(self.sample_number as f64 / self.sample_rate as f64);

        let sample_callback_table: lua::Table = self.lua.named_registry_value("daw.callbacks")?;
        let mut ended = IntSet::default();
        for callback in self.callbacks.borrow().iter() {
            if sample_time < callback.start_time {
                break;
            }
            if sample_time >= callback.end_time {
                ended.insert(callback.handle);
                continue;
            }

            let callable: Callable = sample_callback_table.get(callback.handle)?;
            let () = callable.call(sample_time.0)?;

            if callback.oneshot {
                ended.insert(callback.handle);
            }
        }

        if !ended.is_empty() {
            let mut callbacks = self.callbacks.borrow_mut();
            dbg!(&ended);
            callbacks.retain(|callback| !ended.contains(&callback.handle));
        }

        self.outputs.clear();
        self.node.process(&[], &mut self.outputs);
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
