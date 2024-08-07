use once_cell::sync::Lazy;

use crate::{generate_music_data, ChordName, ChordSpelling, NoteName, NoteOct};
use std::collections::HashMap;

pub struct MusicTheoryBaux {
    note_freqs: HashMap<NoteOct, f64>,
    freq_notes: HashMap<&'static str, NoteOct>,
    note_freq_collections: HashMap<NoteName, Vec<f64>>,
    note_weights: HashMap<NoteOct, usize>,
    weight_notes: HashMap<usize, NoteOct>,
    chord_spellings: HashMap<ChordName, ChordSpelling>,
    enharmonics: HashMap<NoteName, NoteName>,
}

impl MusicTheoryBaux {
    pub fn new() -> Self {
        let (
            note_freqs,
            freq_notes,
            note_freq_collections,
            note_weights,
            weight_notes,
            chord_spellings,
            enharmonics,
            _scale_relatives,
        ) = generate_music_data();

        Self {
            note_freqs,
            freq_notes,
            note_freq_collections,
            note_weights,
            weight_notes,
            chord_spellings,
            enharmonics,
        }
    }

    // === DATA COLLECTION(S) ACCESS === //
    pub fn note_freqs(&self) -> &HashMap<NoteOct, f64> {
        &self.note_freqs
    }

    pub fn freq_notes(&self) -> &HashMap<&'static str, NoteOct> {
        &self.freq_notes
    }

    pub fn note_freq_collections(&self) -> &HashMap<NoteName, Vec<f64>> {
        &self.note_freq_collections
    }

    pub fn note_weights(&self) -> &HashMap<NoteOct, usize> {
        &self.note_weights
    }

    pub fn weight_notes(&self) -> &HashMap<usize, NoteOct> {
        &self.weight_notes
    }

    pub fn chord_spellings(&self) -> &HashMap<ChordName, ChordSpelling> {
        &self.chord_spellings
    }

    pub fn enharmonics(&self) -> &HashMap<NoteName, NoteName> {
        &self.enharmonics
    }

    // === SPECIFIC DATA ACCESS (user input / optional returns) === //
    pub fn get_note_freq_collection(&self, note: &NoteName) -> Option<&Vec<f64>> {
        self.note_freq_collections.get(note)
    }

    pub fn get_freq(&self, note: &NoteOct) -> Option<&f64> {
        self.note_freqs.get(note)
    }

    pub fn get_note(&self, freq: f64) -> Option<&NoteOct> {
        // tolerance for 'close' check of our f64 frequency values (with `0.000000001`)
        const EPSILON: f64 = 1e-9;

        self.freq_notes
            .iter()
            .find(|(&k, _)| {
                let k_freq: f64 = k.parse().unwrap_or(f64::NAN);
                (k_freq - freq).abs() < EPSILON
            })
            .map(|(_, v)| v)
    }

    pub fn get_note_weight(&self, note: &NoteOct) -> Option<&usize> {
        self.note_weights.get(note)
    }

    pub fn get_weight_note(&self, weight: &usize) -> Option<&NoteOct> {
        self.weight_notes.get(weight)
    }

    pub fn get_chord_spelling(&self, chord: &ChordName) -> Option<&ChordSpelling> {
        self.chord_spellings.get(chord)
    }

    pub fn get_enharmonic(&self, note: &NoteName) -> Option<&NoteName> {
        self.enharmonics.get(note)
    }
}

impl Default for MusicTheoryBaux {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(non_upper_case_globals)]
pub static music_theory_baux: Lazy<MusicTheoryBaux> = Lazy::new(MusicTheoryBaux::new);
