use clap::{Parser, Subcommand};
use commands::{
    ent::{ent_command, EntArgs},
    yes::{yes_command, YesArgs},
    zeros::{zeros_command, ZerosArgs},
};

mod commands;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Compute simple Entropy
    Ent(EntArgs),

    /// Output a string repeatedly, generally, until killed
    Yes(YesArgs),

    /// Output zeros repeatedly, generally, until killed
    Zeros(ZerosArgs),
}

fn main() {
    // parse command-line arguments
    let args = Args::parse();

    // execute specific command
    match &args.command {
        Commands::Ent(args) => ent_command(&args),
        Commands::Yes(args) => yes_command(&args),
        Commands::Zeros(args) => zeros_command(&args),
    }
}
