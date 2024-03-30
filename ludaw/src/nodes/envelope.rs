use crate::{
    get_sample_rate,
    indexable::Indexable,
    node::{ContainsNode, Node},
};
use libdaw::{
    nodes::envelope::{self, Offset},
    time::{Duration, Time},
};
use mlua::{FromLua, Lua, UserData};
use std::rc::Rc;

#[derive(Debug)]
pub struct EnvelopePoint(envelope::EnvelopePoint);

impl std::ops::DerefMut for EnvelopePoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for EnvelopePoint {
    type Target = envelope::EnvelopePoint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'lua> FromLua<'lua> for EnvelopePoint {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let indexable = Indexable::from_lua(value, lua)?;
        let volume: f64 = indexable.get("volume")?;
        let whence: f64 = indexable.get("whence")?;
        let offset: Option<Indexable> = indexable.get("offset")?;
        let (ratio_offset, time_offset) = match offset {
            Some(offset) => (offset.get("ratio")?, offset.get("time")?),
            None => (None, None),
        };
        let offset = match (ratio_offset, time_offset) {
            (None, None) => Offset::Time(Time::ZERO),
            (None, Some(time)) => {
                Offset::Time(Time::from_seconds(time).map_err(mlua::Error::external)?)
            }
            (Some(ratio), None) => Offset::Ratio(ratio),
            (Some(_), Some(_)) => {
                return Err(mlua::Error::external(
                    "only one of ratio_offset and time_offset must be set",
                ))
            }
        };

        Ok(EnvelopePoint(envelope::EnvelopePoint {
            offset,
            whence,
            volume,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct Envelope {
    node: Rc<libdaw::nodes::Envelope>,
}

impl ContainsNode for Envelope {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Envelope {
    pub fn new(lua: &Lua, (length, envelope): (f64, Vec<EnvelopePoint>)) -> mlua::Result<Self> {
        let length = Duration::from_seconds(length).map_err(mlua::Error::external)?;
        let node = libdaw::nodes::Envelope::new(
            get_sample_rate(lua)?,
            length,
            envelope.into_iter().map(|point| point.0),
        )
        .into();
        Ok(Self { node })
    }
}
impl UserData for Envelope {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }

    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }
}
