use crate::NoteName;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleQuality {
    Maj,
    Min,
    HarmMin,
    MelMin,
    MajPent,
    MinPent,
}

impl Display for ScaleQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScaleQuality::Maj => write!(f, "major"),
            ScaleQuality::MajPent => write!(f, "major pentatonic"),
            ScaleQuality::Min => write!(f, "minor"),
            ScaleQuality::MinPent => write!(f, "minor pentatonic"),
            ScaleQuality::HarmMin => write!(f, "harmonic minor"),
            ScaleQuality::MelMin => write!(f, "melodic minor"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ScaleNameError(String);

pub enum ScaleName {
    C(ScaleQuality),
    CSharpDFlat(ScaleQuality),
    D(ScaleQuality),
    DSharpEFlat(ScaleQuality),
    E(ScaleQuality),
    F(ScaleQuality),
    FSharpGFlat(ScaleQuality),
    G(ScaleQuality),
    GSharpAFlat(ScaleQuality),
    A(ScaleQuality),
    ASharpBFlat(ScaleQuality),
    B(ScaleQuality),
}

impl Display for ScaleName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScaleName::C(qual) => write!(f, "C {}", qual),
            ScaleName::CSharpDFlat(qual) => write!(f, "C#/Db {}", qual),
            ScaleName::D(qual) => write!(f, "D {}", qual),
            ScaleName::DSharpEFlat(qual) => write!(f, "D#/Eb {}", qual),
            ScaleName::E(qual) => write!(f, "E {}", qual),
            ScaleName::F(qual) => write!(f, "F {}", qual),
            ScaleName::FSharpGFlat(qual) => write!(f, "F#/Gb {}", qual),
            ScaleName::G(qual) => write!(f, "G {}", qual),
            ScaleName::GSharpAFlat(qual) => write!(f, "G#/Ab {}", qual),
            ScaleName::A(qual) => write!(f, "A {}", qual),
            ScaleName::ASharpBFlat(qual) => write!(f, "A#/Bb {}", qual),
            ScaleName::B(qual) => write!(f, "B {}", qual),
        }
    }
}

pub enum ScaleDegree {
    First(NoteName),
    FlatSecond(NoteName),
    Second(NoteName),
    SharpSecond(NoteName),
    FlatThird(NoteName),
    Third(NoteName),
    Fourth(NoteName),
    SharpFourth(NoteName),
    FlatFifth(NoteName),
    Fifth(NoteName),
    SharpFifth(NoteName),
    FlatSixth(NoteName),
    Sixth(NoteName),
    FlatSeventh(NoteName),
    Seventh(NoteName),
}

impl Display for ScaleDegree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let combining_circumflex = '\u{0302}';

        match self {
            ScaleDegree::First(note) => write!(f, "({}1) {note}", combining_circumflex),
            ScaleDegree::FlatSecond(note) => write!(f, "(b{}2) {note}", combining_circumflex),
            ScaleDegree::Second(note) => write!(f, "({}2/9) {note}", combining_circumflex),
            ScaleDegree::SharpSecond(note) => write!(f, "(#{}2/9) {note}", combining_circumflex),
            ScaleDegree::FlatThird(note) => write!(f, "(b{}3) {note}", combining_circumflex),
            ScaleDegree::Third(note) => write!(f, "({}3) {note}", combining_circumflex),
            ScaleDegree::Fourth(note) => write!(f, "({}4) {note}", combining_circumflex),
            ScaleDegree::SharpFourth(note) => write!(f, "(#{}4) {note}", combining_circumflex),
            ScaleDegree::FlatFifth(note) => write!(f, "(b{}5) {note}", combining_circumflex),
            ScaleDegree::Fifth(note) => write!(f, "({}5) {note}", combining_circumflex),
            ScaleDegree::SharpFifth(note) => write!(f, "(#{}5) {note}", combining_circumflex),
            ScaleDegree::FlatSixth(note) => write!(f, "(b{}6) {note}", combining_circumflex),
            ScaleDegree::Sixth(note) => write!(f, "({}6) {note}", combining_circumflex),
            ScaleDegree::FlatSeventh(note) => write!(f, "(b{}7) {note}", combining_circumflex),
            ScaleDegree::Seventh(note) => write!(f, "({}7) {note}", combining_circumflex),
        }
    }
}

pub struct ScaleSpelling {
    name: ScaleName,
    spelling: Vec<ScaleDegree>,
    quality: ScaleQuality,
    root: NoteName,
}
