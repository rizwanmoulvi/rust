fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of rectangle is {} square pixels",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of rectangle is {} square pixels.",
        area1(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of rectangle is {} square pixels",
        area2(&rect2)
    );

    let rect3 = Rect {
        width: 30,
        height: 50,
    };

    println!("\nrect3 can be represented as {:?}", rect3);
    println!("\nrect3 can be represented as {:#?}\n", rect3);
    
    let scale = 2;
    let rect4 = Rect {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect4);

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Refactoring with Tuples
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//Refactoring with Structs

struct Rectangle {
    width: u32,
    height: u32,
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Adding useful functionality with derived traits
#[derive(Debug)]
struct Rect{
    width: u32,
    height: u32,
}