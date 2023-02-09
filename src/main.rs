mod duration;
mod error;
mod note;
mod piece;
mod signal;
mod utils;

use crate::error::MusicError;
use crate::piece::MusicalPiece;
use crate::utils::header;

fn main() {
    match run() {
        Ok(_) => return,
        Err(e) => println!("{}", e),
    }
}

fn run() -> Result<(), MusicError> {
    let args = std::env::args();
    if args.len() < 2 {
        println!("Usage: wav-maker <input file>");
        return Ok(());
    }

    // get file input
    let filename = args.skip(1).next().expect("must exist by if statement");
    let input = match std::fs::read_to_string(&filename) {
        Ok(s) => s,
        Err(_) => return Err(MusicError::FileReadError(filename)),
    };

    // generate output waveform values
    let piece = MusicalPiece::new(&input)?;
    let data = piece.synthesize();

    // prepare output buffer with header
    let mut output = header(data.len() as u32);

    // load output waveform data into buffer
    output.extend_from_slice(&data);

    // write buffer into file
    if std::fs::write(&format!("{}.wav", filename), &output).is_err() {
        return Err(MusicError::FileWriteError(format!("{}.wav", filename)));
    }

    Ok(())
}
