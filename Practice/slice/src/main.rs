fn main() {
    let s1  = String::from("Hello World");
    let s2  = &s1[6..11];
    println!("{s2}");
}