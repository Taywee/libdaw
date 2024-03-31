mod chord;
mod item;
mod note;
mod overlapped;
mod rest;
mod section;

pub use chord::Chord;
pub use item::Item;

pub use note::Note;
pub use overlapped::Overlapped;
pub use rest::Rest;
pub use section::Section;

use mlua::Lua;

pub fn setup_module<'lua>(lua: &'lua Lua, _: ()) -> mlua::Result<mlua::Table<'lua>> {
    let module = lua.create_table()?;
    module.set(
        "Section",
        lua.create_function(|_, section: Section| Ok(section))?,
    )?;
    Ok(module)
}
