use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

/// Cat command written in Rust
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Files to read
    #[arg(default_value("-"))]
    files: Vec<String>,

    /// Number the output lines, starting at 1.
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    /// Number the non-blank output lines, starting at 1.
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}

pub fn run(args: Args) -> MyResult<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to oepn {}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
