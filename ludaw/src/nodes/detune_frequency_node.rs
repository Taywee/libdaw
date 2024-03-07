use crate::get_sample_rate;
use crate::indexable::Indexable;
use crate::lua_state::LuaState;
use crate::node::FrequencyNode;
use crate::node::{ContainsNode, Node};
use libdaw::nodes::{envelope_node, instrument};
use lua::FromLua;
use lua::Lua;
use lua::Table;
use lua::UserData;
use mlua as lua;
use std::rc::Rc;
use std::time::Duration;
#[derive(Debug, Clone)]

pub struct DetuneFrequencyNode {
    node: Rc<libdaw::nodes::DetuneFrequencyNode>,
}

impl ContainsNode for DetuneFrequencyNode {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}
// TODO: everything
