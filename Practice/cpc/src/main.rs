use std::io;

fn main() {
    println!("Practice of common programming practices\n");
    println!("Array");

    println!("Enter index a number");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to take input");

    let arr = [10, 20, 30, 40, 50];

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered is not a number");

    let element = arr[index];

    println!("The element at index {index} is {element}");

    println!("Enter a element value");
    let mut user_element = String::new();

    io::stdin()
        .read_line(&mut user_element)
        .expect("Failed to take input");

    let user_element: usize = user_element
        .trim()
        .parse()
        .expect("Wrong Element");

    let post = arr
        .iter()
        .position(|&r| r == user_element);

    match post {
        Some(ind) => println!("Element found at index {}",ind),
        None => println!("Element not present array"),
    }

    let tup = (5,4,3,2,1);
    let tup1 = tup.2;
    println!("{tup1}");

    println!("\nSimple Calculator");
}