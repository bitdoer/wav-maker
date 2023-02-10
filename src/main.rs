mod error;
mod note;
mod piece;
mod signal;
mod utils;

use crate::error::MusicError;
use crate::piece::MusicalPiece;
use crate::utils::header;

use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,
    #[arg(short, long)]
    bpm: Option<f64>,
    #[arg(short, long)]
    ampl: Option<u16>,
}

fn main() {
    match run() {
        Ok(_) => return,
        Err(e) => println!("{}", e),
    }
}

fn run() -> Result<(), MusicError> {
    let args = Args::parse();

    let input = match std::fs::read_to_string(&args.file) {
        Ok(s) => s,
        Err(_) => return Err(MusicError::FileReadError(args.file)),
    };

    // generate output waveform values
    let piece = MusicalPiece::new(&input, args.bpm, args.ampl)?;
    let data = piece.synthesize();

    // prepare output buffer with header
    let mut output = header(data.len() as u32);

    // load output waveform data into buffer
    output.extend_from_slice(&data);

    // write buffer into file
    if std::fs::write(&format!("{}.wav", args.file), &output).is_err() {
        return Err(MusicError::FileWriteError(format!("{}.wav", args.file)));
    }

    Ok(())
}
