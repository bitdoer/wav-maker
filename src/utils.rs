//! This module contains the constants needed by wav-maker, such as header data
//! and math constants, as well as helper functions for specific waveforms and
//! for header generation.

// the portions of the RIFF header corresponding to ASCII text
pub const RIFF: [u8; 4] = [0x52, 0x49, 0x46, 0x46];
pub const WAVE: [u8; 4] = [0x57, 0x41, 0x56, 0x45];
pub const FMT: [u8; 4] = [0x66, 0x6D, 0x74, 0x20];
pub const DATA: [u8; 4] = [0x64, 0x61, 0x74, 0x61];

// basic format constants (PCM, mono, 44.1kHz, 16-bit samples)
pub const CHUNK_SIZE: u32 = 16;
pub const FORMAT: u16 = 1;
pub const CHANNELS: u16 = 1;
pub const SAMPLE_RATE: u32 = 44100;
pub const BITS_PER_SAMPLE: u16 = 16;

// signal parameters and mathematical constants
pub const BASE_AMPLITUDE: f64 = 2048.0;
pub const PI: f64 = 3.141592653589793;

pub fn sine_wave(n: u32, ampl: f64, freq: f64) -> f64 {
    ampl * BASE_AMPLITUDE * f64::sin(2.0 * PI * freq * n as f64 / SAMPLE_RATE as f64)
}

pub fn square_wave(n: u32, ampl: f64, freq: f64) -> f64 {
    ampl * BASE_AMPLITUDE * f64::signum(f64::sin(2.0 * PI * freq * n as f64 / SAMPLE_RATE as f64))
}

pub fn triangle_wave(n: u32, ampl: f64, freq: f64) -> f64 {
    ampl * BASE_AMPLITUDE
        * (4.0
            * ((n as f64 * freq) / SAMPLE_RATE as f64
                - (0.5 + (n as f64 * freq) / SAMPLE_RATE as f64).floor())
            .abs()
            - 1.0)
}

pub fn sawtooth_wave(n: u32, ampl: f64, freq: f64) -> f64 {
    2.0 * ampl
        * BASE_AMPLITUDE
        * ((n as f64 * freq) / SAMPLE_RATE as f64
            - (0.5 + (n as f64 * freq) / SAMPLE_RATE as f64).floor())
}

pub fn header(data_size: u32) -> Vec<u8> {
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
