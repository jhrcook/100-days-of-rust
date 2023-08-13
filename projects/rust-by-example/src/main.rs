use std::convert::From;
use std::convert::TryFrom;
use std::fmt;

fn main() {
    println!("Rust by Example!");
    ch6_conversion();
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circule of radius{}", self.radius)
    }
}

fn ch6_conversion() {
    println!("Chapter 6. Conversion");

    // `From` trait.
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // `Into` trait.
    let int = 5;
    let num: Number = int.into();
    println!("My other number is {:?}", num);

    // `TryFrom` trait.
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // `TryInto` trait.
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // Converting to String by implementing the `ToString` trait through the `Display` trait.
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Paring a String.
    let parsed: i32 = "5".parse().unwrap();
    println!("Parsed string: {}", parsed);
    let turbo_parsed = "10".parse::<i32>().unwrap(); // "turbofish" syntax 😜
    println!("Turbofish parsed string: {}", turbo_parsed);
}
