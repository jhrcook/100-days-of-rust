use clap::Parser;
use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Parser)]
struct Cli {
    /// Pattern to search for.
    pattern: String,
    /// File to search within.
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let pattern_matches = find_matches(&args.path, &args.pattern);
    for line in pattern_matches.iter() {
        println!("{}", line);
    }
}

fn find_matches(path: &std::path::Path, pattern: &String) -> Vec<String> {
    let file = File::open(path).expect("Could not read file.");
    let reader = BufReader::new(&file);
    reader
        .lines()
        .flat_map(|x| x)
        .filter(|x| x.contains(pattern))
        .collect()
}

// #[test]
// fn test_find_a_match() {
//     let buf = BufReader::new("lorem ipsum\ndolor sit amet".as_bytes());
//     let res = find_matches(buf, &String::from("lorem"));
//     // &PathBuf::from("lorem ipsum\ndolor sit amet"),
//     assert_eq!(res, vec!["lorem ipsum\n"]);
// }
