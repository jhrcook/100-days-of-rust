enum IpAdderKind {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    /// Experimenting with my custom enum.
    let home = IpAdderKind::V4(String::from("127.0.0.1"));
    let loopback = IpAdderKind::V6(String::from("::1"));

    // `Option` enum.
    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    // println!("x == y?: {}", x == y);  // DOES NOT WORK!

    // Coins in a match statement.
    println!("Value of a penny: {}", value_in_cents(&Coin::Penny));

    // Matching on Option<T>.
    // let x_plus1 = plus_one(x);
    let y_plus1 = plus_one(y);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
