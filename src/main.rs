use music_baux::{generate_chord_spellings, generate_enharmonics};

fn main() {
    println!("Hello, world!");

    let _enharmonics = generate_enharmonics();
    // println!("{:?}", enharmonics);

    let (_spellings, _typed_spellings) = generate_chord_spellings();
    // println!("{:?}", _typed_spellings);
}
