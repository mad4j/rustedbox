use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use clap::Args;

#[derive(Debug, Args)]
pub struct EntArgs {
    /// File to be processed
    path: PathBuf,

    /// Full precision output
    #[arg(short, long, action)]
    full: bool,
}

fn calculate_entropy(file_path: &Path) -> io::Result<f64> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut frequencies = HashMap::new();
    let mut total = 0;

    for &byte in &buffer {
        *frequencies.entry(byte).or_insert(0) += 1;
        total += 1;
    }

    let mut entropy = 0.0;
    for &frequency in frequencies.values() {
        let probability = (frequency as f64) / (total as f64);
        entropy -= probability * probability.log2();
    }

    Ok(entropy)
}

pub fn ent_command(args: &EntArgs) {
    if let Ok(result) = calculate_entropy(&args.path) {
        if args.full {
            println!("{}", result);
        } else {
            println!("{:.2}", result);
        };
    } else {
        println!(
            "Unable to read '{}'",
            args.path.to_str().unwrap_or_default()
        );
    }
}
