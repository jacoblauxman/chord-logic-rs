use music_baux::{ChordName, ChordQuality, MusicTheoryBaux, NoteName, NoteOct};

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

    // for (_, spelling) in MusicTheoryBaux.chord_spellings() {
    //     println!("{}", spelling);
    // }
    // println!("{:?}", MusicTheoryBaux.enharmonics());
    //
    //

    // // chord spelling
    let b_maj = MusicTheoryBaux
        .get_chord_spelling(&ChordName::B(ChordQuality::Maj))
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
    let c_sharp4_freq = MusicTheoryBaux.get_freq(&NoteOct::CSharpDFlat(4)).unwrap();
    println!("\nC#/Db4 frequency: {}\n", c_sharp4_freq);

    // note weight lookup
    let c_sharp4_weight = MusicTheoryBaux
        .get_note_weight(&NoteOct::CSharpDFlat(4))
        .unwrap();
    println!("C#/Db4 `weight`: {}\n", c_sharp4_weight);

    let c_sharp4 = MusicTheoryBaux.get_weight_note(c_sharp4_weight).unwrap();
    println!("`weight` {} equivalent: {}\n", c_sharp4_weight, c_sharp4);

    // // === CHORD STRUCT === //

    // // default example
    // let c_chord = Chord::default();
    // println!("DEFAULT C CHORD?: \n{:?}\n", c_chord);

    // OTHER STUFF -- TESTING!
    // let c_7sus4 = ChordName::C(music_baux::ChordQuality::SevSus4);
    // println!("It can stringify? {c_7sus4}");
}
