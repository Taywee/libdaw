mod parse;

use super::{resolve_state::ResolveState, NotePitch};
use crate::{metronome::Beat, parse::IResult};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::{
    ops::{BitOr, BitOrAssign},
    str::FromStr,
};

#[derive(Debug, Clone, Default)]
pub struct Set {
    pub pitch: Option<NotePitch>,
    pub length: Option<Beat>,
}

impl BitOrAssign for Set {
    fn bitor_assign(&mut self, rhs: Self) {
        self.pitch = self.pitch.take().or(rhs.pitch);
        self.length = self.length.or(rhs.length);
    }
}
impl BitOr for Set {
    type Output = Set;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl Set {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::set(input)
    }
    pub(super) fn update_state(&self, state: &mut ResolveState) {
        if let Some(pitch) = &self.pitch {
            pitch.update_state(state);
        }
        if let Some(length) = self.length {
            state.length = length;
        }
    }
}

impl FromStr for Set {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scale = all_consuming(parse::set)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(scale)
    }
}
