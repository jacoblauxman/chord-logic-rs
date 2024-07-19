use crate::NoteName;
use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
#[error("Error trying in conversion of `&str` chord name `{0}` to `ChordName` enum variant \n(Note: all chords spelled with capitalized `name` letter + various common quality symbols and no spaces [ex: `C`, `C#°` or `C#/Dbdim`, `Db+`, etc.]")]
pub struct ChordNameError(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordQuality {
    Maj,
    Min,
    Aug,
    Dim,
    Sus2,
    Sus4,
    MajSev,
    MinSev,
    Sev,
    SevSus,
}

impl Display for ChordQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChordQuality::Maj => write!(f, "maj"),
            ChordQuality::Min => write!(f, "min"),
            ChordQuality::Aug => write!(f, "+"),
            ChordQuality::Dim => write!(f, "°"),
            ChordQuality::Sus2 => write!(f, "sus2"),
            ChordQuality::Sus4 => write!(f, "sus4"),
            ChordQuality::MajSev => write!(f, "maj7"),
            ChordQuality::MinSev => write!(f, "min7"),
            ChordQuality::Sev => write!(f, "7"),
            ChordQuality::SevSus => write!(f, "7sus4"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordName {
    C(ChordQuality),
    CSharpDFlat(ChordQuality),
    D(ChordQuality),
    DSharpEFlat(ChordQuality),
    E(ChordQuality),
    F(ChordQuality),
    FSharpGFlat(ChordQuality),
    G(ChordQuality),
    GSharpAFlat(ChordQuality),
    A(ChordQuality),
    ASharpBFlat(ChordQuality),
    B(ChordQuality),
}

impl ChordName {
    pub fn get_root(&self) -> NoteName {
        match self {
            ChordName::C(_) => NoteName::C,
            ChordName::CSharpDFlat(_) => NoteName::CSharpDFlat,
            ChordName::D(_) => NoteName::D,
            ChordName::DSharpEFlat(_) => NoteName::DSharpEFlat,
            ChordName::E(_) => NoteName::E,
            ChordName::F(_) => NoteName::F,
            ChordName::FSharpGFlat(_) => NoteName::FSharpGFlat,
            ChordName::G(_) => NoteName::G,
            ChordName::GSharpAFlat(_) => NoteName::GSharpAFlat,
            ChordName::A(_) => NoteName::A,
            ChordName::ASharpBFlat(_) => NoteName::ASharpBFlat,
            ChordName::B(_) => NoteName::B,
        }
    }

    pub fn get_quality(&self) -> ChordQuality {
        match self {
            ChordName::C(qual) => *qual,
            ChordName::CSharpDFlat(qual) => *qual,
            ChordName::D(qual) => *qual,
            ChordName::DSharpEFlat(qual) => *qual,
            ChordName::E(qual) => *qual,
            ChordName::F(qual) => *qual,
            ChordName::FSharpGFlat(qual) => *qual,
            ChordName::G(qual) => *qual,
            ChordName::GSharpAFlat(qual) => *qual,
            ChordName::A(qual) => *qual,
            ChordName::ASharpBFlat(qual) => *qual,
            ChordName::B(qual) => *qual,
        }
    }
}

impl Display for ChordName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChordName::C(qual) => write!(f, "C{}", qual),
            ChordName::CSharpDFlat(qual) => write!(f, "C#/Db{}", qual),
            ChordName::D(qual) => write!(f, "D{}", qual),
            ChordName::DSharpEFlat(qual) => write!(f, "D#/Eb{}", qual),
            ChordName::E(qual) => write!(f, "E{}", qual),
            ChordName::F(qual) => write!(f, "F{}", qual),
            ChordName::FSharpGFlat(qual) => write!(f, "F#/Gb{}", qual),
            ChordName::G(qual) => write!(f, "G{}", qual),
            ChordName::GSharpAFlat(qual) => write!(f, "G#/Ab{}", qual),
            ChordName::A(qual) => write!(f, "A{}", qual),
            ChordName::ASharpBFlat(qual) => write!(f, "A#/Bb{}", qual),
            ChordName::B(qual) => write!(f, "B{}", qual),
        }
    }
}

impl TryFrom<&str> for ChordName {
    type Error = ChordNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "C" | "Cmaj" | "CM" => Ok(ChordName::C(ChordQuality::Maj)),
            "Cm" | "Cmin" | "C-" => Ok(ChordName::C(ChordQuality::Min)),
            "Caug" | "C+" => Ok(ChordName::C(ChordQuality::Aug)),
            "Cdim" | "CØ" | "C°" => Ok(ChordName::C(ChordQuality::Dim)),

            "Csus2" | "Cs2" => Ok(ChordName::C(ChordQuality::Sus2)),
            "Csus4" | "Csus" | "Cs4" => Ok(ChordName::C(ChordQuality::Sus4)),
            "Cmaj7" | "CM7" | "C^7" => Ok(ChordName::C(ChordQuality::MajSev)),
            "Cmin7" | "Cm7" | "C-7" => Ok(ChordName::C(ChordQuality::MinSev)),
            "C7" | "Cdom7" => Ok(ChordName::C(ChordQuality::Sev)),
            "C7sus4" | "C7s4" | "C7s11" | "C11" | "Csus11" => {
                Ok(ChordName::C(ChordQuality::SevSus))
            }

            "C#" | "Db" | "C#maj" | "Dbmaj" | "C#/Dbmaj" | "C#M" | "DbM" | "C#/DbM" => {
                Ok(ChordName::CSharpDFlat(ChordQuality::Maj))
            }
            "C#m" | "Dbm" | "C#/Dbm" | "C#min" | "Dbmin" | "C#/Dbmin" | "C#-" | "Db-"
            | "C#/Db-" => Ok(ChordName::CSharpDFlat(ChordQuality::Min)),
            "C#aug" | "C#+" | "Dbaug" | "Db+" | "C#/Db+" | "C#/Dbaug" => {
                Ok(ChordName::CSharpDFlat(ChordQuality::Aug))
            }
            "C#dim" | "C#Ø" | "C#°" | "Dbdim" | "DbØ" | "Db°" | "C#/Dbdim" | "C#/DbØ"
            | "C#/Db°" => Ok(ChordName::CSharpDFlat(ChordQuality::Dim)),

            "C#sus2" | "C#s2" | "C#/Dbsus2" | "C#/Dbs2" | "Dbsus2" | "Dbs2" => {
                Ok(ChordName::CSharpDFlat(ChordQuality::Sus2))
            }
            "C#sus4" | "C#sus" | "C#s4" | "C#/Dbsus4" | "C#/Dbsus" | "C#/Dbs4" | "Dbsus4"
            | "Dbs4" | "Dbsus" => Ok(ChordName::CSharpDFlat(ChordQuality::Sus4)),
            "C#maj7" | "C#M7" | "C#^7" | "C#/Dbmaj7" | "C#/DbM7" | "C#/Db^7" | "Dbmaj7"
            | "DbM7" | "Db^7" => Ok(ChordName::CSharpDFlat(ChordQuality::MajSev)),
            "C#min7" | "C#m7" | "C#-7" | "C#/Dbmin7" | "C#/Dbm7" | "C#/Db-7" | "Dbmin7"
            | "Dbm7" | "Db-7" => Ok(ChordName::CSharpDFlat(ChordQuality::MinSev)),
            "C#7" | "C#dom7" | "C#/Db7" | "C#/Dbdom7" | "Db7" | "Dbdom7" => {
                Ok(ChordName::CSharpDFlat(ChordQuality::Sev))
            }

            "C#7sus4" | "C#7s4" | "C#7s11" | "C#11" | "C#sus11" | "C#/Db7sus4" | "C#/Db7s4"
            | "C#/Db7s11" | "C#/Db11" | "C#/Dbsus11" | "Db7sus4" | "Db7s4" | "Db7s11" | "Db11"
            | "Dbsus11" => Ok(ChordName::CSharpDFlat(ChordQuality::SevSus)),

            "D" | "Dmaj" | "DM" => Ok(ChordName::D(ChordQuality::Maj)),
            "Dm" | "Dmin" | "D-" => Ok(ChordName::D(ChordQuality::Min)),
            "Daug" | "D+" => Ok(ChordName::D(ChordQuality::Aug)),
            "Ddim" | "DØ" | "D°" => Ok(ChordName::D(ChordQuality::Dim)),

            "Dsus2" | "Ds2" => Ok(ChordName::D(ChordQuality::Sus2)),
            "Dsus4" | "Dsus" | "Ds4" => Ok(ChordName::D(ChordQuality::Sus4)),
            "Dmaj7" | "DM7" | "D^7" => Ok(ChordName::D(ChordQuality::MajSev)),
            "Dmin7" | "Dm7" | "D-7" => Ok(ChordName::D(ChordQuality::MinSev)),
            "D7" | "Ddom7" => Ok(ChordName::D(ChordQuality::Sev)),
            "D7sus4" | "D7s4" | "D7s11" | "D11" | "Dsus11" => {
                Ok(ChordName::D(ChordQuality::SevSus))
            }

            "D#" | "Eb" | "D#maj" | "Ebmaj" | "D#/Ebmaj" | "D#M" | "EbM" | "D#/EbM" => {
                Ok(ChordName::DSharpEFlat(ChordQuality::Maj))
            }
            "D#m" | "Ebm" | "D#/Ebm" | "D#min" | "Ebmin" | "D#/Ebmin" | "D#-" | "Eb-"
            | "D#/Eb-" => Ok(ChordName::DSharpEFlat(ChordQuality::Min)),
            "D#aug" | "D#+" | "Ebaug" | "Eb+" | "D#/Eb+" | "D#/Ebaug" => {
                Ok(ChordName::DSharpEFlat(ChordQuality::Aug))
            }
            "D#dim" | "D#Ø" | "D#°" | "Ebdim" | "EbØ" | "Eb°" | "D#/Ebdim" | "D#/EbØ"
            | "D#/Eb°" => Ok(ChordName::DSharpEFlat(ChordQuality::Dim)),

            "D#sus2" | "D#s2" | "D#/Ebsus2" | "D#/Ebs2" | "Ebsus2" | "Ebs2" => {
                Ok(ChordName::DSharpEFlat(ChordQuality::Sus2))
            }
            "D#sus4" | "D#sus" | "D#s4" | "D#/Ebsus4" | "D#/Ebsus" | "D#/Ebs4" | "Ebsus4"
            | "Ebs4" | "Ebsus" => Ok(ChordName::DSharpEFlat(ChordQuality::Sus4)),
            "D#maj7" | "D#M7" | "D#^7" | "D#/Ebmaj7" | "D#/EbM7" | "D#/Eb^7" | "Ebmaj7"
            | "EbM7" | "Eb^7" => Ok(ChordName::DSharpEFlat(ChordQuality::MajSev)),
            "D#min7" | "D#m7" | "D#-7" | "D#/Ebmin7" | "D#/Ebm7" | "D#/Eb-7" | "Ebmin7"
            | "Ebm7" | "Eb-7" => Ok(ChordName::DSharpEFlat(ChordQuality::MinSev)),
            "D#7" | "D#dom7" | "D#/Eb7" | "D#/Ebdom7" | "Eb7" | "Ebdom7" => {
                Ok(ChordName::DSharpEFlat(ChordQuality::Sev))
            }

            "D#7sus4" | "D#7s4" | "D#7s11" | "D#11" | "D#sus11" | "D#/Eb7sus4" | "D#/Eb7s4"
            | "D#/Eb7s11" | "D#/Eb11" | "D#/Ebsus11" | "Eb7sus4" | "Eb7s4" | "Eb7s11" | "Eb11"
            | "Ebsus11" => Ok(ChordName::DSharpEFlat(ChordQuality::SevSus)),

            "E" | "Emaj" | "EM" => Ok(ChordName::E(ChordQuality::Maj)),
            "Em" | "Emin" | "E-" => Ok(ChordName::E(ChordQuality::Min)),
            "Eaug" | "E+" => Ok(ChordName::E(ChordQuality::Aug)),
            "Edim" | "EØ" | "E°" => Ok(ChordName::E(ChordQuality::Dim)),

            "Esus2" | "Es2" => Ok(ChordName::E(ChordQuality::Sus2)),
            "Esus4" | "Esus" | "Es4" => Ok(ChordName::E(ChordQuality::Sus4)),
            "Emaj7" | "EM7" | "E^7" => Ok(ChordName::E(ChordQuality::MajSev)),
            "Emin7" | "Em7" | "E-7" => Ok(ChordName::E(ChordQuality::MinSev)),
            "E7" | "Edom7" => Ok(ChordName::E(ChordQuality::Sev)),
            "E7sus4" | "E7s4" | "E7s11" | "E11" | "Esus11" => {
                Ok(ChordName::E(ChordQuality::SevSus))
            }

            "F" | "Fmaj" | "FM" => Ok(ChordName::F(ChordQuality::Maj)),
            "Fm" | "Fmin" | "F-" => Ok(ChordName::F(ChordQuality::Min)),
            "Faug" | "F+" => Ok(ChordName::F(ChordQuality::Aug)),
            "Fdim" | "FØ" | "F°" => Ok(ChordName::F(ChordQuality::Dim)),

            "Fsus2" | "Fs2" => Ok(ChordName::F(ChordQuality::Sus2)),
            "Fsus4" | "Fsus" | "Fs4" => Ok(ChordName::F(ChordQuality::Sus4)),
            "Fmaj7" | "FM7" | "F^7" => Ok(ChordName::F(ChordQuality::MajSev)),
            "Fmin7" | "Fm7" | "F-7" => Ok(ChordName::F(ChordQuality::MinSev)),
            "F7" | "Fdom7" => Ok(ChordName::F(ChordQuality::Sev)),
            "F7sus4" | "F7s4" | "F7s11" | "F11" | "Fsus11" => {
                Ok(ChordName::F(ChordQuality::SevSus))
            }

            "F#" | "Gb" | "F#maj" | "Gbmaj" | "F#/Gbmaj" | "F#M" | "GbM" | "F#/GbM" => {
                Ok(ChordName::FSharpGFlat(ChordQuality::Maj))
            }
            "F#m" | "Gbm" | "F#/Gbm" | "F#min" | "Gbmin" | "F#/Gbmin" | "F#-" | "Gb-"
            | "F#/Gb-" => Ok(ChordName::FSharpGFlat(ChordQuality::Min)),
            "F#aug" | "F#+" | "Gbaug" | "Gb+" | "F#/Gb+" | "F#/Gbaug" => {
                Ok(ChordName::FSharpGFlat(ChordQuality::Aug))
            }
            "F#dim" | "F#Ø" | "F#°" | "Gbdim" | "GbØ" | "Gb°" | "F#/Gbdim" | "F#/GbØ"
            | "F#/Gb°" => Ok(ChordName::FSharpGFlat(ChordQuality::Dim)),

            "F#sus2" | "F#s2" | "F#/Gbsus2" | "F#/Gbs2" | "Gbsus2" | "Gbs2" => {
                Ok(ChordName::FSharpGFlat(ChordQuality::Sus2))
            }
            "F#sus4" | "F#sus" | "F#s4" | "F#/Gbsus4" | "F#/Gbsus" | "F#/Gbs4" | "Gbsus4"
            | "Gbs4" | "Gbsus" => Ok(ChordName::FSharpGFlat(ChordQuality::Sus4)),
            "F#maj7" | "F#M7" | "F#^7" | "F#/Gbmaj7" | "F#/GbM7" | "F#/Gb^7" | "Gbmaj7"
            | "GbM7" | "Gb^7" => Ok(ChordName::FSharpGFlat(ChordQuality::MajSev)),
            "F#min7" | "F#m7" | "F#-7" | "F#/Gbmin7" | "F#/Gbm7" | "F#/Gb-7" | "Gbmin7"
            | "Gbm7" | "Gb-7" => Ok(ChordName::FSharpGFlat(ChordQuality::MinSev)),
            "F#7" | "F#dom7" | "F#/Gb7" | "F#/Gbdom7" | "Gb7" | "Gbdom7" => {
                Ok(ChordName::FSharpGFlat(ChordQuality::Sev))
            }
            "F#7sus4" | "F#7s4" | "F#7s11" | "F#11" | "F#sus11" | "F#/Gb7sus4" | "F#/Gb7s4"
            | "F#/Gb7s11" | "F#/Gb11" | "F#/Gbsus11" | "Gb7sus4" | "Gb7s4" | "Gb7s11" | "Gb11"
            | "Gbsus11" => Ok(ChordName::FSharpGFlat(ChordQuality::SevSus)),

            "G" | "Gmaj" | "GM" => Ok(ChordName::G(ChordQuality::Maj)),
            "Gm" | "Gmin" | "G-" => Ok(ChordName::G(ChordQuality::Min)),
            "Gaug" | "G+" => Ok(ChordName::G(ChordQuality::Aug)),
            "Gdim" | "GØ" | "G°" => Ok(ChordName::G(ChordQuality::Dim)),

            "Gsus2" | "Gs2" => Ok(ChordName::G(ChordQuality::Sus2)),
            "Gsus4" | "Gsus" | "Gs4" => Ok(ChordName::G(ChordQuality::Sus4)),
            "Gmaj7" | "GM7" | "G^7" => Ok(ChordName::G(ChordQuality::MajSev)),
            "Gmin7" | "Gm7" | "G-7" => Ok(ChordName::G(ChordQuality::MinSev)),
            "G7" | "Gdom7" => Ok(ChordName::G(ChordQuality::Sev)),
            "G7sus4" | "G7s4" | "G7s11" | "G11" | "Gsus11" => {
                Ok(ChordName::G(ChordQuality::SevSus))
            }

            "G#" | "Ab" | "G#maj" | "Abmaj" | "G#/Abmaj" | "G#M" | "AbM" | "G#/AbM" => {
                Ok(ChordName::GSharpAFlat(ChordQuality::Maj))
            }
            "G#m" | "Abm" | "G#/Abm" | "G#min" | "Abmin" | "G#/Abmin" | "G#-" | "Ab-"
            | "G#/Ab-" => Ok(ChordName::GSharpAFlat(ChordQuality::Min)),
            "G#aug" | "G#+" | "Abaug" | "Ab+" | "G#/Ab+" | "G#/Abaug" => {
                Ok(ChordName::GSharpAFlat(ChordQuality::Aug))
            }
            "G#dim" | "G#Ø" | "G#°" | "Abdim" | "AbØ" | "Ab°" | "G#/Abdim" | "G#/AbØ"
            | "G#/Ab°" => Ok(ChordName::GSharpAFlat(ChordQuality::Dim)),

            "G#sus2" | "G#s2" | "G#/Absus2" | "G#/Abs2" | "Absus2" | "Abs2" => {
                Ok(ChordName::GSharpAFlat(ChordQuality::Sus2))
            }
            "G#sus4" | "G#sus" | "G#s4" | "G#/Absus4" | "G#/Absus" | "G#/Abs4" | "Absus4"
            | "Abs4" | "Absus" => Ok(ChordName::GSharpAFlat(ChordQuality::Sus4)),
            "G#maj7" | "G#M7" | "G#^7" | "G#/Abmaj7" | "G#/AbM7" | "G#/Ab^7" | "Abmaj7"
            | "AbM7" | "Ab^7" => Ok(ChordName::GSharpAFlat(ChordQuality::MajSev)),
            "G#min7" | "G#m7" | "G#-7" | "G#/Abmin7" | "G#/Abm7" | "G#/Ab-7" | "Abmin7"
            | "Abm7" | "Ab-7" => Ok(ChordName::GSharpAFlat(ChordQuality::MinSev)),
            "G#7" | "G#dom7" | "G#/Ab7" | "G#/Abdom7" | "Ab7" | "Abdom7" => {
                Ok(ChordName::GSharpAFlat(ChordQuality::Sev))
            }
            "G#7sus4" | "G#7s4" | "G#7s11" | "G#11" | "G#sus11" | "G#/Ab7sus4" | "G#/Ab7s4"
            | "G#/Ab7s11" | "G#/Ab11" | "G#/Absus11" | "Ab7sus4" | "Ab7s4" | "Ab7s11" | "Ab11"
            | "Absus11" => Ok(ChordName::GSharpAFlat(ChordQuality::SevSus)),

            "A" | "Amaj" | "AM" => Ok(ChordName::A(ChordQuality::Maj)),
            "Am" | "Amin" | "A-" => Ok(ChordName::A(ChordQuality::Min)),
            "Aaug" | "A+" => Ok(ChordName::A(ChordQuality::Aug)),
            "Adim" | "AØ" | "A°" => Ok(ChordName::A(ChordQuality::Dim)),

            "Asus2" | "As2" => Ok(ChordName::A(ChordQuality::Sus2)),
            "Asus4" | "Asus" | "As4" => Ok(ChordName::A(ChordQuality::Sus4)),
            "Amaj7" | "AM7" | "A^7" => Ok(ChordName::A(ChordQuality::MajSev)),
            "Amin7" | "Am7" | "A-7" => Ok(ChordName::A(ChordQuality::MinSev)),
            "A7" | "Adom7" => Ok(ChordName::A(ChordQuality::Sev)),
            "A7sus4" | "A7s4" | "A7s11" | "A11" | "Asus11" => {
                Ok(ChordName::A(ChordQuality::SevSus))
            }

            "A#" | "Bb" | "A#maj" | "Bbmaj" | "A#/Bbmaj" | "A#M" | "BbM" | "A#/BbM" => {
                Ok(ChordName::ASharpBFlat(ChordQuality::Maj))
            }
            "A#m" | "Bbm" | "A#/Bbm" | "A#min" | "Bbmin" | "A#/Bbmin" | "A#-" | "Bb-"
            | "A#/Bb-" => Ok(ChordName::ASharpBFlat(ChordQuality::Min)),
            "A#aug" | "A#+" | "Bbaug" | "Bb+" | "A#/Bb+" | "A#/Bbaug" => {
                Ok(ChordName::ASharpBFlat(ChordQuality::Aug))
            }
            "A#dim" | "A#Ø" | "A#°" | "Bbdim" | "BbØ" | "Bb°" | "A#/Bbdim" | "A#/BbØ"
            | "A#/Bb°" => Ok(ChordName::ASharpBFlat(ChordQuality::Dim)),

            "A#sus2" | "A#s2" | "A#/Bbsus2" | "A#/Bbs2" | "Bbsus2" | "Bbs2" => {
                Ok(ChordName::ASharpBFlat(ChordQuality::Sus2))
            }
            "A#sus4" | "A#sus" | "A#s4" | "A#/Bbsus4" | "A#/Bbsus" | "A#/Bbs4" | "Bbsus4"
            | "Bbs4" | "Bbsus" => Ok(ChordName::ASharpBFlat(ChordQuality::Sus4)),
            "A#maj7" | "A#M7" | "A#^7" | "A#/Bbmaj7" | "A#/BbM7" | "A#/Bb^7" | "Bbmaj7"
            | "BbM7" | "Bb^7" => Ok(ChordName::ASharpBFlat(ChordQuality::MajSev)),
            "A#min7" | "A#m7" | "A#-7" | "A#/Bbmin7" | "A#/Bbm7" | "A#/Bb-7" | "Bbmin7"
            | "Bbm7" | "Bb-7" => Ok(ChordName::ASharpBFlat(ChordQuality::MinSev)),
            "A#7" | "A#dom7" | "A#/Bb7" | "A#/Bbdom7" | "Bb7" | "Bbdom7" => {
                Ok(ChordName::ASharpBFlat(ChordQuality::Sev))
            }
            "A#7sus4" | "A#7s4" | "A#7s11" | "A#11" | "A#sus11" | "A#/Bb7sus4" | "A#/Bb7s4"
            | "A#/Bb7s11" | "A#/Bb11" | "A#/Bbsus11" | "Bb7sus4" | "Bb7s4" | "Bb7s11" | "Bb11"
            | "Bbsus11" => Ok(ChordName::ASharpBFlat(ChordQuality::SevSus)),

            "B" | "Bmaj" | "BM" => Ok(ChordName::B(ChordQuality::Maj)),
            "Bm" | "Bmin" | "B-" => Ok(ChordName::B(ChordQuality::Min)),
            "Baug" | "B+" => Ok(ChordName::B(ChordQuality::Aug)),
            "Bdim" | "BØ" | "B°" => Ok(ChordName::B(ChordQuality::Dim)),

            "Bsus2" | "Bs2" => Ok(ChordName::B(ChordQuality::Sus2)),
            "Bsus4" | "Bsus" | "Bs4" => Ok(ChordName::B(ChordQuality::Sus4)),
            "Bmaj7" | "BM7" | "B^7" => Ok(ChordName::B(ChordQuality::MajSev)),
            "Bmin7" | "Bm7" | "B-7" => Ok(ChordName::B(ChordQuality::MinSev)),
            "B7" | "Bdom7" => Ok(ChordName::A(ChordQuality::Sev)),
            "B7sus4" | "B7s4" | "B7s11" | "B11" | "Bsus11" => {
                Ok(ChordName::B(ChordQuality::SevSus))
            }

            unknown => Err(ChordNameError(unknown.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordTone {
    Root(NoteName),
    Second(NoteName),
    Third(NoteName),
    Fourth(NoteName),
    Fifth(NoteName),
    Seventh(NoteName),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordToneDegree {
    Root,
    Second,
    Third,
    Fourth,
    Fifth,
    Seventh,
}

impl ChordTone {
    pub fn note(&self) -> &NoteName {
        match self {
            ChordTone::Root(note) => note,
            ChordTone::Second(note) => note,
            ChordTone::Third(note) => note,
            ChordTone::Fourth(note) => note,
            ChordTone::Fifth(note) => note,
            ChordTone::Seventh(note) => note,
        }
    }

    pub fn get_tone_degree(&self) -> ChordToneDegree {
        match self {
            ChordTone::Root(_) => ChordToneDegree::Root,
            ChordTone::Second(_) => ChordToneDegree::Second,
            ChordTone::Third(_) => ChordToneDegree::Third,
            ChordTone::Fourth(_) => ChordToneDegree::Fourth,
            ChordTone::Fifth(_) => ChordToneDegree::Fifth,
            ChordTone::Seventh(_) => ChordToneDegree::Seventh,
        }
    }
}

impl Display for ChordTone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChordTone::Root(note) => write!(f, "Root: {note}"),
            ChordTone::Second(note) => write!(f, "Second: {note}"),
            ChordTone::Third(note) => write!(f, "Third: {note}"),
            ChordTone::Fourth(note) => write!(f, "Fourth: {note}"),
            ChordTone::Fifth(note) => write!(f, "Fifth: {note}"),
            ChordTone::Seventh(note) => write!(f, "Seventh: {note}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChordSpelling {
    name: ChordName,
    spelling: Vec<ChordTone>,
}

impl ChordSpelling {
    pub fn new(root: &NoteName, quality: &ChordQuality, notes: &Vec<NoteName>) -> Self {
        let name = create_chord_name(&root, &quality);

        let mut spelling = Vec::with_capacity(3);
        spelling.push(ChordTone::Root(*root));

        match quality {
            ChordQuality::Maj | ChordQuality::Min | ChordQuality::Aug | ChordQuality::Dim => {
                spelling.push(ChordTone::Third(notes[1]));
                spelling.push(ChordTone::Fifth(notes[2]));
            }
            ChordQuality::Sus2 => {
                spelling.push(ChordTone::Second(notes[1]));
                spelling.push(ChordTone::Fifth(notes[2]));
            }
            ChordQuality::Sus4 => {
                spelling.push(ChordTone::Fourth(notes[2]));
                spelling.push(ChordTone::Fifth(notes[2]));
            }
            ChordQuality::MajSev | ChordQuality::MinSev | ChordQuality::Sev => {
                spelling.push(ChordTone::Third(notes[2]));
                spelling.push(ChordTone::Seventh(notes[2]));
            }
            ChordQuality::SevSus => {
                spelling.push(ChordTone::Fourth(notes[2]));
                spelling.push(ChordTone::Seventh(notes[2]));
            }
        }

        Self { name, spelling }
    }

    pub fn name(&self) -> &ChordName {
        &self.name
    }

    pub fn spelling(&self) -> &Vec<ChordTone> {
        &self.spelling
    }
}

impl Display for ChordSpelling {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Spelling for {}:\n{} -- {} -- {}\n",
            self.name, self.spelling[0], self.spelling[1], self.spelling[2]
        )
    }
}

fn create_chord_name(root: &NoteName, quality: &ChordQuality) -> ChordName {
    let name = root.to_string() + &quality.to_string();

    ChordName::try_from(name.as_str()).expect(
        "`create_chord_name` should create valid `ChordName` with given `root` + `quality` parts",
    )
}
