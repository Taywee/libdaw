use super::Item;
use crate::metronome::Metronome;
use crate::nodes::instrument::Tone;
use crate::pitch::PitchStandard;
use libdaw::metronome::Beat;
use mlua::FromLua;
use mlua::IntoLua;
use mlua::Lua;

#[derive(Debug)]
pub struct Section(pub libdaw::notation::absolute::Section);

impl Section {
    const METATABLE_ID: &'static str = "libdaw::notation::absolute::Section";

    fn resolve<'lua>(
        _: &'lua Lua,
        (section, metronome, pitch_standard, offset): (
            Section,
            Metronome,
            PitchStandard,
            Option<f64>,
        ),
    ) -> mlua::Result<Vec<Tone>> {
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
    }

    fn length<'lua>(_: &'lua Lua, section: Section) -> mlua::Result<f64> {
        Ok(section.0.length().get())
    }

    fn duration<'lua>(_: &'lua Lua, section: Section) -> mlua::Result<f64> {
        Ok(section.0.duration().get())
    }

    fn metatable<'lua>(lua: &'lua Lua) -> mlua::Result<mlua::Table<'lua>> {
        if let Some(section_metatable) = lua.named_registry_value(Self::METATABLE_ID)? {
            return Ok(section_metatable);
        }
        let section_metatable = lua.create_table()?;
        let section_methods = lua.create_table()?;
        section_methods.set("resolve", lua.create_function(Self::resolve)?)?;
        section_methods.set("length", lua.create_function(Self::length)?)?;
        section_methods.set("duration", lua.create_function(Self::duration)?)?;
        section_metatable.set("__index", section_methods)?;
        lua.set_named_registry_value(Self::METATABLE_ID, section_metatable.clone())?;
        Ok(section_metatable)
    }
}

impl<'lua> IntoLua<'lua> for Section {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let items: Vec<_> = self.0 .0.into_iter().map(Item).collect();
        let section = items.into_lua(lua)?;
        let mlua::Value::Table(section) = section else {
            unreachable!()
        };

        section.set_metatable(Some(Self::metatable(lua)?));
        Ok(mlua::Value::Table(section))
    }
}

impl<'lua> FromLua<'lua> for Section {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let mlua::Value::String(string) = value {
            let string = string.to_str()?;
            string.parse().map_err(mlua::Error::external).map(Self)
        } else {
            let items = <Vec<Item>>::from_lua(value, lua)?;
            Ok(Self(libdaw::notation::absolute::Section(
                items.into_iter().map(|item| item.0).collect(),
            )))
        }
    }
}
