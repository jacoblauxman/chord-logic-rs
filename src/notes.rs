use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
#[error("Error trying in conversion of note name `{0}` to `NoteName` enum variant (Note: expects capital chars for note name value)")]
pub struct NoteNameError(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteName {
    C,
    CSharpDFlat,
    D,
    DSharpEFlat,
    E,
    F,
    FSharpGFlat,
    G,
    GSharpAFlat,
    A,
    ASharpBFlat,
    B,
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
#[error("Error trying in conversion of note + octave name `{0}` to `NoteOct` enum variant (Note: for &str conversion - expects capital name + oct values ex: `C1` or `C#/Db4` or `Eb5`)")]
pub struct NoteOctError(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteOct {
    C(usize),
    CSharpDFlat(usize),
    D(usize),
    DSharpEFlat(usize),
    E(usize),
    F(usize),
    FSharpGFlat(usize),
    G(usize),
    GSharpAFlat(usize),
    A(usize),
    ASharpBFlat(usize),
    B(usize),
}

impl NoteOct {
    pub fn from_note(note_name: &NoteName, oct: usize) -> Self {
        match note_name {
            NoteName::C => NoteOct::C(oct),
            NoteName::CSharpDFlat => NoteOct::CSharpDFlat(oct),
            NoteName::D => NoteOct::D(oct),
            NoteName::DSharpEFlat => NoteOct::DSharpEFlat(oct),
            NoteName::E => NoteOct::E(oct),
            NoteName::F => NoteOct::F(oct),
            NoteName::FSharpGFlat => NoteOct::FSharpGFlat(oct),
            NoteName::G => NoteOct::G(oct),
            NoteName::GSharpAFlat => NoteOct::GSharpAFlat(oct),
            NoteName::A => NoteOct::A(oct),
            NoteName::ASharpBFlat => NoteOct::ASharpBFlat(oct),
            NoteName::B => NoteOct::B(oct),
        }
    }
}

impl Display for NoteOct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteOct::C(oct) => write!(f, "C{oct}"),
            NoteOct::CSharpDFlat(oct) => write!(f, "C#/Db{oct}"),
            NoteOct::D(oct) => write!(f, "D{oct}"),
            NoteOct::DSharpEFlat(oct) => write!(f, "D#/Eb{oct}"),
            NoteOct::E(oct) => write!(f, "E{oct}"),
            NoteOct::F(oct) => write!(f, "F{oct}"),
            NoteOct::FSharpGFlat(oct) => write!(f, "F#/Gb{oct}"),
            NoteOct::G(oct) => write!(f, "G{oct}"),
            NoteOct::GSharpAFlat(oct) => write!(f, "G#/Ab{oct}"),
            NoteOct::A(oct) => write!(f, "A{oct}"),
            NoteOct::ASharpBFlat(oct) => write!(f, "A#/Bb{oct}"),
            NoteOct::B(oct) => write!(f, "B{oct}"),
        }
    }
}

impl TryFrom<&str> for NoteOct {
    type Error = NoteOctError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "C1" => Ok(NoteOct::C(1)),
            "C#1" | "Db1" | "C#/Db1" => Ok(NoteOct::CSharpDFlat(1)),
            "D1" => Ok(NoteOct::D(1)),
            "D#1" | "Eb1" | "D#/Eb1" => Ok(NoteOct::DSharpEFlat(1)),
            "E1" => Ok(NoteOct::E(1)),
            "F1" => Ok(NoteOct::F(1)),
            "F#1" | "Gb1" | "F#/Gb1" => Ok(NoteOct::FSharpGFlat(1)),
            "G1" => Ok(NoteOct::G(1)),
            "G#1" | "Ab1" | "G#/Ab1" => Ok(NoteOct::GSharpAFlat(1)),
            "A1" => Ok(NoteOct::A(1)),
            "A#1" | "Bb1" | "A#/Bb1" => Ok(NoteOct::ASharpBFlat(1)),
            "B1" => Ok(NoteOct::B(1)),

            "C2" => Ok(NoteOct::C(2)),
            "C#2" | "Db2" | "C#/Db2" => Ok(NoteOct::CSharpDFlat(2)),
            "D2" => Ok(NoteOct::D(2)),
            "D#2" | "Eb2" | "D#/Eb2" => Ok(NoteOct::DSharpEFlat(2)),
            "E2" => Ok(NoteOct::E(2)),
            "F2" => Ok(NoteOct::F(2)),
            "F#2" | "Gb2" | "F#/Gb2" => Ok(NoteOct::FSharpGFlat(2)),
            "G2" => Ok(NoteOct::G(2)),
            "G#2" | "Ab2" | "G#/Ab2" => Ok(NoteOct::GSharpAFlat(2)),
            "A2" => Ok(NoteOct::A(2)),
            "A#2" | "Bb2" | "A#/Bb2" => Ok(NoteOct::ASharpBFlat(2)),
            "B2" => Ok(NoteOct::B(2)),

            "C3" => Ok(NoteOct::C(3)),
            "C#3" | "Db3" | "C#/Db3" => Ok(NoteOct::CSharpDFlat(3)),
            "D3" => Ok(NoteOct::D(3)),
            "D#3" | "Eb3" | "D#/Eb3" => Ok(NoteOct::DSharpEFlat(3)),
            "E3" => Ok(NoteOct::E(3)),
            "F3" => Ok(NoteOct::F(3)),
            "F#3" | "Gb3" | "F#/Gb3" => Ok(NoteOct::FSharpGFlat(3)),
            "G3" => Ok(NoteOct::G(3)),
            "G#3" | "Ab3" | "G#/Ab3" => Ok(NoteOct::GSharpAFlat(3)),
            "A3" => Ok(NoteOct::A(3)),
            "A#3" | "Bb3" | "A#/Bb3" => Ok(NoteOct::ASharpBFlat(3)),
            "B3" => Ok(NoteOct::B(3)),

            "C4" => Ok(NoteOct::C(4)),
            "C#4" | "Db4" | "C#/Db4" => Ok(NoteOct::CSharpDFlat(4)),
            "D4" => Ok(NoteOct::D(4)),
            "D#4" | "Eb4" | "D#/Eb4" => Ok(NoteOct::DSharpEFlat(4)),
            "E4" => Ok(NoteOct::E(4)),
            "F4" => Ok(NoteOct::F(4)),
            "F#4" | "Gb4" | "F#/Gb4" => Ok(NoteOct::FSharpGFlat(4)),
            "G4" => Ok(NoteOct::G(4)),
            "G#4" | "Ab4" | "G#/Ab4" => Ok(NoteOct::GSharpAFlat(4)),
            "A4" => Ok(NoteOct::A(4)),
            "A#4" | "Bb4" | "A#/Bb4" => Ok(NoteOct::ASharpBFlat(4)),
            "B4" => Ok(NoteOct::B(4)),

            "C5" => Ok(NoteOct::C(5)),
            "C#5" | "Db5" | "C#/Db5" => Ok(NoteOct::CSharpDFlat(5)),
            "D5" => Ok(NoteOct::D(5)),
            "D#5" | "Eb5" | "D#/Eb5" => Ok(NoteOct::DSharpEFlat(5)),
            "E5" => Ok(NoteOct::E(5)),
            "F5" => Ok(NoteOct::F(5)),
            "F#5" | "Gb5" | "F#/Gb5" => Ok(NoteOct::FSharpGFlat(5)),
            "G5" => Ok(NoteOct::G(5)),
            "G#5" | "Ab5" | "G#/Ab5" => Ok(NoteOct::GSharpAFlat(5)),
            "A5" => Ok(NoteOct::A(5)),
            "A#5" | "Bb5" | "A#/Bb5" => Ok(NoteOct::ASharpBFlat(5)),
            "B5" => Ok(NoteOct::B(5)),

            "C6" => Ok(NoteOct::C(6)),
            "C#6" | "Db6" | "C#/Db6" => Ok(NoteOct::CSharpDFlat(6)),
            "D6" => Ok(NoteOct::D(6)),
            "D#6" | "Eb6" | "D#/Eb6" => Ok(NoteOct::DSharpEFlat(6)),
            "E6" => Ok(NoteOct::E(6)),
            "F6" => Ok(NoteOct::F(6)),
            "F#6" | "Gb6" | "F#/Gb6" => Ok(NoteOct::FSharpGFlat(6)),
            "G6" => Ok(NoteOct::G(6)),
            "G#6" | "Ab6" | "G#/Ab6" => Ok(NoteOct::GSharpAFlat(6)),
            "A6" => Ok(NoteOct::A(6)),
            "A#6" | "Bb6" | "A#/Bb6" => Ok(NoteOct::ASharpBFlat(6)),
            "B6" => Ok(NoteOct::B(6)),

            "C7" => Ok(NoteOct::C(7)),
            "C#7" | "Db7" | "C#/Db7" => Ok(NoteOct::CSharpDFlat(7)),
            "D7" => Ok(NoteOct::D(7)),
            "D#7" | "Eb7" | "D#/Eb7" => Ok(NoteOct::DSharpEFlat(7)),
            "E7" => Ok(NoteOct::E(7)),
            "F7" => Ok(NoteOct::F(7)),
            "F#7" | "Gb7" | "F#/Gb7" => Ok(NoteOct::FSharpGFlat(7)),
            "G7" => Ok(NoteOct::G(7)),
            "G#7" | "Ab7" | "G#/Ab7" => Ok(NoteOct::GSharpAFlat(7)),
            "A7" => Ok(NoteOct::A(7)),
            "A#7" | "Bb7" | "A#/Bb7" => Ok(NoteOct::ASharpBFlat(7)),
            "B7" => Ok(NoteOct::B(7)),

            "C8" => Ok(NoteOct::C(8)),
            "C#8" | "Db8" | "C#/Db8" => Ok(NoteOct::CSharpDFlat(8)),
            "D8" => Ok(NoteOct::D(8)),
            "D#8" | "Eb8" | "D#/Eb8" => Ok(NoteOct::DSharpEFlat(8)),
            "E8" => Ok(NoteOct::E(8)),
            "F8" => Ok(NoteOct::F(8)),
            "F#8" | "Gb8" | "F#/Gb8" => Ok(NoteOct::FSharpGFlat(8)),
            "G8" => Ok(NoteOct::G(8)),
            "G#8" | "Ab8" | "G#/Ab8" => Ok(NoteOct::GSharpAFlat(8)),
            "A8" => Ok(NoteOct::A(8)),
            "A#8" | "Bb8" | "A#/Bb8" => Ok(NoteOct::ASharpBFlat(8)),
            "B8" => Ok(NoteOct::B(8)),

            unknown => Err(NoteOctError(unknown.to_string())),
        }
    }
}
