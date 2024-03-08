mod add;
mod constant_value;
mod delay;
mod detune;
mod envelope;
mod gain;
mod graph;
mod instrument;
mod multi_frequency;
mod multiply;
mod sawtooth_oscillator;
mod square_oscillator;

use lua::Lua;
use lua::Table;
use mlua as lua;

pub fn setup_module<'a>(lua: &'a Lua, _: ()) -> lua::Result<Table<'a>> {
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
    Ok(module)
}
