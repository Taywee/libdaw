mod item;
mod note;
mod overlapped;
mod rest;
mod section;

pub use item::Item;
use libdaw::metronome::Beat;
pub use note::Note;
pub use overlapped::Overlapped;
pub use rest::Rest;
pub use section::Section;

use mlua::Lua;

use crate::{metronome::Metronome, nodes::instrument::Tone, pitch::PitchStandard};

pub fn setup_module<'lua>(lua: &'lua Lua, _: ()) -> mlua::Result<mlua::Table<'lua>> {
    let module = lua.create_table()?;
    module.set(
        "Section",
        lua.create_function(|_, section: Section| Ok(section))?,
    )?;
    module.set(
        "resolve_section",
        lua.create_function(
            |_,
             (section, metronome, pitch_standard, offset): (
                Section,
                Metronome,
                PitchStandard,
                Option<f64>,
            )| {
                let offset = offset
                    .map(|beat| {
                        Beat::new(beat).ok_or_else(move || {
                            mlua::Error::external(format!("illegal beat value: {beat}"))
                        })
                    })
                    .unwrap_or(Ok(Beat::ZERO))?;
                let tones: Vec<_> = section
                    .0
                    .resolve(offset, &metronome.0.borrow(), &*pitch_standard.0)
                    .map(Tone)
                    .collect();

                Ok(tones)
            },
        )?,
    )?;
    Ok(module)
}
