pub mod absolute;

use crate::pitch::Pitch;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Note {
    pub pitch: Pitch,

    // Start time in beats
    pub start: f64,

    // Length in beats
    pub length: f64,
}
