use std::{thread, time};

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

pub fn yes_command(args: &YesArgs) {

    let count = args.count.unwrap_or(1); 
    let mut index = count;

    while index > 0 {
        println!("{}", args.message);
        thread::sleep(time::Duration::from_secs(args.delay));
        if count > 0 {
            index = index - 1;
        }
    }
}
