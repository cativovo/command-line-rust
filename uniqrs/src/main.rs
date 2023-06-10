use std::process;

use uniqrs::run;

fn main() {
    if let Err(err) = run() {
        eprint!("{err}");
        process::exit(1);
    }
}
