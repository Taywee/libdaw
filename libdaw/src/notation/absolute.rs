//! Absolute pitch notation, for writing phrases of music using absolute
//! pitches, rather than relative or scale-based. Looks something like: `b4:1
//! c#4:2 {{b4:1 c#4:2} {b3:1 c#3:2}}` Each note is composed of fields separated
//! by colons. The first field is the absolute pitch, parsed the same way as
//! absolute notes. The second field is the note length, which is its default
//! duration as well as the size of the slot it takes up (influencing the start
//! time of subsequent notes). The third field is the duration, for overriding a
//! duration to last longer than its note length. You can also specfy rests with
//! r in place of the absolute note.  A length may be specified for r, but not
//! a duration. Also present is overlapped sections; multiple runs of notes that
//! run simultaneously to one another.  These are all enclosed in curly braces

mod item;
mod note;
mod overlapped;
mod rest;
mod section;

pub use item::Item;
pub use note::Note;
pub use overlapped::Overlapped;
pub use rest::Rest;
pub use section::Section;
