use std::convert::From;
use std::convert::TryFrom;
use std::fmt;

fn main() {
    println!("Rust by Example!");
    let line_break = "–––––––––––––––––––––––––––";
    println!("{}", line_break);
    ch6_conversion();
    println!("{}", line_break);
    ch7_flow_of_control();
    println!("{}", line_break);
}

// Chapter 6

#[allow(dead_code)]
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

// Chapter 7
fn ch7_flow_of_control() {
    println!("Chapter 7. Flow of Control");

    // if-else
    let n = 5;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is 0", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase by 10-fold");
        n * 10
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);

    // for loops
    for n in 1..11 {
        print!("{} ", n)
    }
    println!();

    // for and iterators
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        // `name` is a `&&str`
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => (),
        }
    }
    println!("names: {:?}", names);

    for name in names.into_iter() {
        // `name` is a `&str`
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => (),
        }
    }
    // println!("names: {:?}", names);  / cannot execute if not commented out

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    // match
    let number = 13;
    match number {
        1 => println!("One"),
        2 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't nuttin' special"),
    }

    let triple = (0, -2, 3);
    match triple {
        (0, y, z) => println!("(0, y: {}, z: {}", y, z),
        (1, ..) => println!("(1, ..., ...)"),
        (.., 2) => println!("(..., 2, ...)"),
        _ => println!("All else"),
    }

    let array = [1, -2, 6];
    match array {
        [0, second, third] => println!("array[0]: 0, array[1]: {}, array[2]: {}", second, third),
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        [first, middle @ .., last] => {
            println!("{} and {} with {:?} in between", first, last, middle)
        }
    }

    #[allow(dead_code)]
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        CYMK(u32, u32, u32, u32),
    }

    let color = Color::Red;
    match color {
        Color::Red | Color::Blue | Color::Green => println!("primary color"),
        Color::RGB(a, b, c) => println!("RGB color: ({},{},{})", a, b, c),
        Color::CYMK(a, b, c, d) => println!("CYMK color: ({},{},{},{})", a, b, c, d),
    }

    // paused: https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_pointers.html
}
