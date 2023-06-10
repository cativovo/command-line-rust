use std::{
    error::Error,
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
};

use clap::Parser;

/// Uniqrs
#[derive(Parser, Debug)]
#[command(author = "Mark Leo Cativo", version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(name = "IN_FILE", default_value = "-")]
    input: String,

    /// Output file
    #[arg(name = "OUT_FILE")]
    output: Option<String>,

    /// Show line count
    #[arg(short, long)]
    count: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let mut reader = open(&args.input).map_err(|err| format!("{}: {}", &args.input, err))?;
    let mut line = String::new();
    let mut previous_line = String::new();
    let mut count = 0;

    let mut out_file: Box<dyn Write> = match args.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(stdout()),
    };

    let mut print = |count: u32, text: &str| -> MyResult<()> {
        const SPACE_WIDTH: usize = 4;
        if args.count {
            write!(out_file, "{count:>SPACE_WIDTH$} {text}")?;
        } else {
            write!(out_file, "{text}")?;
        }

        Ok(())
    };

    loop {
        let bytes = reader.read_line(&mut line)?;

        if previous_line.is_empty() {
            previous_line.push_str(line.as_str());
        }

        if previous_line.trim() != line.trim() {
            print(count, &previous_line)?;

            count = 0;
            previous_line.clear();
            previous_line.push_str(line.as_str());
        }

        count += 1;

        line.clear();

        if bytes == 0 {
            break;
        }
    }

    Ok(())
}
