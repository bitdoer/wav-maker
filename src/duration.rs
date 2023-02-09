//! This module defines an enum for note duration, which has a method to parse
//! the input and another method to spit out how many 16ths-of-a-beat (64th
//! note beats, "ticks") each variant represents.

use crate::error::SyntaxErrorType;

#[derive(Clone, Copy)]
pub enum NoteDuration {
    ThirtySecond,
    DottedThirtySecond,
    Sixteenth,
    DottedSixteenth,
    Eighth,
    DottedEighth,
    Quarter,
    DottedQuarter,
    Half,
    DottedHalf,
    Whole,
}

impl NoteDuration {
    pub fn from_abbrev(input: &str) -> Result<Self, SyntaxErrorType> {
        match input {
            "TS" => Ok(Self::ThirtySecond),
            "DTS" => Ok(Self::DottedThirtySecond),
            "S" => Ok(Self::Sixteenth),
            "DS" => Ok(Self::DottedSixteenth),
            "E" => Ok(Self::Eighth),
            "DE" => Ok(Self::DottedEighth),
            "Q" => Ok(Self::Quarter),
            "DQ" => Ok(Self::DottedQuarter),
            "H" => Ok(Self::Half),
            "DH" => Ok(Self::DottedHalf),
            "W" => Ok(Self::Whole),
            _ => Err(SyntaxErrorType::BadDuration(input.to_owned())),
        }
    }

    pub fn as_ticks(&self) -> u32 {
        match self {
            Self::ThirtySecond => 2,
            Self::DottedThirtySecond => 3,
            Self::Sixteenth => 4,
            Self::DottedSixteenth => 6,
            Self::Eighth => 8,
            Self::DottedEighth => 12,
            Self::Quarter => 16,
            Self::DottedQuarter => 24,
            Self::Half => 32,
            Self::DottedHalf => 48,
            Self::Whole => 64,
        }
    }
}
