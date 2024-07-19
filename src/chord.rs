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

// may want to adjust to `write` chord symbols instead (maj, min, +, °)
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
pub struct VoiceTransitions {
    root: ChordToneTransition,
    second: Option<ChordToneTransition>,
    third: Option<ChordToneTransition>,
}

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
    weights: (usize, usize, usize),
}

impl Chord {
    // creates basic Cmaj instance (C3, E3, G3)
    pub fn default() -> Self {
        let (root, third, fifth) = (NoteOct::C3, NoteOct::E3, NoteOct::G3);

        let root_f = MusicTheoryBaux.get_freq(&root).unwrap();
        let third_f = MusicTheoryBaux.get_freq(&third).unwrap();
        let fifth_f = MusicTheoryBaux.get_freq(&fifth).unwrap();

        let tones = ChordTones {
            root: NoteOct::C3,
            third: Some(NoteOct::E3),
            fifth: Some(NoteOct::G3),
            second: None,
            fourth: None,
            seventh: None,
        };

        let root_w = MusicTheoryBaux.get_note_weight(&root).unwrap();
        let third_w = MusicTheoryBaux.get_note_weight(&third).unwrap();
        let fifth_w = MusicTheoryBaux.get_note_weight(&fifth).unwrap();

        Self {
            name: ChordName::Cmaj,
            root: NoteName::C,
            quality: ChordQuality::Maj,
            notes: Vec::from([NoteOct::C3, NoteOct::E3, NoteOct::G3]),
            frequencies: Vec::from([*root_f, *third_f, *fifth_f]),
            tones,
            voice_transitions: None,
            weights: (*root_w, *third_w, *fifth_w),
        }
    }

    pub fn new(name: &str) -> Self {
        let chord_name = ChordName::try_from(name).unwrap();
        let _spelling = MusicTheoryBaux.get_chord_spelling(&chord_name).unwrap();
        todo!()
    }

    pub fn from_chord(_prev_chord: &Chord) -> Self {
        todo!()
    }
}
