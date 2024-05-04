use std::io::{BufReader, Cursor};

use rodio::{Decoder, OutputStream};

// sound samples
const BEEP_BYTES: &[u8; 11468] = include_bytes!("../../resources/beep-10.wav");

pub fn beep_command() -> anyhow::Result<()> {
    // get default physical sound device handle
    let (_stream, handle) = OutputStream::try_default()?;

    // initialize audio framework
    let sink = rodio::Sink::try_new(&handle)?;
    let reader = BufReader::new(Cursor::new(BEEP_BYTES));
    let source = Decoder::new(reader)?;

    // play sound
    sink.append(source);
    sink.sleep_until_end();

    // return ok when finished
    Ok(())
}
