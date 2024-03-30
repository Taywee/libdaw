use crate::callable::Callable;
use crate::indexable::Indexable;
use crate::lua_state::LuaState;
use crate::metronome::Metronome;
use crate::node::{ContainsNode as _, Node};
use crate::{error::Error, nodes};
use crate::{get_channels, get_sample_rate, pitch};
use blake3::Hasher;
use libdaw::stream::{IntoIter, Stream};
use libdaw::time::Timestamp;
use mlua::{IntoLua, Lua, Table};
use nohash_hasher::IntSet;
use rand::{Rng as _, SeedableRng};
use rand_blake3::Rng;
use rodio::source::Source;
use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

#[derive(Debug)]
enum Message {
    Sample(Stream),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Callback {
    start_time: Timestamp,
    end_time: Timestamp,
    oneshot: bool,
    handle: i64,
}

#[derive(Debug)]
pub struct Track {
    sender: SyncSender<Message>,
    sample_number: u64,
    node: Rc<dyn libdaw::Node>,
    outputs: Vec<Stream>,
    sample_rate: u32,

    /// A sorted set of sample callbacks.  We use a vec instead of something
    /// like BTreeSet because iteration speed is much more important than
    /// insertion and deletion speed.
    callbacks: Rc<RefCell<Vec<Callback>>>,

    /// A copy of the current callbacks that are running, so callbacks can
    /// modify the callbacks table.
    running_callbacks: Vec<Callback>,

    /// A cached intset of callbacks that end per frame.
    ended_callbacks: IntSet<i64>,

    lua: Rc<Lua>,
}

#[derive(Debug)]
pub struct TrackSource {
    sample_rate: u32,
    channels: u16,
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
        let rng = Rc::new(RefCell::new(Rng::from_seed(
            *b"6QW4uIs34JAI5CvRjtug1cXkWWC06l89",
        )));
        let callbacks: Rc<RefCell<Vec<Callback>>> = Default::default();
        let source = source.as_ref();
        let lua = Rc::new(Lua::new());
        lua.set_named_registry_value(
            "daw.lua_state",
            LuaState {
                state: Rc::downgrade(&lua),
            },
        )?;
        lua.set_named_registry_value("daw.callbacks", lua.create_table()?)?;
        {
            let package: Table = lua.globals().get("package")?;
            let preload: Table = package.get("preload")?;
            preload.set(
                "daw",
                lua.create_function(move |lua, ()| {
                    let module = lua.create_table()?;
                    {
                        let rng = rng.clone();
                        module.set(
                            "random",
                            lua.create_function(move |_, (m, n): (Option<i64>, Option<i64>)| {
                                let mut rng = rng.borrow_mut();
                                match (m, n) {
                                    (None, None) => Ok(mlua::Value::Number(rng.gen())),
                                    (None, Some(_)) => Err(mlua::Error::external(
                                        "random may only be called with one or two integers",
                                    )),
                                    (Some(n), None) => {
                                        if n < 0 {
                                            Err(mlua::Error::external(
                                        "random may only be called with one or two integers",
                                            ))
                                        } else if n == 0 {
                                            Ok(mlua::Value::Integer(rng.gen()))
                                        } else {
                                            Ok(mlua::Value::Integer(rng.gen_range(1..=n)))
                                        }
                                    }
                                    (Some(m), Some(n)) => {
                                        if m < n {
                                            Err(mlua::Error::external("m must be greater than n"))
                                        } else {
                                            Ok(mlua::Value::Integer(rng.gen_range(m..=n)))
                                        }
                                    }
                                }
                            })?,
                        )?;
                    }

                    {
                        let rng = rng.clone();
                        module.set(
                            "randomseed",
                            lua.create_function(move |_, seed: mlua::Value| {
                                let mut hasher = Hasher::new();
                                match seed {
                                    mlua::Value::Nil => {
                                        hasher.update(&[0]);
                                    }
                                    mlua::Value::Boolean(b) => {
                                        hasher.update(&[1, b as u8]);
                                    }
                                    mlua::Value::LightUserData(_) => {
                                        return Err(mlua::Error::external(
                                            "Can not seed with light userdata",
                                        ));
                                    }
                                    mlua::Value::Integer(i) => {
                                        hasher.update(&[2]);
                                        hasher.update(&i.to_be_bytes());
                                    }
                                    mlua::Value::Number(f) => {
                                        hasher.update(&[3]);
                                        hasher.update(&f.to_be_bytes());
                                    }
                                    mlua::Value::String(s) => {
                                        hasher.update(&[4]);
                                        hasher.update(&s.as_bytes());
                                    }
                                    mlua::Value::Table(_) => {
                                        return Err(mlua::Error::external(
                                            "Can not seed with table",
                                        ));
                                    }
                                    mlua::Value::Function(_) => {
                                        return Err(mlua::Error::external(
                                            "Can not seed with function",
                                        ));
                                    }
                                    mlua::Value::Thread(_) => {
                                        return Err(mlua::Error::external(
                                            "Can not seed with thread",
                                        ));
                                    }
                                    mlua::Value::UserData(_) => {
                                        return Err(mlua::Error::external(
                                            "Can not seed with userdata",
                                        ));
                                    }
                                    mlua::Value::Error(_) => {
                                        return Err(mlua::Error::external(
                                            "Can not seed with error",
                                        ));
                                    }
                                };
                                *rng.borrow_mut() = hasher.into();
                                Ok(())
                            })?,
                        )?;
                    }

                    module.set(
                        "set_sample_rate",
                        lua.create_function(move |lua, sample_rate: u32| {
                            let value: mlua::Value = lua.named_registry_value("daw.sample_rate")?;
                            match value {
                                mlua::Value::Nil => (),
                                _ => {
                                    return Err(mlua::Error::external(
                                        "can not set_sample_rate twice",
                                    ));
                                }
                            }
                            lua.set_named_registry_value("daw.sample_rate", sample_rate)?;
                            Ok(())
                        })?,
                    )?;
                    module.set(
                        "sample_rate",
                        lua.create_function(move |lua, ()| get_sample_rate(lua))?,
                    )?;
                    module.set(
                        "set_channels",
                        lua.create_function(move |lua, channels: u16| {
                            let value: mlua::Value = lua.named_registry_value("daw.channels")?;
                            match value {
                                mlua::Value::Nil => (),
                                _ => {
                                    return Err(mlua::Error::external(
                                        "can not set_channels twice",
                                    ));
                                }
                            }
                            lua.set_named_registry_value("daw.channels", channels)?;
                            Ok(())
                        })?,
                    )?;
                    module.set(
                        "channels",
                        lua.create_function(move |lua, ()| get_channels(lua))?,
                    )?;
                    module.set("Metronome", lua.create_function(Metronome::new)?)?;

                    Ok(module)
                })?,
            )?;
            preload.set("daw.nodes", lua.create_function(nodes::setup_module)?)?;
            preload.set(
                "daw.notation.absolute",
                lua.create_function(crate::notation::absolute::setup_module)?,
            )?;
            preload.set("daw.pitch", lua.create_function(pitch::setup_module)?)?;
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
                                let table: mlua::Table =
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
                                let start_time = start_time
                                    .map(Timestamp::from_seconds)
                                    .transpose()
                                    .map_err(mlua::Error::external)?
                                    .unwrap_or(Timestamp::MIN);
                                let end_time: Option<f64> = registration.get("end_time")?;
                                let end_time = end_time
                                    .map(Timestamp::from_seconds)
                                    .transpose()
                                    .map_err(mlua::Error::external)?
                                    .unwrap_or(Timestamp::MAX);
                                let oneshot: Option<bool> = registration.get("oneshot")?;

                                table.set(handle, callback)?;
                                let callback = Callback {
                                    start_time,
                                    end_time,
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
                                let table: mlua::Table =
                                    lua.named_registry_value("daw.callbacks")?;
                                table.set(handle, mlua::Value::Nil)?;
                                let mut callbacks = callbacks.borrow_mut();
                                if let Some(index) = callbacks
                                    .iter()
                                    .enumerate()
                                    .find(|(_, callback)| callback.handle == handle)
                                    .map(|(i, _)| i)
                                {
                                    callbacks.remove(index);
                                }
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
        let node: Node = chunk.call(mlua::MultiValue::from_vec(arg_vec))?;
        let node = node.node();
        let sample_rate = get_sample_rate(&lua)?;
        let channels = get_channels(&lua)?;
        let (sender, receiver) = sync_channel(sample_rate as usize * 10);
        let track = Track {
            lua,
            sender,
            sample_number: 0,
            node,
            sample_rate,
            outputs: Default::default(),
            running_callbacks: Default::default(),
            ended_callbacks: Default::default(),
            callbacks,
        };
        let track_source = TrackSource {
            sample_rate,
            channels,
            receiver,
            // The initial sample is empty.
            sample: Stream::default().into_iter(),
        };
        Ok((track, track_source))
    }

    pub fn process(&mut self) -> Result<bool, Error> {
        let sample_time_float = self.sample_number as f64 / self.sample_rate as f64;
        let sample_time = Timestamp::from_seconds(sample_time_float)?;

        self.running_callbacks
            .extend(self.callbacks.borrow().iter().cloned());
        let sample_callback_table: mlua::Table = self.lua.named_registry_value("daw.callbacks")?;
        for callback in self.running_callbacks.drain(..) {
            if sample_time < callback.start_time {
                break;
            }
            if sample_time >= callback.end_time {
                self.ended_callbacks.insert(callback.handle);
                continue;
            }

            let callable: Callable = sample_callback_table.get(callback.handle)?;

            let () = callable.call(sample_time_float)?;

            if callback.oneshot {
                self.ended_callbacks.insert(callback.handle);
            }
        }

        if !self.ended_callbacks.is_empty() {
            self.callbacks
                .borrow_mut()
                .retain(|callback| !self.ended_callbacks.contains(&callback.handle));
            self.ended_callbacks.clear();
        }
        self.outputs.clear();
        self.node.process(&[], &mut self.outputs)?;
        self.lua.expire_registry_values();
        let lua = &self.lua;
        let sample = self.outputs.iter().copied().reduce(Add::add);

        let sample = sample.map_or_else(
            || get_channels(lua).map(|channels| Stream::new(channels as usize)),
            Ok,
        )?;
        self.sender.send(Message::Sample(sample))?;
        self.sample_number += 1;
        Ok(true)
    }
}

impl TrackSource {
    fn refresh(&mut self) {
        if self.sample.len() == 0 {
            if let Ok(Message::Sample(sample)) = self.receiver.recv() {
                self.sample = sample.into_iter();
            }
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
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
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
