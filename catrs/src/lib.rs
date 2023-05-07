use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

/// Rust cat
#[derive(Parser, Debug)]
#[command(author = "Mark Leo Cativo", version, long_about = None)]
struct Catrs {
    /// Input file(s)
    #[arg(name = "FILES", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short = 'n', long = "number")]
    number_lines: bool,

    /// Number nonblank lines
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_with_line_number(line_number: usize, line: String) {
    println!("{line_number:>6}\t{line}");
}

fn print_lines(
    reader: Box<dyn BufRead>,
    number_lines: bool,
    number_nonblank_lines: bool,
) -> MyResult<()> {
    let mut line_number = 0;

    for line in reader.lines() {
        let line = line?;

        if number_lines {
            line_number += 1;
            print_with_line_number(line_number, line);
        } else if number_nonblank_lines {
            if line.is_empty() {
                println!();
            } else {
                line_number += 1;
                print_with_line_number(line_number, line);
            }
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn run() -> MyResult<()> {
    let catrs = Catrs::parse();

    for filename in catrs.files {
        match open(&filename) {
            Err(error) => eprintln!("{filename}: {error}"),
            Ok(reader) => print_lines(reader, catrs.number_lines, catrs.number_nonblank_lines)?,
        }
    }

    Ok(())
}
