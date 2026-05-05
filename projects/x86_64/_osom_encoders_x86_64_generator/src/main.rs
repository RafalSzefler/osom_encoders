#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]

use std::{error::Error, path::PathBuf};

use clap::Parser;
use path_absolutize::Absolutize as _;

mod generator;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Destination directory to store generated files.
    #[arg(short, long)]
    destination: PathBuf,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let path = args.destination.absolutize()?.to_path_buf();
    let generator = generator::Generator::new(path);
    generator.generate()?;
    println!("Success!");
    Ok(())
}
