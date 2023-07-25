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
        self.width > 0
    }
}

// Method that takes additional arguments.
impl Rectangle {
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        other_rect.height < self.height && other_rect.width < self.width
    }
}

// Associated function that is not a method.
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
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

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("My new square: {:?}", sq)
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
