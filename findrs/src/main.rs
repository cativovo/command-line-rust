use findrs::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
    }
}
