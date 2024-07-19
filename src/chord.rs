use crate::{ChordName, MusicTheoryBaux, NoteName, NoteOct};
use std::fmt::{Display, Formatter};

// === CHORD STRUCT + RELATED === //

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ChordQuality {
    Maj,
    Min,
    Aug,
    Dim,
}

impl Display for ChordQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChordQuality::Maj => write!(f, "Major"),
            ChordQuality::Min => write!(f, "Minor"),
            ChordQuality::Aug => write!(f, "Augmented"),
            ChordQuality::Dim => write!(f, "Diminished"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChordTones {
    pub root: NoteOct,
    pub second: Option<NoteOct>,
    pub third: Option<NoteOct>,
    pub fourth: Option<NoteOct>,
    pub fifth: Option<NoteOct>,
    pub seventh: Option<NoteOct>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChordToneTransition(NoteOct, NoteOct);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Chord {
    name: ChordName,
    root: NoteName,
    quality: ChordQuality,
    notes: Vec<NoteOct>,
    frequencies: Vec<f64>,
    tones: ChordTones,
    voice_transitions: Option<Vec<ChordToneTransition>>,
}

impl Chord {
    pub fn default() -> Self {
        let root_f = MusicTheoryBaux.get_freq(&NoteOct::C3).unwrap();
        let third_f = MusicTheoryBaux.get_freq(&NoteOct::E3).unwrap();
        let fifth_f = MusicTheoryBaux.get_freq(&NoteOct::G3).unwrap();

        let tones = ChordTones {
            root: NoteOct::C3,
            third: Some(NoteOct::E3),
            fifth: Some(NoteOct::G3),
            second: None,
            fourth: None,
            seventh: None,
        };

        Self {
            name: ChordName::Cmaj,
            root: NoteName::C,
            quality: ChordQuality::Maj,
            notes: Vec::from([NoteOct::C3, NoteOct::E3, NoteOct::G3]),
            frequencies: Vec::from([*root_f, *third_f, *fifth_f]),
            tones,
            voice_transitions: None,
        }
    }
}
