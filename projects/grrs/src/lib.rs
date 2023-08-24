use std::io::BufRead;

pub fn find_matches<T: BufRead>(reader: T, pattern: &String) -> Vec<String> {
    reader
        .lines()
        .flat_map(|x| x)
        .filter(|x| x.contains(pattern))
        .collect()
}

#[test]
fn test_find_a_match() {
    let buf = std::io::BufReader::new("lorem ipsum\ndolor sit amet".as_bytes());
    let res = find_matches(buf, &String::from("lorem"));
    // &PathBuf::from("lorem ipsum\ndolor sit amet"),
    assert_eq!(res, vec!["lorem ipsum"]);
}
