use std::convert::From;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

fn main() {
    println!("Rust by Example!");
    let line_break = "–––––––––––––––––––––––––––";
    println!("{}", line_break);
    ch6_conversion();
    println!("{}", line_break);
    ch8_flow_of_control();
    println!("{}", line_break);
    ch9_functions();
    println!("{}", line_break);
}

// Chapter 6
fn ch6_conversion() {
    println!("Chapter 6. Conversion");

    // `From` trait.
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

    let num = Number::from(30);
    println!("My number is {:?}", num);

    // `Into` trait.
    let int = 5;
    let num: Number = int.into();
    println!("My other number is {:?}", num);

    // `TryFrom` trait.
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
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // `TryInto` trait.
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // Converting to String by implementing the `ToString` trait through the `Display` trait.
    struct Circle {
        radius: i32,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circule of radius{}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Paring a String.
    let parsed: i32 = "5".parse().unwrap();
    println!("Parsed string: {}", parsed);
    let turbo_parsed = "10".parse::<i32>().unwrap(); // "turbofish" syntax 😜
    println!("Turbofish parsed string: {}", turbo_parsed);
}

// Chapter 8
fn ch8_flow_of_control() {
    println!("Chapter 8. Flow of Control");

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

    // guards
    #[allow(dead_code)]
    enum Temperature {
        C(i32),
        F(i32),
    }
    let temperature = Temperature::C(35);
    match temperature {
        Temperature::C(t) if t > 30 => println!("Temperature is above 30 C"),
        Temperature::C(t) => println!("{} C is below 30 C", t),
        Temperature::F(t) if t > 86 => println!("Temperature is above 86 F"),
        Temperature::F(t) => println!("{} F is above 86 F", t),
    }

    // binding
    fn age() -> u32 {
        return 15;
    }
    match age() {
        0 => println!("No birthdays yet"),
        n @ 1..=12 => println!("Child of age {}", n),
        n @ 13..=19 => println!("Teen of age {}", n),
        n => println!("Adult of age {}", n),
    }

    // if-let
    let number = Some(7);
    if let Some(i) = number {
        println!("Number is {:?}", i)
    } else {
        println!("Number is None")
    }

    // let-else
    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' '); // split the string by ' '
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Cannot segment count item pair: '{s}'");
        };
        let Ok(count) = u64::from_str(count_str) else {
            panic!("Cannot parse integer: '{count_str}'");
        };
        (count, item)
    }
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}

fn ch9_functions() {
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }
    fn fizzbuzz(n: u32) {
        if is_divisible_by(n, 15) {
            print!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            print!("fizz");
        } else if is_divisible_by(n, 5) {
            print!("buzz");
        } else {
            print!("{}", n);
        }
    }
    fn fizzbuzz_to(n: u32) {
        for n in 1..=n {
            fizzbuzz(n);
            print!(" ");
        }
        println!();
    }
    fizzbuzz_to(16);

    // Closures
    let outer_var = 42;
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    println!("closure_annotated: {}", closure_annotated(1));

    let one = || 1;
    println!("closure returing one: {}", one());

    let color = String::from("green");
    let print = || println!("`color`: {}", color);
    print();
    let _reborrow = &color;
    print();
    let _color_moved = color;
    // print(); // can no longer use `color` so the `print()` closure breaks

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    // let _reborrow = &count; // cannot borrow because there is already a mut ref in `inc()`
    inc();

    use std::mem;
    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume(); // `moveable` is consumed on the first call so cannot be called again

    let haystack = vec![1, 2, 3]; // `Vec` has non-copy semantics
    let contains = move |needle| haystack.contains(needle);
    println!("haystack contains &1: {}", contains(&1));
    println!("haystack contains &4: {}", contains(&4));
    // The following is not allowed because ownership of `haystack` is in `contains()`.
    // println!("there are {} elements in `haystack", haystack.len());

    // Higher order functions (HOF)
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    // imperative style:
    let upper = 1000;
    let mut total = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n) {
            total += n_squared
        }
    }
    println!("imperative style: {}", total);

    // functional style (using functions as arguments over collections):
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();
    println!("functional style: {}", sum_of_squared_odd_numbers);

    // Diverging functions
    #[allow(dead_code)]
    fn foo() -> ! {
        panic!("This call never returns 🙃")
    }
}
