use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
#[error("Error trying in conversion of note name `{0}` to `NoteName` enum variant")]
pub struct NoteNameError(String);

#[derive(Debug, Clone, PartialEq)]
pub enum NoteName {
    A,
    ASharpBFlat,
    B,
    C,
    CSharpDFlat,
    D,
    DSharpEFlat,
    E,
    F,
    FSharpGFlat,
    G,
    GSharpAFlat,
}

impl Display for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteName::C => write!(f, "C"),
            NoteName::CSharpDFlat => write!(f, "C#/Db"),
            NoteName::D => write!(f, "D"),
            NoteName::DSharpEFlat => write!(f, "D#/Eb"),
            NoteName::E => write!(f, "E"),
            NoteName::F => write!(f, "F"),
            NoteName::FSharpGFlat => write!(f, "F#/Gb"),
            NoteName::G => write!(f, "G"),
            NoteName::GSharpAFlat => write!(f, "G#/Ab"),
            NoteName::A => write!(f, "A"),
            NoteName::ASharpBFlat => write!(f, "A#/Bb"),
            NoteName::B => write!(f, "B"),
        }
    }
}

impl TryFrom<&str> for NoteName {
    type Error = NoteNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "C" => Ok(NoteName::C),
            "C#" | "Db" | "C#/Db" => Ok(NoteName::CSharpDFlat),
            "D" => Ok(NoteName::D),
            "D#" | "Eb" | "D#/Eb" => Ok(NoteName::DSharpEFlat),
            "E" => Ok(NoteName::E),
            "F" => Ok(NoteName::F),
            "F#" | "Gb" | "F#/Gb" => Ok(NoteName::FSharpGFlat),
            "G" => Ok(NoteName::G),
            "G#" | "Ab" | "G#/Ab" => Ok(NoteName::GSharpAFlat),
            "A" => Ok(NoteName::A),
            "A#" | "Bb" | "A#/Bb" => Ok(NoteName::ASharpBFlat),
            "B" => Ok(NoteName::B),
            unknown => Err(NoteNameError(unknown.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Error trying in conversion of chord name `{0}` to `ChordName` enum variant")]
pub struct ChordNameError(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChordName {
    Cmaj,
    Cmin,
    Caug,
    Cdim,
    CSharpDFlatmaj,
    CSharpDFlatmin,
    CSharpDFlataug,
    CSharpDFlatdim,
    Dmaj,
    Dmin,
    Daug,
    Ddim,
    DSharpEFlatmaj,
    DSharpEFlatmin,
    DSharpEFlataug,
    DSharpEFlatdim,
    Emaj,
    Emin,
    Eaug,
    Edim,
    Fmaj,
    Fmin,
    Faug,
    Fdim,
    FSharpGFlatmaj,
    FSharpGFlatmin,
    FSharpGFlataug,
    FSharpGFlatdim,
    Gmaj,
    Gmin,
    Gaug,
    Gdim,
    GSharpAFlatmaj,
    GSharpAFlatmin,
    GSharpAFlataug,
    GSharpAFlatdim,
    Amaj,
    Amin,
    Aaug,
    Adim,
    ASharpBFlatmaj,
    ASharpBFlatmin,
    ASharpBFlataug,
    ASharpBFlatdim,
    Bmaj,
    Bmin,
    Baug,
    Bdim,
}

impl TryFrom<&str> for ChordName {
    type Error = ChordNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "C" | "Cmaj" | "CM" => Ok(ChordName::Cmaj),
            "Cm" | "Cmin" | "C-" => Ok(ChordName::Cmin),
            "Caug" | "C+" => Ok(ChordName::Caug),
            "Cdim" | "CØ" | "C°" => Ok(ChordName::Cdim),

            "C#" | "Db" | "C#maj" | "Dbmaj" | "C#/Dbmaj" | "C#M" | "DbM" | "C#/DbM" => {
                Ok(ChordName::CSharpDFlatmaj)
            }
            "C#m" | "Dbm" | "C#/Dbm" | "C#min" | "Dbmin" | "C#/Dbmin" | "C#-" | "Db-"
            | "C#/Db-" => Ok(ChordName::CSharpDFlatmin),
            "C#aug" | "C#+" | "Dbaug" | "Db+" | "C#/Db+" | "C#/Dbaug" => {
                Ok(ChordName::CSharpDFlataug)
            }
            "C#dim" | "C#Ø" | "C#°" | "Dbdim" | "DbØ" | "Db°" | "C#/Dbdim" | "C#/DbØ"
            | "C#/Db°" => Ok(ChordName::CSharpDFlatdim),

            "D" | "Dmaj" | "DM" => Ok(ChordName::Dmaj),
            "Dm" | "Dmin" | "D-" => Ok(ChordName::Dmin),
            "Daug" | "D+" => Ok(ChordName::Daug),
            "Ddim" | "DØ" | "D°" => Ok(ChordName::Ddim),

            "D#" | "Eb" | "D#maj" | "Ebmaj" | "D#/Ebmaj" | "D#M" | "EbM" | "D#/EbM" => {
                Ok(ChordName::DSharpEFlatmaj)
            }
            "D#m" | "Ebm" | "D#/Ebm" | "D#min" | "Ebmin" | "D#/Ebmin" | "D#-" | "Eb-"
            | "D#/Eb-" => Ok(ChordName::DSharpEFlatmin),
            "D#aug" | "D#+" | "Ebaug" | "Eb+" | "D#/Eb+" | "D#/Ebaug" => {
                Ok(ChordName::DSharpEFlataug)
            }
            "D#dim" | "D#Ø" | "D#°" | "Ebdim" | "EbØ" | "Eb°" | "D#/Ebdim" | "D#/EbØ"
            | "D#/Eb°" => Ok(ChordName::DSharpEFlatdim),

            "E" | "Emaj" | "EM" => Ok(ChordName::Emaj),
            "Em" | "Emin" | "E-" => Ok(ChordName::Emin),
            "Eaug" | "E+" => Ok(ChordName::Eaug),
            "Edim" | "EØ" | "E°" => Ok(ChordName::Edim),

            "F" | "Fmaj" | "FM" => Ok(ChordName::Fmaj),
            "Fm" | "Fmin" | "F-" => Ok(ChordName::Fmin),
            "Faug" | "F+" => Ok(ChordName::Faug),
            "Fdim" | "FØ" | "F°" => Ok(ChordName::Fdim),

            "F#" | "Gb" | "F#maj" | "Gbmaj" | "F#/Gbmaj" | "F#M" | "GbM" | "F#/GbM" => {
                Ok(ChordName::FSharpGFlatmaj)
            }
            "F#m" | "Gbm" | "F#/Gbm" | "F#min" | "Gbmin" | "F#/Gbmin" | "F#-" | "Gb-"
            | "F#/Gb-" => Ok(ChordName::FSharpGFlatmin),
            "F#aug" | "F#+" | "Gbaug" | "Gb+" | "F#/Gb+" | "F#/Gbaug" => {
                Ok(ChordName::FSharpGFlataug)
            }
            "F#dim" | "F#Ø" | "F#°" | "Gbdim" | "GbØ" | "Gb°" | "F#/Gbdim" | "F#/GbØ"
            | "F#/Gb°" => Ok(ChordName::FSharpGFlatdim),

            "G" | "Gmaj" | "GM" => Ok(ChordName::Gmaj),
            "Gm" | "Gmin" | "G-" => Ok(ChordName::Gmin),
            "Gaug" | "G+" => Ok(ChordName::Gaug),
            "Gdim" | "GØ" | "G°" => Ok(ChordName::Gdim),

            "G#" | "Ab" | "G#maj" | "Abmaj" | "G#/Abmaj" | "G#M" | "AbM" | "G#/AbM" => {
                Ok(ChordName::GSharpAFlatmaj)
            }
            "G#m" | "Abm" | "G#/Abm" | "G#min" | "Abmin" | "G#/Abmin" | "G#-" | "Ab-"
            | "G#/Ab-" => Ok(ChordName::GSharpAFlatmin),
            "G#aug" | "G#+" | "Abaug" | "Ab+" | "G#/Ab+" | "G#/Abaug" => {
                Ok(ChordName::GSharpAFlataug)
            }
            "G#dim" | "G#Ø" | "G#°" | "Abdim" | "AbØ" | "Ab°" | "G#/Abdim" | "G#/AbØ"
            | "G#/Ab°" => Ok(ChordName::GSharpAFlatdim),

            "A" | "Amaj" | "AM" => Ok(ChordName::Amaj),
            "Am" | "Amin" | "A-" => Ok(ChordName::Amin),
            "Aaug" | "A+" => Ok(ChordName::Aaug),
            "Adim" | "AØ" | "A°" => Ok(ChordName::Adim),

            "A#" | "Bb" | "A#maj" | "Bbmaj" | "A#/Bbmaj" | "A#M" | "BbM" | "A#/BbM" => {
                Ok(ChordName::ASharpBFlatmaj)
            }
            "A#m" | "Bbm" | "A#/Bbm" | "A#min" | "Bbmin" | "A#/Bbmin" | "A#-" | "Bb-"
            | "A#/Bb-" => Ok(ChordName::ASharpBFlatmin),
            "A#aug" | "A#+" | "Bbaug" | "Bb+" | "A#/Bb+" | "A#/Bbaug" => {
                Ok(ChordName::ASharpBFlataug)
            }
            "A#dim" | "A#Ø" | "A#°" | "Bbdim" | "BbØ" | "Bb°" | "A#/Bbdim" | "A#/BbØ"
            | "A#/Bb°" => Ok(ChordName::ASharpBFlatdim),

            "B" | "Bmaj" | "BM" => Ok(ChordName::Bmaj),
            "Bm" | "Bmin" | "B-" => Ok(ChordName::Bmin),
            "Baug" | "B+" => Ok(ChordName::Baug),
            "Bdim" | "BØ" | "B°" => Ok(ChordName::Bdim),

            unknown => Err(ChordNameError(unknown.to_string())),
        }
    }
}

impl Display for ChordName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChordName::Cmaj => write!(f, "C"),
            ChordName::Cmin => write!(f, "Cmin"),
            ChordName::Caug => write!(f, "C+"),
            ChordName::Cdim => write!(f, "C°"),
            ChordName::CSharpDFlatmaj => write!(f, "C#/Db"),
            ChordName::CSharpDFlatmin => write!(f, "C#/Dbmin"),
            ChordName::CSharpDFlataug => write!(f, "C#/Db+"),
            ChordName::CSharpDFlatdim => write!(f, "C#/Db°"),
            ChordName::Dmaj => write!(f, "D"),
            ChordName::Dmin => write!(f, "Dmin"),
            ChordName::Daug => write!(f, "D+"),
            ChordName::Ddim => write!(f, "D°"),
            ChordName::DSharpEFlatmaj => write!(f, "D#/Eb"),
            ChordName::DSharpEFlatmin => write!(f, "D#/Ebmin"),
            ChordName::DSharpEFlataug => write!(f, "D#/Eb+"),
            ChordName::DSharpEFlatdim => write!(f, "D#/Eb°"),
            ChordName::Emaj => write!(f, "E"),
            ChordName::Emin => write!(f, "Emin"),
            ChordName::Eaug => write!(f, "E+"),
            ChordName::Edim => write!(f, "#°"),
            ChordName::Fmaj => write!(f, "F"),
            ChordName::Fmin => write!(f, "Fmin"),
            ChordName::Faug => write!(f, "F+"),
            ChordName::Fdim => write!(f, "F°"),
            ChordName::FSharpGFlatmaj => write!(f, "F#/Gb"),
            ChordName::FSharpGFlatmin => write!(f, "F#/Gbmin"),
            ChordName::FSharpGFlataug => write!(f, "F#/Gb+"),
            ChordName::FSharpGFlatdim => write!(f, "F#/Gb°"),
            ChordName::Gmaj => write!(f, "G"),
            ChordName::Gmin => write!(f, "Gmin"),
            ChordName::Gaug => write!(f, "G+"),
            ChordName::Gdim => write!(f, "G°"),
            ChordName::GSharpAFlatmaj => write!(f, "G#/Ab"),
            ChordName::GSharpAFlatmin => write!(f, "G#/Abmin"),
            ChordName::GSharpAFlataug => write!(f, "G#/Ab+"),
            ChordName::GSharpAFlatdim => write!(f, "G#/Ab°"),
            ChordName::Amaj => write!(f, "A"),
            ChordName::Amin => write!(f, "Amin"),
            ChordName::Aaug => write!(f, "A+"),
            ChordName::Adim => write!(f, "A°"),
            ChordName::ASharpBFlatmaj => write!(f, "A#/Bb"),
            ChordName::ASharpBFlatmin => write!(f, "A#/Bbmin"),
            ChordName::ASharpBFlataug => write!(f, "A#/Bb+"),
            ChordName::ASharpBFlatdim => write!(f, "A#/Bb°"),
            ChordName::Bmaj => write!(f, "B"),
            ChordName::Bmin => write!(f, "Bmin"),
            ChordName::Baug => write!(f, "B+"),
            ChordName::Bdim => write!(f, "B°"),
        }
    }
}

pub fn generate_enharmonics() -> HashMap<&'static str, &'static str> {
    HashMap::from([
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
    ])
}

pub fn generate_chord_spellings() -> (
    HashMap<&'static str, Vec<&'static str>>,
    HashMap<ChordName, Vec<NoteName>>,
) {
    let input = include_str!(".././chord_spellings.txt");

    let mut spellings = HashMap::<&'static str, Vec<&'static str>>::new();
    let mut typed_spellings: HashMap<ChordName, Vec<NoteName>> = HashMap::new();

    for line in input.lines() {
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

            spellings.insert(name, notes);
            typed_spellings.insert(typed_name, typed_notes);
        }
    }

    (spellings, typed_spellings)

    // for spelling in spellings {
    //     println!("{} : {:?}", spelling.0, spelling.1);
    // }

    // println!("\n-------------------------------\n TYPED SPELLINGS: \n");
    // for spelling in typed_spellings {
    //     print!(
    //         "{}: {} - {} - {}\n",
    //         spelling.0, spelling.1[0], spelling.1[1], spelling.1[2]
    //     );
    // }
}
