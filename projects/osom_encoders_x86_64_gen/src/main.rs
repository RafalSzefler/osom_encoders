mod generator;
mod globals;
mod models;
mod rustfmt;

use clap::Parser;
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, Parser)]
struct CliArgs {
    #[arg(short, long)]
    input: Option<PathBuf>,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> ExitCode {
    let args = CliArgs::parse();

    let input = args.input.unwrap_or(globals::DEFAULT_INSTRUCTIONS_XML.clone());

    println!("Reading instructions from {}", input.to_str().unwrap());
    let instruction_set = models::read_instruction_set(&input);

    let output_dir = args.output.unwrap_or(globals::ENCODING_OUT_DIR.clone());
    println!("Generating encoders to {}", output_dir.to_str().unwrap());

    if !output_dir.exists() {
        println!("Output directory does not exist. Is [{}] path correct?", output_dir.to_str().unwrap());
        return ExitCode::FAILURE;
    }

    generator::generate(&instruction_set, &output_dir);
    ExitCode::SUCCESS
}
