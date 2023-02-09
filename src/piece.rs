use crate::error::{MusicError, SyntaxErrorType};
use crate::signal::{NoteSignal, WaveType};
use crate::utils::*;

pub struct MusicalPiece {
    signals: Vec<NoteSignal>,
    bpm: f64,
}

impl MusicalPiece {
    pub fn new(input: &str) -> Result<Self, MusicError> {
        let mut signals = vec![];
        let bpm = match input.lines().next() {
            Some(bpm) => match bpm.parse() {
                Ok(n) => n,
                Err(_) => {
                    return Err(MusicError::SyntaxError(
                        1,
                        SyntaxErrorType::BadBPM(bpm.to_string()),
                    ))
                }
            },
            None => return Err(MusicError::SyntaxError(1, SyntaxErrorType::MissingEntry)),
        };

        for (n, line) in input.lines().enumerate().skip(1) {
            signals.push(match NoteSignal::new(line) {
                Ok(sig) => sig,
                Err(e) => return Err(MusicError::SyntaxError(n + 1, e)),
            });
        }
        Ok(Self { signals, bpm })
    }

    pub fn synthesize(&self) -> Vec<u8> {
        let mut data = vec![];
        let mut running_signals = self.signals.clone();
        for sample in 0.. {
            let mut acc = 0.0f64;
            for &signal in running_signals.iter() {
                if signal.start + signal.duration.as_sf() >= self.sample_to_sf(sample)
                    && signal.start <= self.sample_to_sf(sample)
                {
                    acc += match signal.wavetype {
                        WaveType::Sine => {
                            sine_wave(sample, signal.ampl, signal.note.equal_tempered())
                        }
                        WaveType::Square => {
                            square_wave(sample, signal.ampl, signal.note.equal_tempered())
                        }
                        WaveType::Triangle => {
                            triangle_wave(sample, signal.ampl, signal.note.equal_tempered())
                        }
                        WaveType::Sawtooth => {
                            sawtooth_wave(sample, signal.ampl, signal.note.equal_tempered())
                        }
                    };
                }
            }
            let height = if acc.round() > 32767.0 {
                i16::MAX
            } else if acc < -32768.0 {
                i16::MIN
            } else {
                acc.round() as i16
            };
            data.extend_from_slice(&height.to_le_bytes());
            running_signals = running_signals
                .into_iter()
                .filter(|&signal| {
                    signal.start + signal.duration.as_sf() >= self.sample_to_sf(sample)
                })
                .collect();
            if running_signals.is_empty() {
                break;
            }
        }
        data
    }

    fn sample_to_sf(&self, sample: u32) -> u32 {
        ((sample as f64 / SAMPLE_RATE as f64) * 16.0 * self.bpm / 60.0).floor() as u32
    }
}
