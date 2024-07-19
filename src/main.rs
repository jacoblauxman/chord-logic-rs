use music_baux::{Chord, ChordName, MusicTheoryBaux, NoteName, NoteOct};

fn main() {
    //
    // various methods / data return examples (read only)
    //

    //
    //
    // all data collections (via named `MusicTheoryBaux` fields)

    // println!("{:?}", MusicTheoryBaux.note_freqs());
    // println!("{:?}", MusicTheoryBaux.freq_notes());
    // println!("{:?}", MusicTheoryBaux.note_freq_collections());
    // println!("{:?}", MusicTheoryBaux.chord_spellings());
    // println!("{:?}", MusicTheoryBaux.enharmonics());
    //
    //

    // chord spelling
    let b_maj = MusicTheoryBaux
        .get_chord_spelling(&ChordName::Bmaj)
        .unwrap();
    println!("\nB Major is: {:?}\n", b_maj);

    // note frequency collection
    let all_bs = MusicTheoryBaux
        .get_note_freq_collection(&NoteName::B)
        .unwrap();
    println!("All B frequencies: {:?}\nFREQ CHECK:\n", all_bs);

    // frequency to note lookup
    all_bs.iter().for_each(|freq| {
        let note_oct = MusicTheoryBaux.get_note(*freq);

        println!("{} is {}", freq, note_oct.unwrap());
    });

    // note to frequency lookup
    let c_sharp4_freq = MusicTheoryBaux.get_freq(&NoteOct::CSharpDFlat4).unwrap();
    println!("\nC#/Db4 frequency: {}\n", c_sharp4_freq);

    // note weight lookup
    let c_sharp4_weight = MusicTheoryBaux.get_weight(&NoteOct::CSharpDFlat4).unwrap();
    println!("C#/Db4 `weight`: {}\n", c_sharp4_weight);

    // === CHORD STRUCT === //
    let c_chord = Chord::default();
    println!("DEFAULT C CHORD?: \n{:?}", c_chord);
}
