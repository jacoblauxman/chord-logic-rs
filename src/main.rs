#[allow(unused_imports)]
use music_baux::{
    from_input, music_theory_baux, ChordName, ChordQuality, ChordVoicing, NoteName, NoteOct,
};
use music_baux::{ScaleDegree, ScaleName, ScaleQuality};
use std::time::Instant;

fn main() {
    // === MESSY ROOM FOR NOW, YES SOME OF THIS COULD PROB BE TESTS === //

    //
    // various methods / data return examples (read only)
    //

    // -- all data collections (via named `music_theory_baux` fields) -- //
    // println!("{:?}", music_theory_baux.note_freqs());
    // println!("{:?}", music_theory_baux.freq_notes());
    // println!("{:?}", music_theory_baux.note_freq_collections());
    // println!("{:?}", music_theory_baux.chord_spellings());

    let start = Instant::now();

    // let mut all_chords = Vec::<ChordName>::with_capacity(120);
    // for spelling in music_theory_baux.chord_spellings().values() {
    //     println!("{}", spelling);
    //     all_chords.push(*spelling.name());
    // }
    // println!("{:?}", music_theory_baux.enharmonics());
    //
    //

    // -- specific data -- //
    // // chord spelling
    // let b_maj = music_theory_baux
    //     .get_chord_spelling(&ChordName::B(ChordQuality::Maj))
    //     .unwrap();
    // println!("\nB Major is: {:?}\n", b_maj);

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

    //
    //
    //

    // === ! TESTING ! === //

    // default
    // let default_voicing = ChordVoicing::default();
    // println!("\nDEFAULT VOICING:\n{:?}\n", default_voicing);
    // let mut curr_voicing = default_voicing;

    // let mut chord_count = 1;
    // all_chords.iter().for_each(|next_chord| {
    //     println!(
    //         "\n CHORD COUNT AT {}...iterating to next chord: {:?}...\n",
    //         chord_count, next_chord
    //     );

    //     chord_count += 1;
    //     curr_voicing = curr_voicing.voice_lead(next_chord);
    //     println!("NEW CHORD:\n {:?}\n", curr_voicing);
    //     let voices = curr_voicing.voices();
    //     println!("::notes: {} - {} - {}", voices.0, voices.1, voices.2)
    // });

    // progression - with `from_input` for unique voice shape
    // let starter = from_input("Bmaj7", Some(4), Some(5), Some(5)).unwrap();
    // println!(
    //     "\nSTARTER CHORD (WITH INPUT PARTS {} {} {} {}): {:?}",
    //     &"Bmaj7",
    //     4,
    //     5,
    //     4,
    //     starter.chord_name()
    // );

    // let voices = starter.voices();
    // println!("::voices: {} - {} - {}", voices.0, voices.1, voices.2);

    // let mut curr_voicing = starter;

    // let chord_progression = [
    //     ChordName::A(ChordQuality::MajSev),
    //     ChordName::CSharpDFlat(ChordQuality::MinSev),
    //     ChordName::B(ChordQuality::SevSus),
    //     ChordName::B(ChordQuality::Sev),
    //     ChordName::FSharpGFlat(ChordQuality::Sus2),
    //     ChordName::E(ChordQuality::Sus4),
    //     ChordName::DSharpEFlat(ChordQuality::MajSev),
    // ];

    // let first_chord = from_input("Fm7", Some(4), Some(5), Some(5)).unwrap();
    // println!(
    //     "\nSTARTER CHORD (WITH INPUT PARTS {} {} {} {}): {:?}",
    //     &"Fmin7",
    //     4,
    //     5,
    //     4,
    //     first_chord.chord_name()
    // );
    // let voices = first_chord.voices();
    // println!("::voices: {} - {} - {}", voices.0, voices.1, voices.2);

    // let mut curr_voicing = first_chord;

    // let chord_progression = [
    //     ChordName::try_from("Bb7").unwrap(),
    //     ChordName::try_from("Ebmaj7").unwrap(),
    //     ChordName::try_from("Amin7").unwrap(),
    //     ChordName::try_from("D7").unwrap(),
    //     ChordName::try_from("Gmaj7").unwrap(),
    //     ChordName::try_from("C#m7").unwrap(),
    //     ChordName::try_from("F#7").unwrap(),
    //     ChordName::try_from("Bmaj7").unwrap(),
    //     ChordName::try_from("F-7").unwrap(),
    //     ChordName::try_from("Bbdom7").unwrap(),
    //     ChordName::try_from("Eb^7").unwrap(),
    //     ChordName::try_from("C#m7").unwrap(),
    //     ChordName::try_from("F#7").unwrap(),
    // ];

    // ---- //

    let first_chord = from_input("Emaj7", Some(4), Some(5), Some(5)).unwrap();
    println!(
        "\nSTARTER CHORD (WITH INPUT PARTS {} {} {} {}): {:?}",
        &"Emaj7",
        4,
        5,
        4,
        first_chord.chord_name()
    );
    let voices = first_chord.voices();
    println!("::voices: {} - {} - {}", voices.0, voices.1, voices.2);

    let mut curr_voicing = first_chord;

    let chord_progression = [
        ChordName::A(ChordQuality::MajSev),
        ChordName::CSharpDFlat(ChordQuality::MinSev),
        ChordName::B(ChordQuality::MajSev),
        ChordName::E(ChordQuality::MajSev),
    ];

    let chord_progression = [
        ChordName::A(ChordQuality::MajSev),
        ChordName::CSharpDFlat(ChordQuality::MinSev),
        ChordName::B(ChordQuality::Sev),
        ChordName::E(ChordQuality::MajSev),
    ];

    chord_progression.iter().for_each(|next_chord| {
        println!("...iterating to next chord: {:?}...", next_chord);
        curr_voicing = curr_voicing.voice_lead(next_chord);
    });

    // === SCALES === //

    // println!("{}", ScaleName::C(ScaleQuality::MajPent));
    // println!("{}", ScaleDegree::SharpSecond(NoteName::ASharpBFlat));

    let duration = start.elapsed();

    println!("\n --- Took {duration:?} --- \n");
}
