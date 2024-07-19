use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
#[error("Error trying in conversion of `&str` chord name `{0}` to `ChordName` enum variant \n(Note: all chords spelled with capitalized `name` letter + various common quality symbols and no spaces [ex: `C`, `C#°` or `C#/Dbdim`, `Db+`, etc.]")]
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

// consider display to show "[NoteName] [Quality - fully spelled]" re: "free" `toString` implementation and later 'translating' to `Chord` type struct
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
