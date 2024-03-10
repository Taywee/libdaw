use crate::indexable::Indexable;
use libdaw::stream::MAX_CHANNELS;
use mlua::{
    AnyUserDataExt as _, Error, FromLua, Function, IntoLua, Lua, Result, UserData, UserDataFields,
    UserDataMethods, Value,
};
use std::cell::Ref;
use std::rc::Rc;

pub trait ContainsNode {
    fn node(&self) -> Rc<dyn libdaw::Node>;
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node: Rc<dyn libdaw::Node>,
}

impl From<Rc<dyn libdaw::Node>> for Node {
    fn from(node: Rc<dyn libdaw::Node>) -> Self {
        Self { node }
    }
}

impl ContainsNode for Node {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Node {
    pub fn add_node_fields<'lua, T: UserData + ContainsNode, F: UserDataFields<'lua, T>>(
        _fields: &mut F,
    ) {
    }
    pub fn add_node_methods<'lua, T: UserData + ContainsNode, M: UserDataMethods<'lua, T>>(
        methods: &mut M,
    ) {
        methods.add_method("node", |_, this, ()| Ok(Node::from(this.node())));
        methods.add_method("process", |lua, this, inputs: Vec<Stream>| {
            let inputs: Vec<_> = inputs.into_iter().map(|stream| stream.0).collect();
            let mut outputs = Vec::with_capacity(8);
            this.node()
                .process(&inputs, &mut outputs)
                .map_err(mlua::Error::external)?;
            let output_table = lua.create_table_with_capacity(outputs.len(), 0)?;
            for (i, stream) in outputs.into_iter().enumerate() {
                output_table.raw_set(i + 1, Stream(stream))?;
            }
            Ok(output_table)
        });
    }
}

impl UserData for Node {
    fn add_fields<'lua, F>(fields: &mut F)
    where
        F: UserDataFields<'lua, Self>,
    {
        Node::add_node_fields(fields);
    }
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }
}

/// Check if the data is a Node, otherwise repeatedly call the "node" method
/// until we get one.
impl<'lua> FromLua<'lua> for Node {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        match value {
            Value::Table(table) => {
                let node_func: Function = table.get("node")?;
                Node::from_lua(node_func.call(table)?, lua)
            }
            Value::UserData(ud) => {
                if ud.is::<Self>() {
                    let node: Ref<Self> = ud.borrow()?;
                    Ok((*node).clone())
                } else {
                    let node_func: Function = ud.get("node")?;
                    Node::from_lua(node_func.call(ud)?, lua)
                }
            }
            _ => Err(Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Node",
                message: Some("A node function must be receieved from a table or userdata".into()),
            }),
        }
    }
}
pub trait ContainsFrequencyNode: ContainsNode {
    fn frequency_node(&self) -> Rc<dyn libdaw::FrequencyNode>;
}

#[derive(Debug, Clone)]
pub struct FrequencyNode {
    pub node: Rc<dyn libdaw::FrequencyNode>,
}

impl From<Rc<dyn libdaw::FrequencyNode>> for FrequencyNode {
    fn from(node: Rc<dyn libdaw::FrequencyNode>) -> Self {
        Self { node }
    }
}

impl ContainsNode for FrequencyNode {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone().node()
    }
}

impl ContainsFrequencyNode for FrequencyNode {
    fn frequency_node(&self) -> Rc<dyn libdaw::FrequencyNode> {
        self.node.clone()
    }
}

impl FrequencyNode {
    pub fn add_node_fields<
        'lua,
        T: UserData + ContainsFrequencyNode,
        F: UserDataFields<'lua, T>,
    >(
        fields: &mut F,
    ) {
        Node::add_node_fields(fields);
        fields.add_field_method_get("frequency", |_, this| {
            this.frequency_node()
                .get_frequency()
                .map_err(Error::external)
        });
        fields.add_field_method_set("frequency", |_, this, frequency| {
            this.frequency_node()
                .set_frequency(frequency)
                .map_err(Error::external)
        });
    }
    pub fn add_node_methods<
        'lua,
        T: UserData + ContainsFrequencyNode,
        M: UserDataMethods<'lua, T>,
    >(
        methods: &mut M,
    ) {
        Node::add_node_methods(methods);
        methods.add_method("frequency_node", |_, this, ()| {
            Ok(FrequencyNode::from(this.frequency_node()))
        });
    }
}

impl UserData for FrequencyNode {
    fn add_fields<'lua, F>(fields: &mut F)
    where
        F: UserDataFields<'lua, Self>,
    {
        FrequencyNode::add_node_fields(fields);
    }
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        FrequencyNode::add_node_methods(methods);
    }
}

/// Check if the data is a FrequencyNode, otherwise repeatedly call the "node" method
/// until we get one.
impl<'lua> FromLua<'lua> for FrequencyNode {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        match value {
            Value::Table(table) => {
                let node_func: Function = table.get("frequency_node")?;
                FrequencyNode::from_lua(node_func.call(table)?, lua)
            }
            Value::UserData(ud) => {
                if ud.is::<Self>() {
                    let node: Ref<Self> = ud.borrow()?;
                    Ok((*node).clone())
                } else {
                    let node_func: Function = ud.get("frequency_node")?;
                    FrequencyNode::from_lua(node_func.call(ud)?, lua)
                }
            }
            _ => Err(Error::FromLuaConversionError {
                from: value.type_name(),
                to: "FrequencyNode",
                message: Some(
                    "A frequency node function must be receieved from a table or userdata".into(),
                ),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Stream(pub libdaw::stream::Stream);

impl std::ops::DerefMut for Stream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for Stream {
    type Target = libdaw::Stream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'lua> IntoLua<'lua> for Stream {
    fn into_lua(self, lua: &'lua Lua) -> Result<Value<'lua>> {
        let table = lua.create_table_with_capacity(self.len(), 0)?;
        for (i, value) in self.0.into_iter().enumerate() {
            table.raw_set(i + 1, value)?;
        }
        Ok(Value::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Stream {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        let indexable = Indexable::from_lua(value, lua)?;
        let mut buffer = [0f64; MAX_CHANNELS];
        let mut channels = 0usize;
        for i in 0usize.. {
            let Some(value): Option<f64> = indexable.get(i + 1)? else {
                channels = i;
                break;
            };
            assert!(i < MAX_CHANNELS, "stream has too many channels");
            buffer[i] = value;
        }
        Ok(Stream(libdaw::stream::Stream::from_raw_parts(
            buffer, channels,
        )))
    }
}
