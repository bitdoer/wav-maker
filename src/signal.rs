//! This module defines NoteSignal, a struct corresponding to a single row in
//! an input file---effectively, a single note on a music score. It also defines
//! a couple of auxiliary types that are wrappers for parsing the duration and
//! wave type of such a signal.

use crate::error::SyntaxErrorType;
use crate::note::Note;

#[derive(Clone, Copy)]
pub enum WaveType {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

// duration is stored in "ticks", with 1 tick having the same duration as a
// 64th note, assuming a quarter note is 1 beat
#[derive(Clone, Copy)]
pub struct NoteDuration {
    pub ticks: u32,
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

impl NoteDuration {
    pub fn new(input: &str) -> Result<Self, SyntaxErrorType> {
        match input {
            "TS" => Ok(Self { ticks: 2 }),
            "DTS" => Ok(Self { ticks: 3 }),
            "S" => Ok(Self { ticks: 4 }),
            "DS" => Ok(Self { ticks: 6 }),
            "E" => Ok(Self { ticks: 8 }),
            "DE" => Ok(Self { ticks: 12 }),
            "Q" => Ok(Self { ticks: 16 }),
            "DQ" => Ok(Self { ticks: 24 }),
            "H" => Ok(Self { ticks: 32 }),
            "DH" => Ok(Self { ticks: 48 }),
            "W" => Ok(Self { ticks: 64 }),
            _ => match input.parse() {
                Ok(ticks) => Ok(Self { ticks }),
                Err(_) => Err(SyntaxErrorType::BadDuration(input.to_owned())),
            },
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
            Some(s) => NoteDuration::new(s)?,
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
