//! This module defines the type MusicalPiece, which is taken to be a sequence
//! of notes, together with a tempo and an amplitude. It gives it a method to
//! read a file input, and another to spit out the PCM output that the piece
//! represents.

use crate::error::{MusicError, SyntaxErrorType};
use crate::signal::{NoteSignal, WaveType};
use crate::utils::*;

// by a musical piece, we simply mean a score with a tempo
pub struct MusicalPiece {
    signals: Vec<NoteSignal>,
    bpm: f64,
    ampl: u16,
}

impl MusicalPiece {
    pub fn new(
        input: &str,
        mut bpm: Option<f64>,
        mut ampl: Option<u16>,
    ) -> Result<Self, MusicError> {
        let mut signals = vec![];

        for (n, line) in input.lines().enumerate() {
            if line.starts_with("BPM ") {
                if bpm.is_none() {
                    bpm = Some(
                        match line.split_whitespace().nth(1).unwrap_or_default().parse() {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(MusicError::SyntaxError(
                                    n + 1,
                                    SyntaxErrorType::BadBPM(line[4..].to_string()),
                                ))
                            }
                        },
                    );
                }
            } else if line.starts_with("AMPL ") || line.starts_with("AMPLITUDE ") {
                if ampl.is_none() {
                    ampl = Some(
                        match line.split_whitespace().nth(1).unwrap_or_default().parse() {
                            Ok(n) => n,
                            Err(_) => {
                                return Err(MusicError::SyntaxError(
                                    n + 1,
                                    SyntaxErrorType::BadAmplitude(line[4..].to_string()),
                                ))
                            }
                        },
                    );
                }
            } else {
                signals.push(match NoteSignal::new(line) {
                    Ok(sig) => sig,
                    Err(e) => return Err(MusicError::SyntaxError(n + 1, e)),
                });
            }
        }
        Ok(Self {
            signals,
            bpm: bpm.unwrap_or(DEFAULT_BPM),
            ampl: ampl.unwrap_or(DEFAULT_AMPL),
        })
    }

    // given a musical piece, produce a vector of bytes representing the 16-bit
    // PCM encoding of the signal
    pub fn synthesize(&self) -> Vec<u8> {
        let mut data = vec![];
        let mut running_signals = self.signals.clone();
        for sample in 0.. {
            let mut acc = 0.0f64;
            // at each point in time, we want to superpose all signals impinging
            // on that time
            for &signal in running_signals.iter() {
                // if a signal has started and it hasn't ended,
                if signal.start + signal.duration.ticks >= self.sample_to_tick(sample)
                    && signal.start <= self.sample_to_tick(sample)
                {
                    // add that signal to the running total
                    acc += match signal.wavetype {
                        WaveType::Sine => sine_wave(
                            sample,
                            signal.ampl * self.ampl as f64,
                            signal.note.equal_tempered(),
                        ),
                        WaveType::Square => square_wave(
                            sample,
                            signal.ampl * self.ampl as f64,
                            signal.note.equal_tempered(),
                        ),
                        WaveType::Triangle => triangle_wave(
                            sample,
                            signal.ampl * self.ampl as f64,
                            signal.note.equal_tempered(),
                        ),
                        WaveType::Sawtooth => sawtooth_wave(
                            sample,
                            signal.ampl * self.ampl as f64,
                            signal.note.equal_tempered(),
                        ),
                    };
                }
            }
            // then, clamp the datum and convert to 16-bit integer
            let height = if acc.round() > 32767.0 {
                i16::MAX
            } else if acc < -32768.0 {
                i16::MIN
            } else {
                acc.round() as i16
            };
            // add datum to output
            data.extend_from_slice(&height.to_le_bytes());

            // remove all signals that have ended from consideration
            running_signals = running_signals
                .into_iter()
                .filter(|&signal| {
                    signal.start + signal.duration.ticks >= self.sample_to_tick(sample)
                })
                .collect();

            // we're finished if there are no more signals
            if running_signals.is_empty() {
                break;
            }
        }
        data
    }

    // if we're on the nth sample in the audio, what 64th-note beat of the piece
    // are we in, given our tempo, if 0 marks the first such 64th-note beat?
    fn sample_to_tick(&self, sample: u32) -> u32 {
        ((sample as f64 / SAMPLE_RATE as f64) * 16.0 * self.bpm / 60.0).floor() as u32
    }
}
