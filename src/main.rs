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
    /// does testing things
    Yes(YesArgs),
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Yes(args) => yes_command(&args),
    }
}
