use clap::{Parser, Subcommand};
use commands::yes::{yes_command, YesArgs};

mod commands;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Output a string repeatedly, generally, until killed
    Yes(YesArgs),
}

fn main() {

    // parse command-line arguments
    let args = Args::parse();

    // execute specific command
    match &args.command {
        Commands::Yes(args) => yes_command(&args),
    }
}
