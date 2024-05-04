use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::PathBuf,
};

use clap::Args;
use itertools::Itertools;
use ring::digest::{Context, Digest, SHA256};

#[derive(Debug, Args)]
pub struct HashArgs {
    /// File to be processed
    path: PathBuf,
}

fn calculate_digest<R: Read>(mut reader: R) -> io::Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub fn hash_command(args: &HashArgs) -> io::Result<()> {
    let input = File::open(&args.path)?;
    let reader = BufReader::new(input);
    let digest = calculate_digest(reader)?;

    println!("{:02X?}", &digest.as_ref()[24..].iter().format(""));

    Ok(())
}
