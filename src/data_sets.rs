use crate::{ChordName, NoteName, NoteOct};
use std::collections::HashMap;

static NOTE_DATA: &str = include_str!(".././note_frequencies.txt");
static CHORD_DATA: &str = include_str!(".././chord_spellings.txt");

// === NOTES/FREQUENCIES || NOTE WEIGHTS === //

// `note_freqs`, `freq_notes`, `note_freq_collections`, and `note_weights` fields for `MusicBaux`
fn generate_notes_freqs_data() -> (
    HashMap<NoteOct, f64>,
    HashMap<&'static str, NoteOct>,
    HashMap<NoteName, Vec<f64>>,
    HashMap<NoteOct, usize>,
) {
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

                // add `NoteOct` weight value and increment for next line value (`C1 = 0, C#/Db1 = 1, D1 = 2...`)
                note_weights.insert(note_oct, weight);
                weight += 1;
            }
        }
    }

    (note_freqs, freq_notes, note_freq_collections, note_weights)
}

// === ENHARMONICS === //

// `MusicBaux` `enharmonics` field
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

fn generate_chord_spellings() -> HashMap<ChordName, Vec<NoteName>> {
    let mut spellings: HashMap<ChordName, Vec<NoteName>> = HashMap::new();

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

            spellings.insert(typed_name, typed_notes);
        }
    }

    spellings
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
    HashMap<ChordName, Vec<NoteName>>,
    HashMap<NoteName, NoteName>,
) {
    let (note_freqs, freq_notes, note_freq_collections, note_weights) = generate_notes_freqs_data();
    let chord_spellings = generate_chord_spellings();
    let enharmonics = generate_enharmonics();

    (
        note_freqs,
        freq_notes,
        note_freq_collections,
        note_weights,
        chord_spellings,
        enharmonics,
    )
}
