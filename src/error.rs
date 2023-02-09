//! This module defines the error types that the program currently handles;
//! these are used throughout the code and bubbled up whenever necessary.

#[derive(Debug)]
pub enum SyntaxErrorType {
    MissingEntry,
    BadBPM(String),
    BadStartTime(String),
    BadDuration(String),
    BadPitchClass(String),
    BadOctave(String),
    BadAmplitude(String),
    BadWaveform(String),
}

#[derive(Debug)]
pub enum MusicError {
    SyntaxError(usize, SyntaxErrorType),
    FileReadError(String),
    FileWriteError(String),
}

impl std::fmt::Display for SyntaxErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingEntry => write!(f, "missing entry"),
            Self::BadBPM(s) => write!(f, "invalid BPM: \"{}\"", s),
            Self::BadStartTime(s) => write!(f, "invalid start time: \"{}\"", s),
            Self::BadDuration(s) => write!(f, "invalid duration: \"{}\"", s),
            Self::BadPitchClass(s) => write!(f, "invalid pitch class: \"{}\"", s),
            Self::BadOctave(s) => write!(f, "invalid octave: \"{}\"", s),
            Self::BadAmplitude(s) => write!(f, "invalid amplitude: \"{}\"", s),
            Self::BadWaveform(s) => write!(f, "invalid waveform abbreviation: \"{}\"", s),
        }
    }
}

impl std::fmt::Display for MusicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MusicError::SyntaxError(line, e) => write!(f, "Syntax error (line {}): {}", line, e),
            MusicError::FileReadError(file) => {
                write!(f, "I/O error: failed to read file \"{}\"", file)
            }
            MusicError::FileWriteError(file) => {
                write!(f, "I/O error: failed to write to file \"{}\"", file)
            }
        }
    }
}
