use clap::Parser;

/// Write arguments to the standard output.
///
/// The echor utility writes any specified operands,
/// separated by single blank (' ') characters and
/// followed by a new line ('\n') chacter, to the
/// standard output.
#[derive(Parser, Debug)]
#[command(author = "test", version)]
struct Echor {
    /// Text to write to the standard output.
    #[arg(name = "TEXT", required = true)]
    text: Option<Vec<String>>,

    /// Do not print the trailing newline character.
    ///
    /// This may also be achieved by appending '\c' to the
    /// end of the string, as  is done by iBCS2 compatible
    /// systems. Note that this option as well as the effect
    /// of '\c' are implementation-defined in IEEE Std 1003.1-2001
    /// ("POSIX.1") as amended by cor. 1-2022. Applications aiming
    /// for maximum portabiliity are strongly encouraged to use
    /// printf(1) to suppres the newline chacter.
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let args = Echor::parse();

    let text = args.text.unwrap_or(vec![]).join(" ");
    let newline = if args.omit_newline { "" } else { "\n" };

    print!("{text}{newline}");
}
