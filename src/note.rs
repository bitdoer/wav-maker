//! In this module we define a single note to be a pitch class together with
//! an octave, and provide a method (`equal_tempered()`) to extract from a note
//! its frequency in Hz.

use crate::error::SyntaxErrorType;

#[derive(Clone, Copy)]
pub enum PitchClass {
    A,
    BFlat,
    B,
    C,
    DFlat,
    D,
    EFlat,
    E,
    F,
    GFlat,
    G,
    AFlat,
}

// A note is, very simply, just a pitch class (like A, or F#), plus an octave
// number. For example, A4 is 440 Hz and A3 is 220 Hz. C4 is "middle C", and is
// the note right after B3.
#[derive(Clone, Copy)]
pub struct Note {
    pub pitch_class: PitchClass,
    pub octave: u32,
}

impl Note {
    pub fn new(note: &str) -> Result<Self, SyntaxErrorType> {
        // at least want to handle black key enharmonics, but let's not go crazy
        // with stuff like B# or Gbb for now
        let pitch_class = match &note[..(note.len() - 1)] {
            "A" => PitchClass::A,
            "A#" | "Bb" => PitchClass::BFlat,
            "B" => PitchClass::B,
            "C" => PitchClass::C,
            "C#" | "Db" => PitchClass::DFlat,
            "D" => PitchClass::D,
            "D#" | "Eb" => PitchClass::EFlat,
            "E" => PitchClass::E,
            "F" => PitchClass::F,
            "F#" | "Gb" => PitchClass::GFlat,
            "G" => PitchClass::G,
            "G#" | "Ab" => PitchClass::AFlat,
            e => return Err(SyntaxErrorType::BadPitchClass(e.to_string())),
        };
        // as currently coded, the octave can only go up to 9; all but the last
        // char of the string we're parsing is assumed to be part of the note
        let octave = match note.chars().last() {
            Some(ch) => match ch.to_digit(10) {
                Some(n) => n,
                None => return Err(SyntaxErrorType::BadOctave(ch.to_string())),
            },
            None => return Err(SyntaxErrorType::MissingEntry),
        };

        Ok(Self {
            pitch_class,
            octave,
        })
    }

    pub fn equal_tempered(&self) -> f64 {
        // Since we're using 12-tone equal temperament, we just have to pick a
        // base frequency; then, the octave number tells us how many times we
        // should double or halve it, and the pitch class tells us how many
        // times we should multiply the frequency by the 12th root of 2.

        // Note the weirdness with A, Bb, and B: scientific pitch notation
        // increments the octave number when going from B to C, but A is a much
        // more convenient base frequency as it is the only one set at a
        // rational number in, for example, A440 (A0 is 27.5 Hz, so we start at
        // 13.75 Hz, and the lowest representable frequency in the program is
        // C0).
        13.75
            * match self.pitch_class {
                PitchClass::A => 2.0f64.powi(self.octave as i32 + 1),
                PitchClass::BFlat => 2.0f64.powf((self.octave + 1) as f64 + 1.0 / 12.0),
                PitchClass::B => 2.0f64.powf((self.octave + 1) as f64 + 1.0 / 6.0),
                PitchClass::C => 2.0f64.powf(self.octave as f64 + 0.25),
                PitchClass::DFlat => 2.0f64.powf(self.octave as f64 + 1.0 / 3.0),
                PitchClass::D => 2.0f64.powf(self.octave as f64 + 5.0 / 12.0),
                PitchClass::EFlat => 2.0f64.powf(self.octave as f64 + 0.5),
                PitchClass::E => 2.0f64.powf(self.octave as f64 + 7.0 / 12.0),
                PitchClass::F => 2.0f64.powf(self.octave as f64 + 2.0 / 3.0),
                PitchClass::GFlat => 2.0f64.powf(self.octave as f64 + 0.75),
                PitchClass::G => 2.0f64.powf(self.octave as f64 + 5.0 / 6.0),
                PitchClass::AFlat => 2.0f64.powf(self.octave as f64 + 11.0 / 12.0),
            }
    }
}
