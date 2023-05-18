use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

/// WC test
#[derive(Debug, Parser)]
#[command(author = "Mark Leo Cativo", version, about, long_about = None)]
struct Args {
    /// Input file(s)
    #[arg(name = "FILES", default_value = "-")]
    files: Vec<String>,

    /// Show byte count
    #[arg(short = 'c', long = "bytes", conflicts_with = "chars")]
    bytes: bool,

    /// Show character count
    #[arg(short = 'm', long = "chars", conflicts_with = "bytes")]
    chars: bool,

    /// Show line count
    #[arg(short = 'l', long = "lines")]
    lines: bool,

    /// Show word count
    #[arg(short = 'w', long = "words")]
    words: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


pub fn run() -> MyResult<()> {
    let args = Args::parse();

    Ok(())
}
