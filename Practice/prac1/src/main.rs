use std::io;

fn main() {
    println!("Learn to take input");

    // println!("Enter a number:");

    // let mut x = String::new();
    // let mut y = String::new();

    // io::stdin().read_line(&mut x).expect("Failed to take input");
    // io::stdin().read_line(&mut y).expect("Failed to take input");

    // let x: i32 = x.trim().parse().expect("Only number is allowed");
    // let y: i32 = y.trim().parse().expect("Only number is allowed");

    // let sum = x + y;

    // println!("The number you entered is: {}", sum);
    loop {
        println!("Please enter your name");
    }
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Failed to take input");
    
    let name: String = name.trim().parse().expect("Trimmed name");

    println!("Hello {name}. How are you");
}