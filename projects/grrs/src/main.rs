use clap::Parser;
use std::{fs::File, io::BufReader};

#[derive(Parser)]
struct Cli {
    /// Pattern to search for.
    pattern: String,
    /// File to search within.
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let file = File::open(&args.path).expect("Could not read file.");
    let reader = BufReader::new(file);
    let pattern_matches = grrs::find_matches(reader, &args.pattern);

    for line in pattern_matches.iter() {
        println!("{}", line);
    }
}
