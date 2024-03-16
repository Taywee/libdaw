mod add;
mod constant_value;
mod custom;
mod custom_frequency;
mod delay;
mod detune;
mod envelope;
mod gain;
mod graph;
mod instrument;
mod multi_frequency;
mod multiply;
mod sawtooth_oscillator;
mod sine_oscillator;
mod square_oscillator;
mod triangle_oscillator;

use mlua::Lua;
use mlua::Table;

pub fn setup_module<'lua>(lua: &'lua Lua, _: ()) -> mlua::Result<Table<'lua>> {
    let module = lua.create_table()?;
    module.set("Graph", lua.create_function(graph::Graph::new)?)?;
    module.set(
        "SquareOscillator",
        lua.create_function(square_oscillator::SquareOscillator::new)?,
    )?;
    module.set(
        "SawtoothOscillator",
        lua.create_function(sawtooth_oscillator::SawtoothOscillator::new)?,
    )?;
    module.set(
        "TriangleOscillator",
        lua.create_function(triangle_oscillator::TriangleOscillator::new)?,
    )?;
    module.set(
        "SineOscillator",
        lua.create_function(sine_oscillator::SineOscillator::new)?,
    )?;
    module.set(
        "ConstantValue",
        lua.create_function(constant_value::ConstantValue::new)?,
    )?;
    module.set("Add", lua.create_function(add::Add::new)?)?;
    module.set("Multiply", lua.create_function(multiply::Multiply::new)?)?;
    module.set("Delay", lua.create_function(delay::Delay::new)?)?;
    module.set("Envelope", lua.create_function(envelope::Envelope::new)?)?;
    module.set("Gain", lua.create_function(gain::Gain::new)?)?;
    module.set(
        "Instrument",
        lua.create_function(instrument::Instrument::new)?,
    )?;
    module.set("Detune", lua.create_function(detune::Detune::new)?)?;
    module.set(
        "MultiFrequency",
        lua.create_function(multi_frequency::MultiFrequency::new)?,
    )?;
    module.set("Custom", lua.create_function(custom::Custom::new)?)?;
    module.set(
        "CustomFrequency",
        lua.create_function(custom_frequency::CustomFrequency::new)?,
    )?;
    Ok(module)
}
