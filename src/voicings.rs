use crate::{
    music_theory_baux, ChordName, ChordQuality, ChordSpelling, ChordTone, ChordToneDegree, NoteOct,
};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct VoiceLeadingChoice {
    diff: usize,
    voice: ChordVoice,
}

#[derive(Debug, Clone)]
pub struct BestChoices {
    lo: (VoiceLeadingChoice, VoiceLeadingChoice),
    hi: (VoiceLeadingChoice, VoiceLeadingChoice),
}

#[derive(Debug, Clone)]
pub struct VoiceLeadingConfig {
    voices: Vec<ChordVoice>,
    diff: usize,
}

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

    pub fn note_oct(&self) -> &NoteOct {
        match self {
            ChordVoice::Root(note) => note,
            ChordVoice::Second(note) => note,
            ChordVoice::Third(note) => note,
            ChordVoice::Fourth(note) => note,
            ChordVoice::Fifth(note) => note,
            ChordVoice::Seventh(note) => note,
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
    root: NoteOct,
    voices: (ChordVoice, ChordVoice, ChordVoice),
    frequencies: Vec<f64>,
    note_weights: Vec<usize>,
    transitions: Option<HashMap<ChordVoice, ChordVoice>>,
}

#[allow(dead_code)]
impl ChordVoicing {
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

    pub fn voices(&self) -> &(ChordVoice, ChordVoice, ChordVoice) {
        &self.voices
    }

    pub fn get_transitions(&self) -> &Option<HashMap<ChordVoice, ChordVoice>> {
        &self.transitions
    }

    pub fn voice_lead(&self, new_chord: &ChordName) -> ChordVoicing {
        let new_spelling = music_theory_baux.get_chord_spelling(new_chord).unwrap();
        let new_voice_choices = self.get_new_voice_choices(new_spelling);

        let best_choices = self.calculate_best_choices(&new_voice_choices);
        let configurations = self.generate_leads(&best_choices);

        let valid_configs: Vec<_> = configurations
            .into_iter()
            .filter(|config| !self.has_duplicate_note(config))
            .collect();

        let mut sorted_configs = valid_configs;
        sorted_configs.sort_by_key(|config| config.diff);

        self.create_new_voicing(new_chord, &sorted_configs[0].voices)
    }

    fn calculate_diff(&self, old_val: &ChordVoice, new_val: &ChordVoice) -> usize {
        let old_weight = music_theory_baux
            .get_note_weight(old_val.note_oct())
            .unwrap();
        let new_weight = music_theory_baux
            .get_note_weight(new_val.note_oct())
            .unwrap();
        old_weight.abs_diff(*new_weight)
    }

    fn create_new_voicing(&self, new_chord: &ChordName, new_voices: &[ChordVoice]) -> ChordVoicing {
        println!("\n::new_chord: {}", new_chord);
        println!(
            "::new_voices: {} - {} - {}\n",
            new_voices[0], new_voices[1], new_voices[2]
        );

        let mut note_weights = Vec::new();
        let mut frequencies = Vec::new();

        for voice in new_voices {
            let note_oct = voice.note_oct();
            note_weights.push(*music_theory_baux.get_note_weight(note_oct).unwrap());
            frequencies.push(*music_theory_baux.get_freq(note_oct).unwrap());
        }

        let root = *new_voices
            .iter()
            .filter(|voice| voice.note_oct().note_name() == new_chord.get_root())
            .collect::<Vec<_>>()[0]
            .note_oct();

        ChordVoicing {
            chord_name: *new_chord,
            root,
            voices: (new_voices[0], new_voices[1], new_voices[2]),
            note_weights,
            transitions: None,
            frequencies,
        }
    }

    pub fn get_new_voice_choices(&self, new_spelling: &ChordSpelling) -> Vec<ChordVoice> {
        new_spelling
            .spelling()
            .iter()
            .flat_map(|chord_tone| {
                (1..=8).map(move |oct| {
                    let note_oct = NoteOct::from_note(chord_tone.note(), oct);
                    ChordVoice::from_parts(&note_oct, chord_tone)
                })
            })
            .collect()
    }

    fn calculate_best_choices(
        &self,
        new_chord_voices: &[ChordVoice],
    ) -> HashMap<ChordVoice, BestChoices> {
        let mut best_choices = HashMap::<ChordVoice, BestChoices>::new();

        for &old_voice in &[self.voices.0, self.voices.1, self.voices.2] {
            let old_weight = *music_theory_baux
                .get_note_weight(old_voice.note_oct())
                .unwrap();
            let mut lo = (
                VoiceLeadingChoice {
                    diff: usize::MAX,
                    voice: old_voice,
                },
                VoiceLeadingChoice {
                    diff: usize::MAX,
                    voice: old_voice,
                },
            );
            let mut hi = (
                VoiceLeadingChoice {
                    diff: usize::MAX,
                    voice: old_voice,
                },
                VoiceLeadingChoice {
                    diff: usize::MAX,
                    voice: old_voice,
                },
            );

            for &new_voice in new_chord_voices {
                let new_weight = *music_theory_baux
                    .get_note_weight(new_voice.note_oct())
                    .unwrap();
                let diff = old_weight.abs_diff(new_weight);

                if new_weight <= old_weight {
                    if diff < lo.0.diff {
                        lo.1 = lo.0.clone();
                        lo.0 = VoiceLeadingChoice {
                            diff,
                            voice: new_voice,
                        };
                    } else if diff < lo.1.diff {
                        lo.1 = VoiceLeadingChoice {
                            diff,
                            voice: new_voice,
                        };
                    }
                } else if diff < hi.0.diff {
                    hi.1 = hi.0.clone();
                    hi.0 = VoiceLeadingChoice {
                        diff,
                        voice: new_voice,
                    };
                } else if diff < hi.1.diff {
                    hi.1 = VoiceLeadingChoice {
                        diff,
                        voice: new_voice,
                    };
                }
            }

            best_choices.insert(old_voice, BestChoices { lo, hi });
        }

        best_choices
    }

    fn generate_leads(
        &self,
        best_choices: &HashMap<ChordVoice, BestChoices>,
    ) -> Vec<VoiceLeadingConfig> {
        let mut configurations = vec![VoiceLeadingConfig {
            voices: Vec::new(),
            diff: 0,
        }];

        for choices in best_choices.values() {
            let new_choices = vec![&choices.lo.0, &choices.lo.1, &choices.hi.0, &choices.hi.1];

            configurations = self.generate_configs(configurations, new_choices);
        }

        configurations
    }

    fn generate_configs(
        &self,
        current_configs: Vec<VoiceLeadingConfig>,
        choices: Vec<&VoiceLeadingChoice>,
    ) -> Vec<VoiceLeadingConfig> {
        let mut new_configs = Vec::new();

        for config in current_configs {
            for choice in &choices {
                let mut new_voices = config.voices.clone();

                new_voices.push(choice.voice);
                new_configs.push(VoiceLeadingConfig {
                    voices: new_voices,
                    // todo(?): sort out usize::MAX add'ing
                    // diff: config.diff + choice.diff,
                    diff: config.diff.saturating_add(choice.diff),
                });
            }
        }

        new_configs
    }

    fn has_duplicate_note(&self, config: &VoiceLeadingConfig) -> bool {
        let mut note_set = HashSet::new();

        for voice in &config.voices {
            let note_name = voice.note_oct().note_name();
            if note_set.contains(&note_name) {
                return true;
            }
            note_set.insert(note_name);
        }

        false
    }
}

impl Default for ChordVoicing {
    fn default() -> Self {
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
            frequencies,
            note_weights,
            transitions: None,
        }
    }
}

//
//
//

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
                if !(1..=8).contains(&root_oct) {
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
                if !(1..=8).contains(&voice_oct) {
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
                if !(1..=8).contains(&voice_oct) {
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
