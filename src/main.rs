// +------------+
// | Data types |
// +------------+

#[derive(Clone, Copy)]
enum NoteDuration {
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

#[derive(Clone, Copy)]
enum PitchClass {
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

#[derive(Clone, Copy)]
struct Note {
    pitch_class: PitchClass,
    octave: u32,
}

#[derive(Clone, Copy)]
enum WaveType {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

#[derive(Clone, Copy)]
struct NoteSignal {
    start: u32,
    duration: NoteDuration,
    note: Note,
    ampl: f64,
    wavetype: WaveType,
}

struct MusicalPiece {
    signals: Vec<NoteSignal>,
    bpm: f64,
}

// +-------------------+
// |  Implementations  |
// +-------------------+

impl NoteDuration {
    fn from_abbrev(input: &str) -> Self {
        match input {
            "TS" => Self::ThirtySecond,
            "DTS" => Self::DottedThirtySecond,
            "S" => Self::Sixteenth,
            "DS" => Self::DottedSixteenth,
            "E" => Self::Eighth,
            "DE" => Self::DottedEighth,
            "Q" => Self::Quarter,
            "DQ" => Self::DottedQuarter,
            "H" => Self::Half,
            "DH" => Self::DottedHalf,
            "W" => Self::Whole,
            _ => panic!("bad duration abbrev"),
        }
    }

    fn as_sf(&self) -> u32 {
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

impl Note {
    fn new(note: &str) -> Self {
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
            _ => panic!("Bad pitch class"),
        };
        let octave = note
            .chars()
            .last()
            .expect("Bad octave")
            .to_digit(10)
            .expect("Bad octave");
        Self {
            pitch_class,
            octave,
        }
    }
}

impl WaveType {
    fn new(input: &str) -> Self {
        match input {
            "S" => Self::Sine,
            "Q" => Self::Square,
            "T" => Self::Triangle,
            "A" => Self::Sawtooth,
            _ => panic!("Bad wave type"),
        }
    }
}

impl NoteSignal {
    fn new(input: &str) -> Self {
        let parts = input.split_whitespace().collect::<Vec<_>>();
        Self {
            start: parts[0].parse().expect("Bad start value"),
            duration: NoteDuration::from_abbrev(parts[1]),
            note: Note::new(parts[2]),
            ampl: parts[3].parse().expect("Bad amplitude"),
            wavetype: WaveType::new(parts[4]),
        }
    }
}

impl MusicalPiece {
    fn new(input: &str) -> Self {
        let mut signals = vec![];
        let bpm = input
            .lines()
            .next()
            .expect("Bad BPM")
            .parse()
            .expect("Bad BPM");
        for line in input.lines().skip(1) {
            signals.push(NoteSignal::new(line));
        }
        Self { signals, bpm }
    }

    fn synthesize(&self) -> Vec<u8> {
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
                            sine_wave(sample, signal.ampl, equal_tempered(signal.note))
                        }
                        WaveType::Square => {
                            square_wave(sample, signal.ampl, equal_tempered(signal.note))
                        }
                        WaveType::Triangle => {
                            triangle_wave(sample, signal.ampl, equal_tempered(signal.note))
                        }
                        WaveType::Sawtooth => {
                            sawtooth_wave(sample, signal.ampl, equal_tempered(signal.note))
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

// +-----------+
// | Constants |
// +-----------+

// the portions of the RIFF header corresponding to ASCII text
const RIFF: [u8; 4] = [0x52, 0x49, 0x46, 0x46];
const WAVE: [u8; 4] = [0x57, 0x41, 0x56, 0x45];
const FMT: [u8; 4] = [0x66, 0x6D, 0x74, 0x20];
const DATA: [u8; 4] = [0x64, 0x61, 0x74, 0x61];

// basic format constants (PCM, mono, 44.1kHz, 16-bit samples)
const CHUNK_SIZE: u32 = 16;
const FORMAT: u16 = 1;
const CHANNELS: u16 = 1;
const SAMPLE_RATE: u32 = 44100;
const BITS_PER_SAMPLE: u16 = 16;

// signal parameters and mathematical constants
const BASE_AMPLITUDE: f64 = 2048.0;
const PI: f64 = 3.141592653589793;

// +-------------------------+
// | Signal/helper functions |
// +-------------------------+

fn sine_wave(n: u32, ampl: f64, freq: f64) -> f64 {
    ampl * BASE_AMPLITUDE * f64::sin(2.0 * PI * freq * n as f64 / SAMPLE_RATE as f64)
}

fn square_wave(n: u32, ampl: f64, freq: f64) -> f64 {
    ampl * BASE_AMPLITUDE * f64::signum(f64::sin(2.0 * PI * freq * n as f64 / SAMPLE_RATE as f64))
}

fn triangle_wave(n: u32, ampl: f64, freq: f64) -> f64 {
    ampl * BASE_AMPLITUDE
        * (4.0
            * ((n as f64 * freq) / SAMPLE_RATE as f64
                - (0.5 + (n as f64 * freq) / SAMPLE_RATE as f64).floor())
            .abs()
            - 1.0)
}

fn sawtooth_wave(n: u32, ampl: f64, freq: f64) -> f64 {
    2.0 * ampl
        * BASE_AMPLITUDE
        * ((n as f64 * freq) / SAMPLE_RATE as f64
            - (0.5 + (n as f64 * freq) / SAMPLE_RATE as f64).floor())
}

fn equal_tempered(note: Note) -> f64 {
    13.5 * match note.pitch_class {
        PitchClass::A => 2.0f64.powi(note.octave as i32 + 1),
        PitchClass::BFlat => 2.0f64.powf((note.octave + 1) as f64 + 1.0 / 12.0),
        PitchClass::B => 2.0f64.powf((note.octave + 1) as f64 + 1.0 / 6.0),
        PitchClass::C => 2.0f64.powf(note.octave as f64 + 0.25),
        PitchClass::DFlat => 2.0f64.powf(note.octave as f64 + 1.0 / 3.0),
        PitchClass::D => 2.0f64.powf(note.octave as f64 + 5.0 / 12.0),
        PitchClass::EFlat => 2.0f64.powf(note.octave as f64 + 0.5),
        PitchClass::E => 2.0f64.powf(note.octave as f64 + 7.0 / 12.0),
        PitchClass::F => 2.0f64.powf(note.octave as f64 + 2.0 / 3.0),
        PitchClass::GFlat => 2.0f64.powf(note.octave as f64 + 0.75),
        PitchClass::G => 2.0f64.powf(note.octave as f64 + 5.0 / 6.0),
        PitchClass::AFlat => 2.0f64.powf(note.octave as f64 + 11.0 / 12.0),
    }
}

fn header(data_size: u32) -> Vec<u8> {
    let mut output = RIFF.to_vec();
    let block_align = CHANNELS * BITS_PER_SAMPLE / 8;
    let bytes_per_sec = SAMPLE_RATE * (block_align as u32);
    let file_size = 36u32 + data_size;
    output.extend_from_slice(&file_size.to_le_bytes());
    output.extend_from_slice(&WAVE);
    output.extend_from_slice(&FMT);
    output.extend_from_slice(&CHUNK_SIZE.to_le_bytes());
    output.extend_from_slice(&FORMAT.to_le_bytes());
    output.extend_from_slice(&CHANNELS.to_le_bytes());
    output.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    output.extend_from_slice(&bytes_per_sec.to_le_bytes());
    output.extend_from_slice(&block_align.to_le_bytes());
    output.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());
    output.extend_from_slice(&DATA);
    output.extend_from_slice(&data_size.to_le_bytes());
    output
}

// +---------------+
// | Main function |
// +---------------+

fn main() {
    let args = std::env::args();
    if args.len() < 2 {
        println!("Usage: wav-maker <input file>");
        return;
    }

    // generate waveform
    let input =
        std::fs::read_to_string(args.skip(1).next().unwrap()).expect("Failed to read from file");
    let piece = MusicalPiece::new(&input);
    let data = piece.synthesize();

    // prepare output buffer with header
    let mut output = header(data.len() as u32);

    // load data into buffer
    output.extend_from_slice(&data);

    // write buffer into file
    std::fs::write("test.wav", &output).expect("Failed to write to file");
}
