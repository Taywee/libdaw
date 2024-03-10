use crate::node::{ContainsNode, Node};
use libdaw::nodes::graph::Index;
use mlua::Lua;
use mlua::UserData;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Graph {
    pub node: Rc<libdaw::nodes::Graph>,
}

impl ContainsNode for Graph {
    fn node(&self) -> Rc<dyn libdaw::Node> {
        self.node.clone()
    }
}

impl Graph {
    pub fn new(_lua: &Lua, _: ()) -> mlua::Result<Self> {
        let node = libdaw::nodes::Graph::default().into();
        Ok(Self { node })
    }
}

impl UserData for Graph {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        Node::add_node_methods(methods);
        methods.add_method_mut("add", |_, this, node: Node| Ok(this.node.add(node.node).0));
        methods.add_method_mut("remove", |_, this, index| {
            Ok(this
                .node
                .remove(Index(index))
                .map_err(mlua::Error::external)?
                .map(Node::from))
        });
        methods.add_method_mut(
            "connect",
            |_, this, (source, destination, stream): (usize, usize, Option<usize>)| {
                this.node
                    .connect(Index(source), Index(destination), stream)
                    .map_err(mlua::Error::external)?;
                Ok(())
            },
        );
        methods.add_method_mut(
            "disconnect",
            |_, this, (source, destination, stream): (usize, usize, Option<usize>)| {
                this.node
                    .disconnect(Index(source), Index(destination), stream)
                    .map_err(mlua::Error::external)?;
                Ok(())
            },
        );
        methods.add_method_mut(
            "output",
            |_, this, (source, stream): (usize, Option<usize>)| {
                this.node
                    .output(Index(source), stream)
                    .map_err(mlua::Error::external)?;
                Ok(())
            },
        );
        methods.add_method_mut(
            "remove_output",
            |_, this, (source, stream): (usize, Option<usize>)| {
                this.node
                    .remove_output(Index(source), stream)
                    .map_err(mlua::Error::external)?;
                Ok(())
            },
        );
        methods.add_method_mut(
            "input",
            |_, this, (destination, stream): (usize, Option<usize>)| {
                this.node
                    .input(Index(destination), stream)
                    .map_err(mlua::Error::external)?;
                Ok(())
            },
        );
        methods.add_method_mut(
            "remove_input",
            |_, this, (destination, stream): (usize, Option<usize>)| {
                this.node
                    .remove_input(Index(destination), stream)
                    .map_err(mlua::Error::external)?;
                Ok(())
            },
        );
    }

    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        Node::add_node_fields(fields);
    }
}
