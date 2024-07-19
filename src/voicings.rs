use crate::{music_theory_baux, ChordName, ChordQuality, ChordTone, ChordToneDegree, NoteOct};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum ChordVoicingError {
    #[error("ERROR: failed to parse provided `chord_name` {0} (Note: should consist of `note letters` [`C`, `C#`, `Db, `C#/Db`] + symbols  [`maj`, `min`, `+`, `7sus4`]")]
    ParseChordName(String),
    #[error("ERROR: invalid octave `{0}` provided for {0} (octave ranges are from `1` to `8`)")]
    InvalidOct(usize, String),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordVoice {
    Root(NoteOct),
    Second(NoteOct),
    Third(NoteOct),
    Fourth(NoteOct),
    Fifth(NoteOct),
    Seventh(NoteOct),
}

#[allow(dead_code)]
impl ChordVoice {
    fn from_parts(note: &NoteOct, chord_tone: &ChordTone) -> ChordVoice {
        match chord_tone.get_tone_degree() {
            ChordToneDegree::Root => ChordVoice::Root(*note),
            ChordToneDegree::Second => ChordVoice::Second(*note),
            ChordToneDegree::Third => ChordVoice::Third(*note),
            ChordToneDegree::Fourth => ChordVoice::Fourth(*note),
            ChordToneDegree::Fifth => ChordVoice::Fifth(*note),
            ChordToneDegree::Seventh => ChordVoice::Seventh(*note),
        }
    }
}

impl Display for ChordVoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChordVoice::Root(note) => write!(f, "Root: {note}"),
            ChordVoice::Second(note) => write!(f, "Second: {note}"),
            ChordVoice::Third(note) => write!(f, "Third: {note}"),
            ChordVoice::Fourth(note) => write!(f, "Fourth: {note}"),
            ChordVoice::Fifth(note) => write!(f, "Fifth: {note}"),
            ChordVoice::Seventh(note) => write!(f, "Seventh: {note}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChordVoicing {
    chord_name: ChordName,
    note_weights: Vec<usize>,
    root: NoteOct,
    transitions: Option<HashMap<ChordVoice, ChordVoice>>,
    frequencies: Vec<f64>,
    voices: (ChordVoice, ChordVoice, ChordVoice),
}

#[allow(dead_code)]
impl ChordVoicing {
    pub fn default() -> Self {
        let voices = [NoteOct::C(3), NoteOct::E(3), NoteOct::G(3)];
        let [root, third, fifth] = voices;
        let mut note_weights = Vec::with_capacity(3);
        let mut frequencies: Vec<f64> = Vec::with_capacity(3);
        for voice in voices {
            frequencies.push(*music_theory_baux.get_freq(&voice).unwrap());
            note_weights.push(*music_theory_baux.get_note_weight(&voice).unwrap());
        }

        ChordVoicing {
            chord_name: ChordName::C(ChordQuality::Maj),
            root,
            voices: (
                ChordVoice::Root(root),
                ChordVoice::Third(third),
                ChordVoice::Fifth(fifth),
            ),
            note_weights,
            transitions: None,
            frequencies,
        }
    }

    pub fn chord_name(&self) -> ChordName {
        self.chord_name
    }

    pub fn quality(&self) -> ChordQuality {
        self.chord_name.get_quality()
    }

    pub fn note_weights(&self) -> &Vec<usize> {
        &self.note_weights
    }

    pub fn frequencies(&self) -> &Vec<f64> {
        &self.frequencies
    }

    pub fn root(&self) -> NoteOct {
        self.root
    }

    pub fn get_transitions(&self) -> &Option<HashMap<ChordVoice, ChordVoice>> {
        &self.transitions
    }

    pub fn to_voicing(&self, next_chord: &ChordName) -> ChordVoicing {
        // logic here for comparing current chord note weights to (all) possible note weights from `next_chord` weights lookup
        todo!()
    }
}

pub fn from_input(
    name: &str,
    root_oct: Option<usize>,
    voice_1_oct: Option<usize>,
    voice_2_oct: Option<usize>,
) -> Result<ChordVoicing, ChordVoicingError> {
    let chord_name = ChordName::try_from(name);

    if chord_name.is_ok() {
        let chord_name = chord_name.unwrap();
        let chord_notes = music_theory_baux
            .get_chord_spelling(&chord_name)
            .expect("should be a valid name access for `spelling` in `from_input`")
            .spelling();

        let root_oct = match root_oct {
            Some(root_oct) => {
                if root_oct < 1 || root_oct > 8 {
                    return Err(ChordVoicingError::InvalidOct(root_oct, "root".to_string()));
                }
                root_oct
            }
            None => 4,
        };

        let root_note_oct = NoteOct::from_note(chord_notes[0].note(), root_oct);
        let root_freq = *music_theory_baux
            .get_freq(&root_note_oct)
            .expect("should be a valid root note `freq` lookup");
        let root_weight = *music_theory_baux
            .get_note_weight(&root_note_oct)
            .expect("should be a valid root note `weight` lookup");
        let root_voice = ChordVoice::from_parts(&root_note_oct, &chord_notes[0]);

        let voice_1_oct = match voice_1_oct {
            Some(voice_oct) => {
                if voice_oct < 1 || voice_oct > 8 {
                    return Err(ChordVoicingError::InvalidOct(
                        voice_oct,
                        "voice 1".to_string(),
                    ));
                }
                voice_oct
            }
            None => 4,
        };

        let note_oct_1 = NoteOct::from_note(chord_notes[1].note(), voice_1_oct);
        let voice_1_freq = *music_theory_baux
            .get_freq(&note_oct_1)
            .expect("should be a valid 'first voice' note `freq` lookup");
        let voice_1_weight = *music_theory_baux
            .get_note_weight(&note_oct_1)
            .expect("should be a valid 'first voice' `weight` lookup");
        let voice_1 = ChordVoice::from_parts(&note_oct_1, &chord_notes[1]);

        let voice_2_oct = match voice_2_oct {
            Some(voice_oct) => {
                if voice_oct < 1 || voice_oct > 8 {
                    return Err(ChordVoicingError::InvalidOct(
                        voice_oct,
                        "voice 2".to_string(),
                    ));
                }
                voice_oct
            }
            None => 4,
        };

        let note_oct_2 = NoteOct::from_note(chord_notes[2].note(), voice_2_oct);
        let voice_2_freq = *music_theory_baux
            .get_freq(&note_oct_2)
            .expect("should be a valid 'second voice' note `freq` lookup");
        let voice_2_weight = *music_theory_baux
            .get_note_weight(&note_oct_2)
            .expect("should be a valid 'second voice' `weight` lookup");
        let voice_2 = ChordVoice::from_parts(&note_oct_2, &chord_notes[2]);

        Ok(ChordVoicing {
            chord_name,
            root: root_note_oct,
            transitions: None,
            frequencies: vec![root_freq, voice_1_freq, voice_2_freq],
            voices: (root_voice, voice_1, voice_2),
            note_weights: vec![root_weight, voice_1_weight, voice_2_weight],
        })
    } else {
        Err(ChordVoicingError::ParseChordName(name.to_string()))
    }
}
