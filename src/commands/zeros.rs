use rand::Rng;
use std::{
    io::{self, Write},
    thread, time,
};

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

    /// Increment value at each repetition
    #[arg(short, long, action)]
    increment: bool,

    /// Increment value at each repetition
    #[arg(
        short,
        long,
        action,
        conflicts_with = "value",
        conflicts_with = "increment"
    )]
    random: bool,

    /// Append a new-line
    #[arg(short, long, action, conflicts_with = "binary")]
    new_line: bool,

    /// Delay between repetions (in seconds)
    #[arg(short, long, default_value_t = 0, requires = "count")]
    delay: u64,
}

pub fn zeros_command(args: &ZerosArgs) -> io::Result<()> {
    // create a new random generator
    let mut rng = rand::thread_rng();

    // retrive start value from args
    let mut value = args.value.unwrap_or(0);

    // verify if forevere loop is requested
    let forever = args.count.is_none();

    // retrieve number of requested repetitions
    let mut index = args.count.unwrap_or(1);

    // loop on repetions (or forevere, if requested)
    while index > 0 {
        // new random value, if requested
        if args.random {
            value = rng.gen();
        }

        // dump binary values or numbers
        if args.binary {
            let _ = io::stdout().write_all(&[value]);
        } else {
            print!("{}", value);
            if args.new_line {
                println!();
            }
        }

        // wait for a while
        thread::sleep(time::Duration::from_secs(args.delay));

        // count repetitions
        if !forever {
            index = index - 1;
        }

        // increment value, if needed
        if args.increment {
            value = value.wrapping_add(1);
        }
    }

    Ok(())
}
