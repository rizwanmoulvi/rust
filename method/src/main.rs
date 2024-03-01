#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self, scale: u32) -> u32 {
        self.width * scale + self.height * scale
    }

    fn change(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // Constructor 
    fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 15,
    };

    let rect3 = Rectangle::new(25, 20);
    
    println!(
        "The area of rect3 is {}",
        rect3.area()
    );

    println!(
        "The area of rectangle is {} square pixels",
        rect1.area()
    );

    println!(
        "The perimeter of rectangle is {} pixels",
        rect1.perimeter(2)
    );

    rect1.change(10,10);

    println!(
        "The perimeter of rectangle is {} pixels",
        rect1.perimeter(2)
    );

    if rect1.width() {
        println!("Width is non zero value, it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}