use std::io;

fn main() {
    println!("Enter Your Name");
    let mut s1 = String::new();
    io::stdin()
        .read_line(&mut s1)
        .expect("Failed to take input");
    let mut s1: String = s1.trim().parse().expect("Wrong String");
    // let s1: String = s1.trim().parse().expect("Wrong String");
    let len = calculate_len(&mut s1);
    println!("The length of '{}' is {} ",s1, len);

    // let s2 = s1; // Ownership transfer
    // println!("{s1}"); //results in error

    let s2 = &s1; // Referencing
    println!("{s1}");

    // let s3 = &mut s1;
    // let s4 = &mut s1; // compiles but not work when used

    // println!("{}, {}", s3, s4);

    let s3 = &mut s1;
    println!("{s3}");

    let s4 = &mut s1;
    println!("{s4}");

    let s5 = &mut s1;

    {
        let s6 = &mut s1;
    }

    // Will not work due to write and read anomaly
    // let s7 = &s1;
    // let s8 = &s1;
    // let s9 = &mut s1;
    // println!("{s7} {s8} {s9}");

    let s7 = &s1;
    let s8 = &s1;
    println!("{s7} {s8}");
    let s9 = &mut s1;
    println!("{s9}");

    // Trying Dangling Reference
    // let reference_to_nothing = dangle();
}

//Here refrencing is used instead of move (ownership)
fn calculate_len(a_string: &mut String) -> usize { 
    let len = a_string.len();
    a_string.push_str(" is your name!");
    len
}

// fn calculate_len(a_string: &String) -> usize { 
//     let len = a_string.len();
//     len
// }

// Will not work 
// fn dangle() -> &String {
//     let s = String::from("Hello");

//     &s
// }