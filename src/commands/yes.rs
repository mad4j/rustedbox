use std::{io, thread, time};

use clap::Args;

#[derive(Debug, Args)]
pub struct YesArgs {
    /// Message to be dumped
    #[arg(short, long, default_value = "y")]
    message: String,

    /// Number of repetitions (otherwise forever)
    #[arg(short, long)]
    count: Option<u16>,

    /// Delay between repetions (in seconds)
    #[arg(short, long, default_value_t = 0, requires = "count")]
    delay: u64,
}

pub fn yes_command(args: &YesArgs) -> io::Result<()> {
    let forever = args.count.is_none();
    let mut index = args.count.unwrap_or(1);

    while index > 0 {
        println!("{}", args.message);
        thread::sleep(time::Duration::from_secs(args.delay));
        if !forever {
            index = index - 1;
        }
    }

    Ok(())
}
