use std::error::Error;

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

/// Findrs something something
#[derive(Parser, Debug)]
#[command(author = "Mark Leo Cativo", version, about, long_about = None)]
struct Args {
    /// directory
    #[arg(name = "PATH")]
    path: String,
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    println!("{args:#?}");

    Ok(())
}
