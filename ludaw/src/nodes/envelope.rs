use crate::get_sample_rate;
use crate::indexable::Indexable;
use crate::lua_state::LuaState;
use crate::node::FrequencyNode;
use crate::node::{ContainsNode, Node};
use libdaw::nodes::{envelope, instrument};
use lua::FromLua;
use lua::Lua;
use lua::Table;
use lua::UserData;
use mlua as lua;
use std::rc::Rc;
use std::time::Duration;

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
    fn from_lua(value: lua::Value<'lua>, lua: &'lua Lua) -> lua::Result<Self> {
        let indexable = Indexable::from_lua(value, lua)?;
        let volume: f64 = indexable.get("volume")?;
        let whence: f64 = indexable.get("whence")?;
        let ratio_offset: Option<f64> = indexable.get("ratio_offset")?;
        let time_offset: Option<f64> = indexable.get("time_offset")?;
        if ratio_offset.is_some() && time_offset.is_some() {
            return Err(lua::Error::external(
                "only one of ratio_offset and time_offset must be set",
            ));
        }

        let ratio_offset = ratio_offset.map(envelope::Offset::Ratio);
        let time_offset = time_offset.map(|offset| {
            if offset >= 0.0 {
                envelope::Offset::TimeForward(Duration::from_secs_f64(offset))
            } else {
                envelope::Offset::TimeBackward(Duration::from_secs_f64(-offset))
            }
        });

        let offset = ratio_offset
            .or(time_offset)
            .unwrap_or_else(|| envelope::Offset::TimeForward(Duration::ZERO));
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
    pub fn new(lua: &Lua, (length, envelope): (f64, Vec<EnvelopePoint>)) -> lua::Result<Self> {
        let length = Duration::from_secs_f64(length);
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
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
    }

    fn add_fields<'lua, F: lua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }
}
