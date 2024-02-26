fn main() {
    greet();

    let a = add(6,3);
    println!("{a}");

    let b = sub(9.3, 5.3);
    println!("{b}");

    let x = product(4,5);
    println!("{x}");
}

fn greet() {
    println!("Hello, Welocme to function print");
}

fn add(x: i32, y: i32) -> i32 {
    x+y
}

fn sub(x: f32, y: f32) -> f32 { 
    let a = add(x as i32, y as i32) as f32;
    println!("Value of a is {a}");
    // let a = a;
    return x-a
}

fn product(x: i32, y: i32) -> i32 {
    let a = x;
    let b = y;
    a*b
}