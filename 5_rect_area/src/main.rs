// This enables automatic printing of Rectangle structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(dim: u32) -> Rectangle {
        Rectangle { width: dim, height: dim }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Its fields are {:#?}", &rect);

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect.can_hold(&rect3));

    let square = Rectangle::square(20);

    println!("This square has fields: {:#?}", &square);
}