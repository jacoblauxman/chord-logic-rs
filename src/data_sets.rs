use crate::{ChordName, ChordSpelling, NoteName, NoteOct, ScaleName, ScaleSpelling};
use std::collections::HashMap;

static NOTE_DATA: &str = include_str!(".././note_frequencies.txt");
static CHORD_DATA: &str = include_str!(".././chord_spellings.txt");
static SCALE_DATA: &str = include_str!(".././major_minor_scales.txt");

// === NOTES/FREQUENCIES || NOTE WEIGHTS === //

// todo(?): refactor to an encompassing 'type' (re: clippy)
// #[derive(Debug, Clone)]
// pub struct NoteFreqData {
//     note_freqs: HashMap<NoteOct, f64>,
//     freq_notes: HashMap<&'static str, NoteOct>,
//     note_freq_collections: HashMap<NoteName, Vec<f64>>,
//     note_weights: HashMap<NoteOct, usize>,
//     weight_notes: HashMap<usize, NoteOct>,
// }

// `note_freqs`, `freq_notes`, `note_freq_collections`, and `note_weights` fields for `MusicTheoryBaux`
fn generate_notes_freqs_data() -> (
    HashMap<NoteOct, f64>,
    HashMap<&'static str, NoteOct>,
    HashMap<NoteName, Vec<f64>>,
    HashMap<NoteOct, usize>,
    HashMap<usize, NoteOct>,
) {
    // todo(?): refactor to an encompassing 'type' (re: clippy)
    // fn generate_notes_freqs_data() -> NoteFreqData {
    let mut note_freqs = HashMap::<NoteOct, f64>::new();
    let mut freq_notes = HashMap::<&'static str, NoteOct>::new();
    let mut note_freq_collections = HashMap::<NoteName, Vec<f64>>::from([
        (NoteName::A, vec![]),
        (NoteName::ASharpBFlat, vec![]),
        (NoteName::B, vec![]),
        (NoteName::C, vec![]),
        (NoteName::CSharpDFlat, vec![]),
        (NoteName::D, vec![]),
        (NoteName::DSharpEFlat, vec![]),
        (NoteName::E, vec![]),
        (NoteName::F, vec![]),
        (NoteName::FSharpGFlat, vec![]),
        (NoteName::G, vec![]),
        (NoteName::GSharpAFlat, vec![]),
    ]);
    let mut note_weights = HashMap::<NoteOct, usize>::new();
    let mut weight_notes = HashMap::<usize, NoteOct>::new();
    let mut weight = 0;

    for line in NOTE_DATA.lines() {
        if let Some((note_name_str, freq_str)) = line.split_once(" = ") {
            let freq_str = freq_str.trim();
            let freq = freq_str
                .parse::<f64>()
                .expect("ERROR: `generate_note_freqs` parsing issue in initial data read of `freq` line value - note_frequencies.txt");
            let note_name_parts = note_name_str
                .trim()
                .split(&['[', ']'])
                .collect::<Vec<&str>>();

            // expect a collection shape similar to `["noteFreq", "1", "", "\"C\"", ""]`
            if note_name_parts.len() < 5 {
                panic!("ERROR: `generate_note_freqs` parsing issue in iniital data read of `note_name_parts` line value - note_frequencies.txt")
            } else {
                let note = note_name_parts[3].replace('"', "");
                let octave = note_name_parts[1];

                let note_name = NoteName::try_from(note.as_str()).unwrap();

                // add current frequency to `NoteName` collection
                let note_name_freqs = note_freq_collections
                    .get_mut(&note_name)
                    .expect("`note_freqs_collection` should have all `NoteName` values");
                note_name_freqs.push(freq);

                // format and add `NoteOct` freq value to collection
                let note_oct = format!("{}{}", note, octave);
                let note_oct = NoteOct::try_from(note_oct.as_str()).expect("ERROR: malformed data in parsing `NoteOct` from `note_name` (in `generate_note_freqs`");

                note_freqs.insert(note_oct, freq);

                // add `freq` (str)'s `NoteOct` value to collection
                freq_notes.insert(freq_str, note_oct);

                // add `NoteOct` weight value  (`C1 = 0, C#/Db1 = 1, D1 = 2...`)
                note_weights.insert(note_oct, weight);
                // add weight value's `NoteOct`
                weight_notes.insert(weight, note_oct);
                // increment `weight` for next input line's value
                weight += 1;
            }
        }
    }

    // NoteFreqData {
    //     note_freqs,
    //     freq_notes,
    //     note_freq_collections,
    //     note_weights,
    //     weight_notes,
    // }
    (
        note_freqs,
        freq_notes,
        note_freq_collections,
        note_weights,
        weight_notes,
    )
}

// === ENHARMONICS === //

fn generate_enharmonics() -> HashMap<NoteName, NoteName> {
    [
        ("C#", "Db"),
        ("Db", "C#"),
        ("D#", "Eb"),
        ("Eb", "D#"),
        ("F#", "Gb"),
        ("Gb", "F#"),
        ("G#", "Ab"),
        ("Ab", "G#"),
        ("A#", "Bb"),
        ("Bb", "A#"),
    ]
    .into_iter()
    .map(|(target, enharm)| {
        let target = NoteName::try_from(target)
            .expect("should convert enharmonic note string values successfully");
        let enharm = NoteName::try_from(enharm)
            .expect("should convert enharmonic note string values successfully");

        (target, enharm)
    })
    .fold(HashMap::new(), |mut acc, (target, enharm)| {
        acc.insert(target, enharm);
        acc
    })
}

//
//
//

// === CHORD SPELLINGS === //

fn generate_chord_spellings() -> HashMap<ChordName, ChordSpelling> {
    let mut chord_spellings: HashMap<ChordName, ChordSpelling> = HashMap::new();

    for line in CHORD_DATA.lines() {
        if let Some((name, notes_str)) = line.split_once(": ") {
            let notes: Vec<&str> = notes_str
                .trim()
                .trim_start_matches('[')
                .trim_end_matches("],")
                .trim()
                .split(", ")
                .collect();

            let typed_name: ChordName = name.try_into().unwrap();
            let typed_notes: Vec<NoteName> =
                notes.iter().map(|n| n.trim().try_into().unwrap()).collect();

            let chord_spelling = ChordSpelling::new(
                &typed_name.get_root(),
                &typed_name.get_quality(),
                &typed_notes,
            );

            chord_spellings.insert(typed_name, chord_spelling);
        }
    }

    chord_spellings
}

//
//
//

// === `RUN IT!` === //

pub fn generate_music_data() -> (
    HashMap<NoteOct, f64>,
    HashMap<&'static str, NoteOct>,
    HashMap<NoteName, Vec<f64>>,
    HashMap<NoteOct, usize>,
    HashMap<usize, NoteOct>,
    HashMap<ChordName, ChordSpelling>,
    HashMap<NoteName, NoteName>,
    HashMap<NoteName, NoteName>,
) {
    let (note_freqs, freq_notes, note_freq_collections, note_weights, weight_notes) =
        generate_notes_freqs_data();
    let chord_spellings = generate_chord_spellings();
    let enharmonics = generate_enharmonics();

    let scale_relatives = generate_relative_keys();

    (
        note_freqs,
        freq_notes,
        note_freq_collections,
        note_weights,
        weight_notes,
        chord_spellings,
        enharmonics,
        scale_relatives,
    )
}

//
//
//

// === SCALE SPELLING === //
fn _generate_scale_spellings() -> HashMap<ScaleName, ScaleSpelling> {
    todo!();
    // let mut scale_spellings: HashMap<ScaleName, ScaleSpelling> = HashMap::new();

    // for line in SCALE_DATA.lines() {}
}

// === RELATIVE KEYS (MAJOR-MINOR) === //

// Note: "major -> minor" scale root note kv's
fn generate_relative_keys() -> HashMap<NoteName, NoteName> {
    [
        ("C", "A"),
        ("C#", "A#"),
        ("D", "B"),
        ("D#", "Eb"),
        ("E", "C#"),
        ("F", "D"),
        ("F#", "Gb"),
        ("G", "E"),
        ("G#", "Ab"),
        ("A", "F#"),
        ("Bb", "G"),
        ("B", "G#"),
    ]
    .into_iter()
    .map(|(maj, min)| {
        let maj = NoteName::try_from(maj)
            .expect("should convert relative maj-min note string values successfully");
        let min = NoteName::try_from(min)
            .expect("should convert relative maj-min note string values successfully");

        (maj, min)
    })
    .fold(HashMap::new(), |mut acc, (maj, min)| {
        acc.insert(maj, min);
        acc
    })
}
