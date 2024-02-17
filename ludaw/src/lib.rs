pub mod error;

use error::Error;

//use mlua::prelude::*;
use lua::{AnyUserDataExt as _, FromLua, Lua, UserData};
use mlua as lua;
use rodio::source::Source;

use std::cell::Ref;

use std::sync::{Arc, Mutex};
use std::time::Duration;

fn get_node<'lua>(value: lua::Value<'lua>) -> lua::Result<Node> {
    match value {
        lua::Value::Table(table) => {
            let node_func: lua::Function = table.get("node")?;
            node_func.call(table)
        }
        lua::Value::UserData(ud) => {
            let node_func: lua::Function = ud.get("node")?;
            node_func.call(ud)
        }
        _ => Err(lua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Node",
            message: Some("A node function must be receieved from a table or userdata".into()),
        }),
    }
}

#[derive(Debug, Clone)]
struct Node(Arc<Mutex<dyn libdaw::Node + Send>>);

impl<T> From<Arc<Mutex<T>>> for Node
where
    T: libdaw::Node + Send + 'static,
{
    fn from(value: Arc<Mutex<T>>) -> Self {
        Node(value.clone())
    }
}

impl UserData for Node {
    fn add_fields<'lua, F>(fields: &mut F)
    where
        F: lua::UserDataFields<'lua, Self>,
    {
        fields.add_field_method_set("sample_rate", |_, this, sample_rate| {
            this.0
                .lock()
                .expect("poisoned")
                .set_sample_rate(sample_rate);
            Ok(())
        });
    }
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // node:node() clones itself for convenience.
        methods.add_method("node", |_, this, ()| Ok(this.clone()));
    }
}

impl<'lua> FromLua<'lua> for Node {
    fn from_lua(value: lua::Value<'lua>, _lua: &'lua Lua) -> lua::Result<Self> {
        let type_name = value.type_name();
        let lua::Value::UserData(ud) = value else {
            return Err(lua::Error::FromLuaConversionError {
                from: type_name,
                to: "Node",
                message: None,
            });
        };
        if !ud.is::<Node>() {
            return Err(lua::Error::FromLuaConversionError {
                from: type_name,
                to: "Node",
                message: None,
            });
        }
        let node: Ref<Node> = ud.borrow()?;
        Ok(Node(node.0.clone()))
    }
}

#[derive(Debug, Default, Clone)]
struct SquareOscillator(Arc<Mutex<libdaw::SquareOscillator>>);

impl UserData for SquareOscillator {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("frequency", |_, this| {
            Ok(this.0.lock().expect("poisoned").get_frequency())
        });
        fields.add_field_method_set("frequency", |_, this, frequency| {
            this.0.lock().expect("poisoned").set_frequency(frequency);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("node", |_, this, ()| Ok(Node::from(this.0.clone())));
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    _lua: Arc<Mutex<Lua>>,
    node: Node,
    sample: smallvec::IntoIter<[f64; 2]>,
}

impl Track {
    pub fn new() -> Result<Self, Error> {
        let lua = Lua::new();
        lua.globals().set(
            "SquareOscillator",
            lua.create_function(|_, ()| Ok(SquareOscillator::default()))?,
        )?;

        let chunk = lua.load(
            r#"
            local oscillator = SquareOscillator()
            print(oscillator)
            return oscillator
        "#,
        );
        let node: Node = get_node(chunk.call(())?)?;
        let sample = node
            .0
            .lock()
            .expect("poisoned")
            .process(Default::default())
            .0
            .into_iter()
            .next()
            .unwrap()
            .0
            .into_iter();
        Ok(Track {
            _lua: Arc::new(Mutex::new(lua)),
            sample,
            node,
        })
    }
}

impl Source for Track {
    fn current_frame_len(&self) -> Option<usize> {
        Some(1)
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

impl Iterator for Track {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.sample.next().map(|sample| sample as f32);
        if self.sample.len() == 0 {
            self.sample = self
                .node
                .0
                .lock()
                .expect("poisoned")
                .process(Default::default())
                .0
                .into_iter()
                .next()
                .unwrap()
                .0
                .into_iter();
        }
        next
    }
}
