use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
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

#[derive(PartialEq, Debug)]
pub struct FileInfo {
    num_bytes: usize,
    num_chars: usize,
    num_lines: usize,
    num_words: usize,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut reader: impl BufRead) -> MyResult<FileInfo> {
    let mut buf = String::new();

    let num_bytes = reader.read_to_string(&mut buf)?;
    let num_chars = buf.chars().count();
    let num_lines = buf.lines().count();
    let num_words = buf.split_whitespace().count();

    Ok(FileInfo {
        num_bytes,
        num_chars,
        num_lines,
        num_words,
    })
}

fn get_args() -> Args {
    let mut args = Args::parse();

    if [args.bytes, args.chars, args.lines, args.words]
        .iter()
        .all(|v| v == &false)
    {
        args.bytes = true;
        args.lines = true;
        args.words = true;
    }

    Args {
        files: args.files,
        bytes: args.bytes,
        chars: args.chars,
        lines: args.lines,
        words: args.words,
    }
}

pub fn format_field(value: impl Display, show: bool) -> String {
    const WIDTH: usize = 8;

    if show {
        format!("{value:>WIDTH$}")
    } else {
        "".to_string()
    }
}

pub fn run() -> MyResult<()> {
    let args = get_args();
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut num_lines = 0;
    let mut num_words = 0;
    let files_len = args.files.len();

    for filename in args.files {
        match open(&filename) {
            Ok(reader) => {
                let show_filename = filename != "-";
                let file_info = count(reader)?;

                println!(
                    "{}{}{}{}{}",
                    format_field(file_info.num_lines, args.lines),
                    format_field(file_info.num_words, args.words),
                    format_field(file_info.num_bytes, args.bytes),
                    format_field(file_info.num_chars, args.chars),
                    format_field(format!(" {filename}"), show_filename),
                );

                num_lines += file_info.num_lines;
                num_words += file_info.num_words;
                num_bytes += file_info.num_bytes;
                num_chars += file_info.num_chars;
            }
            Err(err) => eprintln!("{filename}: {err}"),
        }
    }

    if files_len > 1 {
        println!(
            "{}{}{}{} total",
            format_field(num_lines, args.lines),
            format_field(num_words, args.words),
            format_field(num_bytes, args.bytes),
            format_field(num_chars, args.chars),
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(4, true), "       4");
        assert_eq!(format_field(300, false), "");
        assert_eq!(format_field("sample text", true), "sample text");
        assert_eq!(format_field("sample text", false), "");
    }
}
