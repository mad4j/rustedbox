use std::time::Duration;

use rodio::{source::SineWave, OutputStream, Source};

pub fn beep_command() -> anyhow::Result<()> {
    // get default physical sound device handle
    let (_stream, handle) = OutputStream::try_default()?;

    // initialize audio framework
    let sink = rodio::Sink::try_new(&handle)?;
    //let source = Decoder::new(reader)?;
    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.20);

    // play sound
    sink.append(source);
    sink.sleep_until_end();

    // return ok when finished
    Ok(())
}
