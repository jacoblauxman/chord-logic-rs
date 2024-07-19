// pub use crate::baux::MusicTheoryBaux;
// pub use crate::chord::Chord;
// pub use crate::notes::{NoteName, NoteOct};
// pub use crate::spellings::ChordName;
// // pub use crate::testing::*;

// mod baux;
// mod chord;
// mod data_sets;
// mod notes;
// mod spellings;
// mod testing;

mod baux;
mod chords;
mod data_sets;
mod notes;
mod scales;

pub use crate::baux::MusicTheoryBaux;
pub use crate::chords::{ChordName, ChordQuality, ChordSpelling, ChordTone};
pub use crate::notes::{NoteName, NoteOct};
pub use data_sets::generate_music_data;
