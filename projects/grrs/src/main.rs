use std::{fs::File, io::BufRead, io::BufReader};

use clap::Parser;

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
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let line_str = line.expect("Error in line.");
        if line_str.contains(&args.pattern) {
            println!("{}", line_str);
        }
    }
}
