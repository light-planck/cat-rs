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
    for filename in &args.files {
        match open(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                let mut line_number = 1;

                for line in file.lines() {
                    let line = line?;

                    match (args.number_lines, args.number_nonblank_lines) {
                        (false, false) => println!("{}", line),
                        (true, false) => {
                            println!("{:>6}\t{}", line_number, line);
                            line_number += 1;
                        }
                        (false, true) => {
                            if line.is_empty() {
                                println!();
                            } else {
                                println!("{:>6}\t{}", line_number, line);
                                line_number += 1;
                            }
                        }
                        (true, true) => unreachable!(),
                    }
                }
            }
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
