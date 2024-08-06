mod baux;
mod chords;
mod data_sets;
mod notes;
mod scales;
mod voicings;

mod scales_generator;

pub use crate::baux::music_theory_baux;
pub use crate::chords::{ChordName, ChordQuality, ChordSpelling, ChordTone, ChordToneDegree};
pub use crate::notes::{NoteName, NoteOct};
pub use data_sets::generate_music_data;
pub use scales::{ScaleDegree, ScaleName, ScaleQuality, ScaleSpelling};
pub use voicings::{from_input, ChordVoicing};

pub use scales_generator::*;
