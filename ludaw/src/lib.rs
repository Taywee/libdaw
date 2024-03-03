mod callable;
pub mod error;
mod indexable;
mod lua_state;
mod node;
mod nodes;
mod track;

use lua::Lua;
pub use track::{Track, TrackSource};

use mlua as lua;

// Get the sample rate if it exists, or set it to the default of 48000
pub fn get_sample_rate(lua: &Lua) -> lua::Result<u32> {
    if let Some(sample_rate) = lua.named_registry_value("daw.sample_rate")? {
        Ok(sample_rate)
    } else {
        lua.set_named_registry_value("daw.sample_rate", 48000u32)?;
        Ok(48000)
    }
}

// Get the channel count if it exists, or set it to the default of 2
pub fn get_channels(lua: &Lua) -> lua::Result<u16> {
    if let Some(channels) = lua.named_registry_value("daw.channels")? {
        Ok(channels)
    } else {
        lua.set_named_registry_value("daw.channels", 2u16)?;
        Ok(2)
    }
}
