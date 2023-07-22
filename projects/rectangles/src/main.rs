#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// `area()` method for the Rectangle struct.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width() > 0
    }
}

fn main() {
    let scale = 2;
    let r1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("Area of rectangle: {rect_area}", rect_area = area(&r1));

    println!("`r1` is {:?}", r1);

    dbg!(&r1);

    if r1.width() {
        println!(
            "Width of the `r1` is greater than 0; it is {w}.",
            w = r1.width
        );
    }

    println!("Area of rectangle: {rect_area}", rect_area = r1.area());
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
