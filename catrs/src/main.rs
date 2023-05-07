use std::process;

use catrs::run;

fn main() {
    if let Err(err) = run() {
        // eprintln!("Exit with error: \"{err}\"");
        eprintln!("{err}");
        process::exit(1);
    }
}
