fn main() {
    let s = String::from("Hello World");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}");
    println!("{world}");
    
    for i in s.chars() {
        println!("{i}");
    }
}

