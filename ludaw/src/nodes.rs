use super::Node;
use crate::get_node;
use libdaw::Node as _;
use lua::UserData;
use mlua as lua;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Graph(Rc<RefCell<libdaw::nodes::Graph>>);

impl UserData for Graph {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("sample_rate", |_, this| {
            Ok(this.0.borrow_mut().get_sample_rate())
        });
        fields.add_field_method_set("sample_rate", |_, this, sample_rate| {
            this.0.borrow_mut().set_sample_rate(sample_rate);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("node", |_, this, ()| Ok(Node::from(this.0.clone())));
        methods.add_method_mut(
            "connect",
            |_, this, (source, destination, output): (lua::Value, lua::Value, Option<usize>)| {
                let source = get_node(source)?;
                let destination = get_node(destination)?;
                source
                    .0
                    .borrow_mut()
                    .set_sample_rate(this.0.borrow_mut().get_sample_rate());
                destination
                    .0
                    .borrow_mut()
                    .set_sample_rate(this.0.borrow_mut().get_sample_rate());
                let output = output.unwrap_or(0);
                this.0.borrow_mut().connect(source.0, destination.0, output);
                Ok(())
            },
        );
        methods.add_method_mut(
            "sink",
            |_, this, (source, output): (lua::Value, Option<usize>)| {
                let source = get_node(source)?;
                source
                    .0
                    .borrow_mut()
                    .set_sample_rate(this.0.borrow_mut().get_sample_rate());
                let output = output.unwrap_or(0);
                this.0.borrow_mut().sink(source.0, output);
                Ok(())
            },
        );
    }
}

#[derive(Debug, Default, Clone)]
pub struct SquareOscillator(Rc<RefCell<libdaw::nodes::SquareOscillator>>);

impl UserData for SquareOscillator {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("frequency", |_, this| {
            Ok(this.0.borrow_mut().get_frequency())
        });
        fields.add_field_method_set("frequency", |_, this, frequency| {
            this.0.borrow_mut().set_frequency(frequency);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("node", |_, this, ()| Ok(Node::from(this.0.clone())));
    }
}

#[derive(Debug, Default, Clone)]
pub struct SawtoothOscillator(Rc<RefCell<libdaw::nodes::SawtoothOscillator>>);

impl UserData for SawtoothOscillator {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("frequency", |_, this| {
            Ok(this.0.borrow_mut().get_frequency())
        });
        fields.add_field_method_set("frequency", |_, this, frequency| {
            this.0.borrow_mut().set_frequency(frequency);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("node", |_, this, ()| Ok(Node::from(this.0.clone())));
    }
}

#[derive(Debug, Default, Clone)]
pub struct ConstantValue(Rc<RefCell<libdaw::nodes::ConstantValue>>);

impl ConstantValue {
    pub fn new(value: f64) -> Self {
        ConstantValue(Rc::new(RefCell::new(libdaw::nodes::ConstantValue::new(
            value,
        ))))
    }
}

impl UserData for ConstantValue {
    fn add_fields<'lua, F: lua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| Ok(this.0.borrow_mut().get_value()));
        fields.add_field_method_set("value", |_, this, value| {
            this.0.borrow_mut().set_value(value);
            Ok(())
        });
    }

    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("node", |_, this, ()| Ok(Node::from(this.0.clone())));
    }
}

#[derive(Debug, Default, Clone)]
pub struct Multiply(Rc<RefCell<libdaw::nodes::Multiply>>);

impl UserData for Multiply {
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("node", |_, this, ()| Ok(Node::from(this.0.clone())));
    }
}

#[derive(Debug, Default, Clone)]
pub struct Add(Rc<RefCell<libdaw::nodes::Add>>);

impl UserData for Add {
    fn add_methods<'lua, M: lua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("node", |_, this, ()| Ok(Node::from(this.0.clone())));
    }
}
