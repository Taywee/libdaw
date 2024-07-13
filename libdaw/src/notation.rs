mod chord;
mod duration;
mod item;
mod mode;
mod note;
mod note_pitch;
mod overlapped;
mod pitch;
mod rest;
mod scale;
mod sequence;
mod set;
mod state_member;
mod step;
mod tone_generation_state;
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    pitch::PitchStandard,
};
use std::{
    fmt,
};

pub use chord::Chord;
pub use duration::Duration;
pub use item::{Item, ItemElement};
pub use mode::Mode;
pub use note::Note;
pub use note_pitch::NotePitch;
pub use overlapped::Overlapped;
pub use pitch::Pitch;
pub use rest::Rest;
pub use scale::Scale;
pub use sequence::Sequence;
pub use set::Set;
pub use state_member::StateMember;
pub use step::Step;
pub use tone_generation_state::ToneGenerationState;

pub trait Element: fmt::Debug + Send {
    /// Resolve all the section's notes to playable instrument tones.
    fn tones(
        &self,
        _offset: Beat,
        _metronome: &Metronome,
        _pitch_standard: &dyn PitchStandard,
        _state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static> {
        Box::new(std::iter::empty())
    }

    fn length(&self, _state: &ToneGenerationState) -> Beat {
        Beat::ZERO
    }

    fn update_state(&self, _state: &mut ToneGenerationState) {}

    fn duration(&self, _state: &ToneGenerationState) -> Beat {
        Beat::ZERO
    }
}
