use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use clap::Args;

#[derive(Debug, Args)]
pub struct BitsArgs {
    /// File to be processed
    path: PathBuf,

    /// Full precision output
    #[arg(short, long, action)]
    full: bool,
}

fn calculate_bits<R: Read>(mut reader: R) -> io::Result<HashMap<u8, u64>> {
    let mut frequencies = HashMap::new();

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    for &byte in &buffer {
        *frequencies.entry(0).or_insert(0u64) += byte.count_zeros() as u64;
        *frequencies.entry(1).or_insert(0u64) += byte.count_ones() as u64;
    }

    Ok(frequencies)
}

pub fn bits_command(args: &BitsArgs) -> io::Result<()> {
    let file = File::open(&args.path)?;

    let h = calculate_bits(file)?;

    let total = h.values().sum::<u64>() as f64;

    let result = h[&1] as f64 / total;

    if args.full {
        println!("{}", result);
    } else {
        println!("{:.2}", result);
    };

    Ok(())
}
