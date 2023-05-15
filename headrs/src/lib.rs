use std::{
    error::Error,
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

/// Rust head
#[derive(Parser, Debug)]
#[command(author = "Mark Leo Cativo", version, long_about = None)]
struct Headrs {
    /// Input file(s)
    #[arg(name = "FILES", default_value = "-")]
    files: Vec<String>,

    /// Print count lines of each of the specified file(s).
    #[arg(short = 'n', long , value_parser = parse_positive_int)]
    #[arg(value_name = "NUM", default_value = "10")]
    lines: usize,

    /// Print bytes of each of the specified file(s).
    #[arg(short = 'c', long, value_parser = parse_positive_int)]
    #[arg(value_name = "NUM", conflicts_with("lines"))]
    bytes: Option<usize>,
}

fn parse_positive_int(val: &str) -> Result<usize, String> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.to_string()),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() -> MyResult<()> {
    let headrs = Headrs::parse();
    let files_len = headrs.files.len();

    for (file_num, filename) in headrs.files.iter().enumerate() {
        match open(filename) {
            Ok(mut reader) => {
                if files_len > 1 {
                    let new_line = if file_num > 0 { "\n" } else { "" };

                    println!("{new_line}==> {filename} <==");
                }

                if let Some(bytes) = headrs.bytes {
                    let mut handle = reader.take(bytes as u64);
                    let mut buf: Vec<u8> = vec![0; bytes];
                    let bytes_read = handle.read(&mut buf)?;
                    let result = String::from_utf8_lossy(&buf[..bytes_read]);

                    print!("{result}");
                } else {
                    let mut buf = String::new();

                    for _ in 0..headrs.lines {
                        let bytes = reader.read_line(&mut buf)?;

                        if bytes == 0 {
                            break;
                        }

                        print!("{buf}");
                        buf.clear();
                    }
                }
            }
            Err(err) => eprintln!("{filename}: {err}"),
        }
    }

    Ok(())
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let result = parse_positive_int("3");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3);

    // Any string is an error
    let result = parse_positive_int("foo");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let result = parse_positive_int("0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "0".to_string());
}
