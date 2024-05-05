use clap::{Parser, Subcommand};
use commands::{
    beep::beep_command,
    bits::{bits_command, BitsArgs},
    ent::{ent_command, EntArgs},
    hash::{hash_command, HashArgs},
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
    /// Play a beep
    Beep,

    /// Count bits on specified file
    Bits(BitsArgs),

    /// Compute simple bits entropy
    Ent(EntArgs),

    /// Compute 64-bit hash on specified file
    Hash(HashArgs),

    /// Output a string repeatedly, generally, until killed
    Yes(YesArgs),

    /// Output zeros repeatedly, generally, until killed
    Zeros(ZerosArgs),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command-line arguments
    let args = Args::parse();

    // execute specific command
    match &args.command {
        Commands::Beep => beep_command()?,
        Commands::Bits(args) => bits_command(&args)?,
        Commands::Ent(args) => ent_command(&args)?,
        Commands::Hash(args) => hash_command(&args)?,
        Commands::Yes(args) => yes_command(&args)?,
        Commands::Zeros(args) => zeros_command(&args)?,
    };

    // return ok
    Ok(())
}
