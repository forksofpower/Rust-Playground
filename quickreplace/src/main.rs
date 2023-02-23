use clap::Parser;
use regex::Regex;
use std::{error, fs};
use text_colorizer::*;

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    filename: std::path::PathBuf,

    #[arg(short, long)]
    output: String,

    target: String,
    replacement: String,
}

fn print_error_and_halt(msg: &str, err: &dyn error::Error) -> ! {
    eprintln!(
        "{} {} : {:?}",
        "Error:".red().bold(),
        String::from(msg),
        err
    );
    std::process::exit(1)
}

fn main() {
    let args = Arguments::parse();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => print_error_and_halt("failed to read from file", &e),
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => print_error_and_halt("failed to replace text", &e),
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}
        Err(e) => print_error_and_halt("failed to replace text", &e),
    }
}
