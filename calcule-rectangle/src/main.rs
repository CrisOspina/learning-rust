#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.", 
        rect.area()
    );

    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    let sq = Rectangle::square(3);
    println!("sq is {:?}", sq);
}
