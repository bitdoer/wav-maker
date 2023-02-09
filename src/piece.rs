use crate::error::{MusicError, SyntaxErrorType};
use crate::signal::{NoteSignal, WaveType};
use crate::utils::*;

// by a musical piece, we simply mean a score with a tempo
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
                if signal.start + signal.duration.as_ticks() >= self.sample_to_tick(sample)
                    && signal.start <= self.sample_to_tick(sample)
                {
                    // add that signal to the running total
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
                    signal.start + signal.duration.as_ticks() >= self.sample_to_tick(sample)
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
