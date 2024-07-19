#[allow(unused_imports)]
use music_baux::{music_theory_baux, ChordName, ChordQuality, NoteName, NoteOct};

fn main() {
    //
    // various methods / data return examples (read only)
    //

    // -- all data collections (via named `music_theory_baux` fields) -- //
    // println!("{:?}", music_theory_baux.note_freqs());
    // println!("{:?}", music_theory_baux.freq_notes());
    // println!("{:?}", music_theory_baux.note_freq_collections());
    println!("{:?}", music_theory_baux.chord_spellings());

    // for (_, spelling) in music_theory_baux.chord_spellings() {
    //     println!("{}", spelling);
    // }
    // println!("{:?}", music_theory_baux.enharmonics());
    //
    //

    // -- specific data -- //
    // // chord spelling
    let b_maj = music_theory_baux
        .get_chord_spelling(&ChordName::B(ChordQuality::Maj))
        .unwrap();
    println!("\nB Major is: {:?}\n", b_maj);

    // // note frequency collection
    // let all_bs = music_theory_baux
    //     .get_note_freq_collection(&NoteName::B)
    //     .unwrap();
    // println!("All B frequencies: {:?}\nFREQ CHECK:\n", all_bs);

    // // frequency to note lookup
    // all_bs.iter().for_each(|freq| {
    //     let note_oct = music_theory_baux.get_note(*freq);

    //     println!("{} is {}", freq, note_oct.unwrap());
    // });

    // // note to frequency lookup
    // let c_sharp4_freq = music_theory_baux
    //     .get_freq(&NoteOct::CSharpDFlat(4))
    //     .unwrap();
    // println!("\nC#/Db4 frequency: {}\n", c_sharp4_freq);

    // // note weight lookup
    // let c_sharp4_weight = music_theory_baux
    //     .get_note_weight(&NoteOct::CSharpDFlat(4))
    //     .unwrap();
    // println!("C#/Db4 `weight`: {}\n", c_sharp4_weight);

    // let c_sharp4 = music_theory_baux.get_weight_note(c_sharp4_weight).unwrap();
    // println!("`weight` {} equivalent: {}\n", c_sharp4_weight, c_sharp4);
}
