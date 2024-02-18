pub mod error;
mod nodes;

use error::Error;
use lua::{AnyUserDataExt as _, FromLua, Lua, Table, UserData};
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

#[derive(Debug, Clone)]
pub struct Track {
    _lua: Arc<Mutex<Lua>>,
    node: Node,
    sample: smallvec::IntoIter<[f64; 2]>,
}

impl Track {
    pub fn new() -> Result<Self, Error> {
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
        let chunk = lua.load(
            r#"
            local daw = require 'daw'
            local graph = daw.Graph()
            graph.sample_rate = 48000
            local function sawtooth(frequency)
                local constant = daw.ConstantValue(1 / 6)
                local node = daw.SawtoothOscillator()
                node.frequency = frequency
                local mul = daw.Multiply()
                graph:connect(node, mul)
                graph:connect(constant, mul)
                return mul
            end
            local function square(frequency)
                local constant = daw.ConstantValue(1 / 16)
                local node = daw.SquareOscillator()
                node.frequency = frequency
                local mul = daw.Multiply()
                graph:connect(node, mul)
                graph:connect(constant, mul)
                return mul
            end
            local add = daw.Add()
            graph:connect(sawtooth(256), add)
            graph:connect(sawtooth(256 * 2 ^ (4 / 12)), add)
            graph:connect(sawtooth(256 * 2 ^ (7 / 12)), add)
            graph:connect(square(256), add)
            graph:connect(square(256 * 2 ^ (4 / 12)), add)
            graph:connect(square(256 * 2 ^ (7 / 12)), add)
            graph:sink(add)
            return graph
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
