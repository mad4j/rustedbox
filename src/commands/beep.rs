use std::io;


pub fn beep_command() -> io::Result<()> {
    print!("\x07");
    Ok(())
}