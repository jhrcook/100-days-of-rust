fn main() {
    let s = String::from("Hello world");
    let first_word = first_word(&s);
    println!("First word of '{s}' is '{first_word}'.");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // Return index of the first space.
        }
    }
    &s[..] // No spaces found, return full length.
}
