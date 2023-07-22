// Example of a custom struct.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Example of a tuple struct.
struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Johnny Appleseed"),
        email: String::from("john@apple.com"),
        sign_in_count: 1,
    };
    let username = user1.username;
    println!("User: {username}");
    user1.username = String::from("Jane Appleseed");
    println!("User: {username}");

    let user2 = build_user(String::from("my-email@yahoo.com"), String::from("dave"));

    // Using "struct update syntax."
    let user3 = User {
        email: String::from("new-email@internet.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    // Example with "field init shorthand" syntax.
    User {
        active: true,
        username, // Variable with same name as field.
        email,
        sign_in_count: 1,
    }
}
