//! Absolute pitch notation, for writing phrases of music using absolute
//! pitches, rather than relative or scale-based.
//!
//! ## Sections
//! A section is a run of notes, rests, chords, and overlapped sections.
//!
//! ### Examples
//! * `b4:1 c#4:2 [[b4:1 c#4:2] [b3:1 c#3:2]]`
//!
//! ## Notes
//! Each note is composed of fields separated by colons. The first field is the
//! absolute pitch, parsed the same way as absolute notes. The second field is
//! the note length, which is its default duration as well as the size of the
//! slot it takes up (influencing the start time of subsequent notes). The third
//! field is the duration, for overriding a duration to last longer than its
//! note length. If the note is left without a length, it will adopt the length
//! of the previous note, ignoring all overlapped sections. If there is no
//! previous note, the length is 1.
//!
//! ### Examples
//! * `b4:1`
//! * `c#4`
//! * `db:3:1`
//!
//! ## Rests
//! Rests are like notes, but with `r` in place of the absolute note.  A length
//! may be specified for r, but not a duration.

//! ### Examples
//! * `r:1`
//! * `r`
//!
//! ## Chords
//! A chord is an brace-surrounded list of absolute pitches, followed by a
//!length and duration, like a note.
//!
//! ### Examples
//!
//! * `{a#4 c#4 e#4}:3:4`
//!
//! ## Overlapped sections
//! Overlapped sections are specified as a bracket-surrounded list of
//! bracket-surrounded sections The first note in the section will be assigned
//! the previous_length of the previous note outside of the section.
//!
//! ### Examples
//!
//! * `[[b4:1 c#4:2] [b3:1 c#3:2]]`

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
