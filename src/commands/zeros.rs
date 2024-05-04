use std::{io::{self, Write}, thread, time};

use clap::Args;

#[derive(Debug, Args)]
pub struct ZerosArgs {

    /// Value to be dumped
    #[arg(short, long)]
    value: Option<u8>,

    /// Number of repetitions (otherwise forever)
    #[arg(short, long)]
    count: Option<u16>,

    /// Dump binary output
    #[arg(short, long, action)]
    binary: bool,

    /// Delay between repetions (in seconds)
    #[arg(short, long, default_value_t = 0, requires = "count")]
    delay: u64,
}

pub fn zeros_command(args: &ZerosArgs) {

    let value = args.value.unwrap_or(0);

    let forever = args.count.is_none();
    let mut index = args.count.unwrap_or(1);

    while index > 0 {
        if args.binary {
            let _ = io::stdout().write_all(&[value]);
        } else {
            print!("{}", value as char);
        }
        thread::sleep(time::Duration::from_secs(args.delay));
        if !forever {
            index = index - 1;
        }
    }
}
