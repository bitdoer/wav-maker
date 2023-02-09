//! This module defines NoteSignal, a struct corresponding to a single row in
//! an input file---effectively, a single note on a music score.

use crate::duration::NoteDuration;
use crate::error::SyntaxErrorType;
use crate::note::Note;

#[derive(Clone, Copy)]
pub enum WaveType {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

// a "note signal" is a pitch, a start, a duration, an amplitude, and a waveform
#[derive(Clone, Copy)]
pub struct NoteSignal {
    pub start: u32,
    pub duration: NoteDuration,
    pub note: Note,
    pub ampl: f64,
    pub wavetype: WaveType,
}

impl WaveType {
    pub fn new(input: &str) -> Result<Self, SyntaxErrorType> {
        match input {
            "S" => Ok(Self::Sine),
            "Q" => Ok(Self::Square),
            "T" => Ok(Self::Triangle),
            "A" => Ok(Self::Sawtooth),
            _ => Err(SyntaxErrorType::BadWaveform(input.to_string())),
        }
    }
}

impl NoteSignal {
    pub fn new(input: &str) -> Result<Self, SyntaxErrorType> {
        let parts = input.split_whitespace().collect::<Vec<_>>();
        let start = match parts.get(0) {
            Some(s) => match s.parse() {
                Ok(n) => n,
                Err(_) => return Err(SyntaxErrorType::BadStartTime(s.to_string())),
            },
            None => return Err(SyntaxErrorType::MissingEntry),
        };
        let duration = match parts.get(1) {
            Some(s) => NoteDuration::from_abbrev(s)?,
            None => return Err(SyntaxErrorType::MissingEntry),
        };
        let note = match parts.get(2) {
            Some(s) => Note::new(s)?,
            None => return Err(SyntaxErrorType::MissingEntry),
        };
        let ampl = match parts.get(3) {
            Some(s) => match s.parse() {
                Ok(n) => n,
                Err(_) => return Err(SyntaxErrorType::BadAmplitude(s.to_string())),
            },
            None => return Err(SyntaxErrorType::MissingEntry),
        };
        let wavetype = match parts.get(4) {
            Some(s) => WaveType::new(s)?,
            None => return Err(SyntaxErrorType::MissingEntry),
        };
        Ok(Self {
            start,
            duration,
            note,
            ampl,
            wavetype,
        })
    }
}
