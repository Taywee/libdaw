use mlua::Lua;

mod pitch;
mod pitch_standard;

pub fn setup_module<'lua>(lua: &'lua Lua, _: ()) -> mlua::Result<mlua::Table<'lua>> {
    let module = lua.create_table()?;
    module.set(
        "A440",
        lua.create_function(|_, ()| {
            Ok(pitch_standard::PitchStandard(Box::new(libdaw::pitch::A440)))
        })?,
    )?;
    module.set(
        "ScientificPitch",
        lua.create_function(|_, ()| {
            Ok(pitch_standard::PitchStandard(Box::new(
                libdaw::pitch::ScientificPitch,
            )))
        })?,
    )?;
    Ok(module)
}
